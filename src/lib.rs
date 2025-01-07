// Using the lib.rs file as the place to configure the crate's public API.

/// Contains all models and function calls associated with the Perspective API.
pub mod client;

/// Contains a request builder and some convenience functions for accessing the API, returning anyhow errors.
#[cfg(feature = "service")]
pub mod service;
