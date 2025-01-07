use serde::{Deserialize, Serialize};

/// Raw success structure from the API. It is unclear which fields always exist, so they are all optional.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeCommentResponse {
    attribute_scores: Option<AttributeScores>,
    languages: Option<Vec<String>>,
    detected_languages: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AttributeScores {
    toxicity: Option<AttributeScore>,
    identity_attack: Option<AttributeScore>,
    sexually_explicit: Option<AttributeScore>,
    severe_toxicity: Option<AttributeScore>,
    profanity: Option<AttributeScore>,
    threat: Option<AttributeScore>,
    flirtation: Option<AttributeScore>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AttributeScore {
    span_scores: Option<Vec<SpanScore>>,
    summary_score: Option<Score>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpanScore {
    begin: Option<i64>,
    end: Option<i64>,
    score: Option<Score>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Score {
    value: Option<f64>,
    #[serde(rename = "type")]
    score_type: Option<Type>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    #[serde(rename = "PROBABILITY")]
    Probability,
}
