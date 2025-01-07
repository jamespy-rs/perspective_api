use anyhow::bail;
use tracing::info;

use crate::client::{AnalyzeCommentError, AnalyzeCommentRequest, AnalyzeCommentResponse};
use crate::service::request_builder::AnalyzeCommentRequestBuilder;
use crate::service::PerspectiveService;

impl AnalyzeCommentRequest {
    pub fn builder() -> AnalyzeCommentRequestBuilder {
        AnalyzeCommentRequestBuilder::new()
    }
}

impl PerspectiveService {
    pub async fn analyze_comment(&self, text: &str) -> anyhow::Result<AnalyzeCommentResponse> {
        info!("Analyzing the following comment: {}", text);

        let mut request_builder = AnalyzeCommentRequestBuilder::default().with_text(text);

        match self.perspective_client.analyze(request_builder.build()).await {
            Err(res) => {
                match res {
                    AnalyzeCommentError::RequestError(ref e) => {
                        let request = &request_builder.build();
                        /* log, retry */
                        info!("HTTP error. Retrying. Error: {}. Request: {:?}.", e, request);
                        bail!("{res}")
                    }
                    AnalyzeCommentError::TextTooLong => {
                        /* log, truncate, retry */
                        bail!("{res}")
                    }
                    AnalyzeCommentError::LanguageUnsupportedByAttribute { .. } => {
                        /* log, remove the attribute and retry */
                        bail!("{res}")
                    }
                    AnalyzeCommentError::ApiKeyInvalid | AnalyzeCommentError::NoApiKey => {
                        /* log, bail */
                        info!("{res}");
                        bail!("{res}")
                    }
                    AnalyzeCommentError::Unknown(ref perspective_error) => {
                        /* log, bail */
                        bail!("{perspective_error:?}\n{res}")
                    }
                    AnalyzeCommentError::DeserializationError(ref e) => {
                        /* log, bail */
                        bail!("{e}\n{res}")
                    }
                    AnalyzeCommentError::TextEmpty => {
                        /* log, bail */
                        bail!("{res}")
                    }
                }
            }
            Ok(res) => Ok(res),
        }
    }
}
