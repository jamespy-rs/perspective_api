use anyhow::Result;
use perspective_api::client::{AnalyzeCommentRequest, AnalyzeCommentResponse, PerspectiveClient};
use std::error::Error;

// These tests highlight if Perspective change their API by calling it directly and comparing to the last known response.
const PERSPECTIVE_API_KEY: &str = include_str!("../resources/key");

#[tokio::test]
async fn test_success() -> Result<(), Box<dyn Error>> {
    let body = include_str!("../src/client/v1alpha1/models/examples/request.json");
    let request: AnalyzeCommentRequest = serde_json::from_str(body)?;
    let body = include_str!("../src/client/v1alpha1/models/examples/response.json");
    let response: AnalyzeCommentResponse = serde_json::from_str(body)?;

    assert_eq!(response, PerspectiveClient::new(PERSPECTIVE_API_KEY).analyze(request).await.unwrap());
    Ok(())
}
