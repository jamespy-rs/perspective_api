pub(crate) mod analyze;
pub(crate) mod models;

// client constants
pub(crate) const BASE_URL: &str = "https://commentanalyzer.googleapis.com/v1alpha1/comments:analyze";

use reqwest::Client;

/// A wrapper for a `reqwest` client and your API key, and has the API entry points.
pub struct PerspectiveClient {
    pub client: Client,
    pub api_key: String,
}

impl PerspectiveClient {
    pub fn new(api_key: &str) -> Self {
        Self { client: Client::new(), api_key: api_key.to_owned() }
    }
}
