use crate::client::PerspectiveClient;

mod analyze_comment;
mod request_builder;

pub use request_builder::AnalyzeCommentRequestBuilder;
pub use request_builder::Attribute;

/// A wrapper for the `PerspectiveClient`, which is extended with convenience functions.
pub struct PerspectiveService {
    perspective_client: PerspectiveClient,
}

impl PerspectiveService {
    pub fn new(api_key: &str) -> Self {
        Self { perspective_client: PerspectiveClient::new(api_key) }
    }
}
