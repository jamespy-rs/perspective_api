use anyhow::Result;
use perspective_api::client::{AnalyzeCommentError, AnalyzeCommentRequest, PerspectiveClient};
use std::error::Error;

// These tests highlight if Perspective change their API by calling it directly and comparing to the last known response.

#[tokio::test]
async fn test_no_api_key() -> Result<(), Box<dyn Error>> {
    let body = include_str!("../src/client/v1alpha1/models/examples/request.json");
    let request: AnalyzeCommentRequest = serde_json::from_str(body)?;

    assert_eq!(AnalyzeCommentError::NoApiKey, PerspectiveClient::new("").analyze(request).await.unwrap_err());
    Ok(())
}
