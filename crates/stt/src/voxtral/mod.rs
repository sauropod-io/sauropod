pub struct Voxtral;

impl sauropod_inference_thread::InferenceProvider for Voxtral {
    type Input = Vec<f32>;
    type Output = String;

    fn process(
        &self,
        _input: &[Self::Input],
        _output: &mut Vec<anyhow::Result<Self::Output>>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
