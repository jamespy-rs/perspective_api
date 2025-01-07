use serde::{Deserialize, Serialize};

/// Raw success structure from the API. It is unclear which fields always exist,
/// so they are all optional.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeCommentResponse {
    pub attribute_scores: Option<AttributeScores>,
    pub languages: Option<Vec<String>>,
    pub detected_languages: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AttributeScores {
    pub toxicity: Option<AttributeScore>,
    pub identity_attack: Option<AttributeScore>,
    pub sexually_explicit: Option<AttributeScore>,
    pub severe_toxicity: Option<AttributeScore>,
    pub profanity: Option<AttributeScore>,
    pub threat: Option<AttributeScore>,
    pub flirtation: Option<AttributeScore>,
    /// experimental stuff.
    pub toxicity_experimental: Option<AttributeScore>,
    pub severe_toxicity_experimental: Option<AttributeScore>,
    pub identity_attack_experimental: Option<AttributeScore>,
    pub insult_experimental: Option<AttributeScore>,
    pub profanity_experimental: Option<AttributeScore>,
    pub threat_experimental: Option<AttributeScore>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AttributeScore {
    pub span_scores: Option<Vec<SpanScore>>,
    pub summary_score: Option<Score>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpanScore {
    pub begin: Option<i64>,
    pub end: Option<i64>,
    pub score: Option<Score>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Score {
    pub value: Option<f64>,
    #[serde(rename = "type")]
    pub score_type: Option<Type>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    #[serde(rename = "PROBABILITY")]
    Probability,
}
