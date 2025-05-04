//! Saurpod JSON Schema extensions.

/// Regex used to validate image data - this should be kept in sync with jsonSchemaExtensions.ts.
pub const IMAGE_REGEX: &str = "^data:image/([^;]+);base64,[A-Za-z0-9+/=]+$";

/// Regex used to validate audio data - this should be kept in sync with jsonSchemaExtensions.ts.
pub const AUDIO_REGEX: &str = "^data:audio/([^;]+);base64,[A-Za-z0-9+/=]+$";
