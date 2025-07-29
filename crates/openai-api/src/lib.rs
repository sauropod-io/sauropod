//! OpenAI-compatible API.

mod generated;
mod helpers;
mod interfaces;

#[cfg(test)]
mod generated_tests;
#[cfg(test)]
mod test_utils;
#[cfg(test)]
mod tests;

pub use generated::*;
pub use interfaces::*;
