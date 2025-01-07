use perspective_api::client::AnalyzeCommentRequest;
use perspective_api::client::PerspectiveClient;
use perspective_api::service::PerspectiveService;

#[tokio::test]
async fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let body = include_str!("../src/client/v1alpha1/models/examples/text_empty_request.json");

    let request: AnalyzeCommentRequest = serde_json::from_str(body)?;
    const PERSPECTIVE_API_KEY: &str = include_str!("../resources/key");
    match PerspectiveClient::new(PERSPECTIVE_API_KEY).analyze(request).await {
        Ok(res) => println!("analyze:\n{res:#?}"),
        Err(res) => println!("analyze:\n{res}"),
    }

    match PerspectiveService::new(PERSPECTIVE_API_KEY).analyze_comment("").await {
        Ok(res) => println!("analyze_comment:\n{res:#?}"),
        Err(res) => println!("analyze_comment:\n{res}"),
    }
    Ok(())
}
