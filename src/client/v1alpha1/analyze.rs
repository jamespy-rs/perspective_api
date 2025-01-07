use reqwest::StatusCode;
use tracing::debug;
use url::Url;

use crate::client::AnalyzeCommentError;
use crate::client::AnalyzeCommentErrorResponse;
use crate::client::AnalyzeCommentRequest;
use crate::client::AnalyzeCommentResponse;
use crate::client::PerspectiveClient;

use super::BASE_URL;
// Tests for the interaction with the API are in the `tests` directory.
impl PerspectiveClient {
    /// This function encapsulates the interaction with the Perspective API, taking a request object and returning a success response or a custom error.
    pub async fn analyze(&self, request: AnalyzeCommentRequest) -> Result<AnalyzeCommentResponse, AnalyzeCommentError> {
        let url = Url::parse_with_params(BASE_URL, [("key", &self.api_key)]).expect("no api_keys can fail this function and the base url is const");
        debug!("URL: {}", &url);

        let response = self.client.post(url).json(&request).send().await?;

        match response.status() {
            StatusCode::OK => {
                let body = &response.text().await?;
                debug!("Response: {}", &body);
                Ok(serde_json::from_str(body)?)
            }
            StatusCode::FORBIDDEN => Err(AnalyzeCommentError::NoApiKey),
            _ => {
                let body = &response.text().await?;
                debug!("Response: {}", &body);
                Err(serde_json::from_str::<AnalyzeCommentErrorResponse>(body)?)?
            }
        }
    }
}
