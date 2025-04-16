//! Sauropod's core tools.

use std::sync::Arc;

pub mod fetch;
pub mod notify;

/// Get the default list of tools.
pub fn get_default_tools() -> Vec<Arc<dyn sauropod_task_context::Tool>> {
    vec![Arc::new(fetch::FetchTool)]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_default_tools() {
        // Every tool should have a provider prefix of `builtin`.
        get_default_tools().iter().for_each(|tool| {
            assert_eq!(
                tool.get_id().split_once(':').map(|x| x.0),
                Some(sauropod_task_context::BUILTIN_PROVIDER)
            );
        });
    }
}
