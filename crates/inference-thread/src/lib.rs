//! Thread management for inference.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error putting request into the queue")]
    ErrorQueuingRequest,
    #[error("Error receiving response from inference thread")]
    ErrorReceivingResponse,
}

/// Trait for an inference worker.
pub trait InferenceProvider {
    /// The input type for the inference worker.
    type Input;
    /// The output type for the inference worker.
    type Output;

    /// Processes the input and returns the output.
    fn process(
        &self,
        input: &[Self::Input],
        output: &mut Vec<anyhow::Result<Self::Output>>,
    ) -> anyhow::Result<()>;
}

/// A queued request that holds the input and a sender for the output.
struct QueuedRequest<Input, Output> {
    /// The input to be processed.
    input: Input,
    /// The sender to return the output.
    sender: tokio::sync::oneshot::Sender<anyhow::Result<Output>>,
}

/// Batch inference thread.
pub struct BatchInferenceThread<Input, Output> {
    /// The queue for inputs to be processed.
    queue: tokio::sync::mpsc::Sender<QueuedRequest<Input, Output>>,
    /// The thread handle for the inference worker.
    _thread_handle: std::thread::JoinHandle<()>,
    /// Phantom data to hold the output type.
    _phantom: std::marker::PhantomData<Output>,
}

impl<Input, Output> BatchInferenceThread<Input, Output>
where
    Input: Send + 'static,
    Output: Send + 'static,
{
    /// Creates a new batch inference thread.
    pub fn new(
        name: String,
        batch_size: usize,
        provider: impl InferenceProvider<Input = Input, Output = Output> + Send + 'static,
    ) -> std::io::Result<Self> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(batch_size * 4);

        let thread_handle = std::thread::Builder::new().name(name).spawn(move || {
            let mut queued_inputs: Vec<QueuedRequest<Input, Output>> =
                Vec::with_capacity(batch_size);
            loop {
                let count = rx.blocking_recv_many(&mut queued_inputs, batch_size);
                if count == 0 {
                    // No more inputs, exit the loop.
                    break;
                }
                let mut inputs: Vec<Input> = Vec::with_capacity(batch_size);
                let mut response_senders: Vec<
                    tokio::sync::oneshot::Sender<anyhow::Result<Output>>,
                > = Vec::with_capacity(batch_size);
                let mut outputs = Vec::with_capacity(batch_size);
                for queued in queued_inputs.drain(..count) {
                    inputs.push(queued.input);
                    response_senders.push(queued.sender);
                }

                match provider.process(&inputs[..count], &mut outputs) {
                    Ok(()) => {
                        for (output, sender) in outputs.into_iter().zip(response_senders) {
                            let _ = sender.send(output);
                        }
                    }
                    Err(e) => {
                        for sender in response_senders {
                            let _ = sender.send(Err(anyhow::anyhow!("{}", e)));
                        }
                    }
                }
            }
            tracing::debug!("Batch inference thread exiting");
        })?;

        Ok(BatchInferenceThread {
            queue: tx,
            _thread_handle: thread_handle,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Put an input into the queue for processing.
    pub async fn enqueue(&self, input: Input) -> anyhow::Result<Output> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.queue
            .send(QueuedRequest { input, sender: tx })
            .await
            .map_err(|_| Error::ErrorQueuingRequest)?;

        rx.await.map_err(|_| Error::ErrorReceivingResponse)?
    }
}
