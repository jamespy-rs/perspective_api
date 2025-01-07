/// The Perspective client for version v1alpha1 of their API.
pub(crate) mod v1alpha1;
pub use v1alpha1::models::error::AnalyzeCommentError;
pub use v1alpha1::models::error::AnalyzeCommentErrorResponse;
pub use v1alpha1::models::error::MAX_LENGTH;
pub use v1alpha1::models::request::*;
pub use v1alpha1::models::response::*;
pub use v1alpha1::PerspectiveClient;

// This client is versioned by API. When a new version of the Perspective API is
// released, a new module should be created here.
