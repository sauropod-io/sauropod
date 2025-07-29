//! Profiling tools.

pub use tracy_client::GpuContext;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tracy client is not running")]
    NoTracyClient,
    #[error("Failed to create GPU context: {0}")]
    GpuContextCreationError(#[from] tracy_client::GpuContextCreationError),
}

#[cfg(feature = "memory-profiling")]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, 64);

#[cfg(feature = "profiling")]
pub fn get_profiling_layer() -> Option<tracing_tracy::TracyLayer> {
    Some(tracing_tracy::TracyLayer::default())
}

#[cfg(not(feature = "profiling"))]
pub fn get_profiling_layer<S>() -> Option<Box<dyn tracing_subscriber::layer::Layer<S> + Send + Sync>>
{
    None
}

/// Get the Tracy client.
pub fn get_client() -> Result<tracy_client::Client, Error> {
    tracy_client::Client::running().ok_or(Error::NoTracyClient)
}
