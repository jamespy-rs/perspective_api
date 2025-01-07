use anyhow::Result;
use perspective_api::client::{AnalyzeCommentError, AnalyzeCommentErrorResponse, AnalyzeCommentRequest, PerspectiveClient};
use pretty_assertions::assert_eq;
use std::error::Error;
// These tests highlight if Perspective change their API by calling it directly and comparing to the last known response.

#[tokio::test]
async fn test_broken_api() -> Result<(), Box<dyn Error>> {
    let body = include_str!("../src/client/v1alpha1/models/examples/request.json");
    let request: AnalyzeCommentRequest = serde_json::from_str(body)?;
    let body = include_str!("../src/client/v1alpha1/models/examples/api_error.json");
    let error: AnalyzeCommentError = serde_json::from_str::<AnalyzeCommentErrorResponse>(body).unwrap().into();

    assert_eq!(error, PerspectiveClient::new("rubbish").analyze(request).await.unwrap_err());
    Ok(())
}
