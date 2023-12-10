//! Telraam library for working with the Telraam API.

pub mod client;
pub mod endpoint;
pub mod error;
pub mod response;

/// Version of the Telraam API this library supports
pub const VER: &str = "v1";
