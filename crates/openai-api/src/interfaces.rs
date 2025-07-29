//! Interfaces for OpenAI API types.

pub trait HasId {
    /// Get the ID of the item.
    fn get_id(&self) -> Option<&str>;
}
