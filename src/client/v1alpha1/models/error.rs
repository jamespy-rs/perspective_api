use serde::Deserialize;
use thiserror::Error;

/// This is the current maximum length of an accepted comment. This may change in the future. The request builder in the service
/// handles truncating the input if that is the intended behaviour.
pub const MAX_LENGTH: usize = 20480;

/// Enumerates all the ways that the `client.analyze()` function can fail and that can be reproduced with Bruno/Postman.
#[derive(Error, Debug)]
pub enum AnalyzeCommentError {
    #[error("HTTP error")]
    RequestError(#[from] reqwest::Error),
    /// The Deserialization error will only happen when the model of the response changes in the API, which will also be caught by
    /// the integration tests.
    #[error("The model appears to have changed")]
    DeserializationError(#[from] serde_json::Error),
    #[error("The comment was too long. It exceeded 20KB or {MAX_LENGTH} characters.")]
    TextTooLong,
    #[error("The comment to analyze was empty")]
    TextEmpty,
    #[error("{message}")]
    LanguageUnsupportedByAttribute { message: String },
    #[error("The API key is invalid")]
    ApiKeyInvalid,
    #[error("No API key was used")]
    NoApiKey,
    #[error("The request failed in some as-yet unhandled way")]
    Unknown(AnalyzeCommentErrorResponse),
}

impl PartialEq for AnalyzeCommentError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl From<AnalyzeCommentErrorResponse> for AnalyzeCommentError {
    /// This is the translation layer between the data structure we get from the API and the custom error type.
    /// For testing, is is convenient to convert a .json file into this enum, and so I check the HTTP status code here too.
    /// I can also check it in the `.analyze()` function directly but have chosen not to at this time.
    fn from(value: AnalyzeCommentErrorResponse) -> Self {
        let message: Option<&str> = (|| value.as_ref().error.as_ref()?.message.as_ref())().map(|x| x.as_str());
        let code: Option<u16> = (|| value.as_ref().error.as_ref()?.code)().or(None);
        let error_type: Option<&str> = (|| value.as_ref().error.as_ref()?.details.as_ref()?.first()?.error_type.as_ref())().map(|x| x.as_str());
        let reason: Option<&str> = (|| value.as_ref().error.as_ref()?.details.as_ref()?.first()?.reason.as_ref())().map(|x| x.as_str());

        // every error message so far has a "message" field. Don't need to check this first, but definitely want to highlight the issue early.
        if code == Some(403) {
            AnalyzeCommentError::NoApiKey
        } else if let Some(message) = message {
            if message.contains("too many bytes") {
                AnalyzeCommentError::TextTooLong
            } else if reason == Some("API_KEY_INVALID") {
                AnalyzeCommentError::ApiKeyInvalid
            } else if error_type == Some("COMMENT_EMPTY") {
                AnalyzeCommentError::TextEmpty
            } else if error_type == Some("LANGUAGE_NOT_SUPPORTED_BY_ATTRIBUTE") {
                AnalyzeCommentError::LanguageUnsupportedByAttribute { message: message.to_owned() }
            } else {
                AnalyzeCommentError::Unknown(value)
            }
        } else {
            AnalyzeCommentError::Unknown(value)
        }
    }
}

/// This is the literal representation of the error response from the API. I am not modelling every field here. Only enough to
/// translate into the custom error type above, which is why this does not implement Error.
#[derive(Debug, Clone, Deserialize)]
pub struct AnalyzeCommentErrorResponse {
    error: Option<Error>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Error {
    code: Option<u16>,
    message: Option<String>,
    details: Option<Vec<Detail>>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    reason: Option<String>,
    error_type: Option<String>,
}
impl AsRef<AnalyzeCommentErrorResponse> for AnalyzeCommentErrorResponse {
    fn as_ref(&self) -> &AnalyzeCommentErrorResponse {
        self
    }
}
