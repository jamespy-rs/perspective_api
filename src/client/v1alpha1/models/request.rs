use serde::{Deserialize, Serialize};

/// This models the entire scope of request options for the API, and can be
/// constructed more easily with the request builder in the service.
/// Documentation for the fields is copied from [the website](https://developers.perspectiveapi.com/s/about-the-api-methods?language=en_US).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeCommentRequest {
    /// (required) The text to score. This is assumed to be utf8 raw text of the
    /// text to be checked. Emoji and other non-ascii characters can be included
    /// (HTML will probably result in lower performance).
    pub comment: Comment,
    /// (optional) A list of objects providing the context for comment. The API
    /// currently does not make use of this field, but it may influence API
    /// responses in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    /// (required) A map from attribute name to a configuration object. See the
    /// ‘Attributes and Languages’ page for a list of available attribute names.
    /// If no configuration options are specified, defaults are used, so the
    /// empty object {} is a valid (and common) choice. You can specify multiple
    /// attribute names here to get scores from multiple attributes in a single
    /// request.
    pub requested_attributes: RequestedAttributes,
    /// (optional) A list of ISO 631-1 two-letter language codes specifying the
    /// language(s) that comment is in (for example, "en", "es", "fr", "de",
    /// etc). If unspecified, the API will auto-detect the comment language. If
    /// language detection fails, the API returns an error. Note: See currently
    /// supported languages on the ‘Attributes and Languages’ page. There is no
    /// simple way to use the API across languages with production support and
    /// languages with experimental support only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// (optional) A boolean value that indicates if the request should return
    /// spans that describe the scores for each part of the text (currently done
    /// at per-sentence level). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_annotations: Option<bool>,
    /// (optional) Whether the API is permitted to store comment and context
    /// from this request. Stored comments will be used for future research and
    /// community attribute building purposes to improve the API over time.
    /// Defaults to false (request data may be stored). Warning: This should be
    /// set to true if data being submitted is private (i.e. not publicly
    /// accessible), or if the data submitted contains content written by
    /// someone under 13 years old (or the relevant age determined by applicable
    /// law in my jurisdiction).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_store: Option<bool>,
    /// (optional) An opaque token that is echoed back in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// (optional) An opaque session ID. This should be set for authorship
    /// experiences by the client side so that groups of requests can be grouped
    /// together into a session. This should not be used for any user-specific
    /// id. This is intended for abuse protection and individual sessions of
    /// interaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// (optional) An opaque identifier associating this comment with a
    /// particular community within your platform. If set, this field allows us
    /// to differentiate comments from different communities, as each community
    /// may have different norms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub text: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_type: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Context {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<Comment>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RequestedAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severe_toxicity: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_attack: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexually_explicit: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flirtation: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insult: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity_experimental: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severe_toxicity_experimental: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_attack_experimental: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insult_experimental: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity_experimental: Option<ScoreOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_experimental: Option<ScoreOptions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreOptions {
    /// (optional) The score type returned for this attribute. Currently, only
    /// "PROBABILITY" is supported. Probability scores are in the range [0,1].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_type: Option<String>,
    /// (optional) The API won't return scores that are below this threshold for
    /// this attribute. By default, all scores are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
}

#[cfg(feature = "service")]
impl ScoreOptions {
    pub(crate) fn new() -> Self {
        ScoreOptions {
            score_type: None,
            score_threshold: None,
        }
    }
    pub(crate) fn with_limit(mut self, threshold: f64) -> Self {
        self.score_threshold = Some(threshold);
        self
    }
}

impl Default for AnalyzeCommentRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzeCommentRequest {
    pub fn new() -> Self {
        AnalyzeCommentRequest {
            comment: Comment {
                text: "".to_owned(),
                comment_type: None,
            },
            context: None,
            requested_attributes: RequestedAttributes {
                toxicity: None,
                severe_toxicity: None,
                identity_attack: None,
                insult: None,
                profanity: None,
                threat: None,
            },
            languages: None,
            span_annotations: None,
            do_not_store: None,
            client_token: None,
            session_id: None,
            community_id: None,
        }
    }
}
