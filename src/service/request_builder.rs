use crate::client::v1alpha1::models::request::ScoreOptions;
use crate::client::AnalyzeCommentRequest;
use crate::client::MAX_LENGTH;
use tracing::info;

/// A more concise way to construct a request, using the builder pattern
pub struct AnalyzeCommentRequestBuilder {
    // values in the request directly.
    text: String,
    requested_attributes: Vec<Attribute>,
    do_not_store: bool,
    // switches modifying the content
    truncate: bool,
    session_id: String,
    community_id: String,
    client_token: String,
    span_annotations: bool,
}

// This is the standard Arwen configuration.
impl Default for AnalyzeCommentRequestBuilder {
    fn default() -> Self {
        AnalyzeCommentRequestBuilder::new()
            .with_attribute(Attribute::Toxicity(None))
            .do_not_store()
    }
}

/// All the production-ready attributes which can be requested for every language.
pub enum Attribute {
    /// A rude, disrespectful, or unreasonable comment that is likely to make people leave a discussion.
    Toxicity(Option<f64>),
    /// A very hateful, aggressive, disrespectful comment or otherwise very likely to make a user leave a discussion or give up on sharing their perspective. This attribute is much less sensitive to more mild forms of toxicity, such as comments that include positive uses of curse words.
    SevereToxicity(Option<f64>),
    /// Negative or hateful comments targeting someone because of their identity.
    IdentityAttack(Option<f64>),
    /// Insulting, inflammatory, or negative comment towards a person or a group of people.
    Insult(Option<f64>),
    /// Swear words, curse words, or other obscene or profane language.
    Profanity(Option<f64>),
    /// Describes an intention to inflict pain, injury, or violence against an individual or group.
    Threat(Option<f64>),
}

impl AnalyzeCommentRequestBuilder {
    pub fn new() -> Self {
        AnalyzeCommentRequestBuilder {
            text: "".to_string(),
            requested_attributes: Vec::new(),
            do_not_store: false,
            truncate: false,
            session_id: "".to_owned(),
            community_id: "".to_owned(),
            client_token: "".to_owned(),
            span_annotations: false,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Logic to produce the JSON from the config in the builder
    pub fn build(&mut self) -> AnalyzeCommentRequest {
        let mut req: AnalyzeCommentRequest = AnalyzeCommentRequest::new();

        if self.truncate && self.text.len() >= MAX_LENGTH {
            let closest_char_boundary;
            // can all be replaced with self.text.floor_char_boundary() when stabilised.
            // This is to avoid the panic from `truncate` when called on a non-char-boundary index.
            if self.text.is_char_boundary(MAX_LENGTH) {
                closest_char_boundary = MAX_LENGTH
            } else if self.text.is_char_boundary(MAX_LENGTH - 1) {
                closest_char_boundary = MAX_LENGTH - 1;
            } else if self.text.is_char_boundary(MAX_LENGTH - 2) {
                closest_char_boundary = MAX_LENGTH - 2;
            } else {
                closest_char_boundary = MAX_LENGTH - 3;
            }
            info!("Truncating comment from {} to {closest_char_boundary}", self.text.len());
            self.text.truncate(closest_char_boundary);
        }

        req.comment.text.clone_from(&self.text);
        req.do_not_store = Some(self.do_not_store);
        req.session_id = Some(self.session_id.clone());
        req.community_id = Some(self.community_id.clone());
        req.client_token = Some(self.client_token.clone());
        req.span_annotations = Some(self.span_annotations);

        for attr in self.requested_attributes.iter() {
            match attr {
                Attribute::IdentityAttack(None) => req.requested_attributes.identity_attack = Some(ScoreOptions::new()),
                Attribute::IdentityAttack(Some(limit)) => req.requested_attributes.identity_attack = Some(ScoreOptions::new().with_limit(*limit)),
                Attribute::Toxicity(None) => req.requested_attributes.toxicity = Some(ScoreOptions::new()),
                Attribute::Toxicity(Some(limit)) => req.requested_attributes.toxicity = Some(ScoreOptions::new().with_limit(*limit)),
                Attribute::Insult(None) => req.requested_attributes.insult = Some(ScoreOptions::new()),
                Attribute::Insult(Some(limit)) => req.requested_attributes.insult = Some(ScoreOptions::new().with_limit(*limit)),
                Attribute::Profanity(None) => req.requested_attributes.profanity = Some(ScoreOptions::new()),
                Attribute::Profanity(Some(limit)) => req.requested_attributes.profanity = Some(ScoreOptions::new().with_limit(*limit)),
                Attribute::Threat(None) => req.requested_attributes.threat = Some(ScoreOptions::new()),
                Attribute::Threat(Some(limit)) => req.requested_attributes.threat = Some(ScoreOptions::new().with_limit(*limit)),
                Attribute::SevereToxicity(None) => req.requested_attributes.severe_toxicity = Some(ScoreOptions::new()),
                Attribute::SevereToxicity(Some(limit)) => req.requested_attributes.severe_toxicity = Some(ScoreOptions::new().with_limit(*limit)),
            }
        }
        req
    }
    pub fn truncate_comment(mut self) -> Self {
        self.truncate = true;
        self
    }
    pub fn do_not_store(mut self) -> Self {
        self.do_not_store = true;
        self
    }
    pub fn with_attribute(mut self, attr: Attribute) -> Self {
        self.requested_attributes.push(attr);
        self
    }
    pub fn with_session_id(mut self, session_id: &str) -> Self {
        self.session_id = session_id.to_owned();
        self
    }
    pub fn with_community_id(mut self, community_id: &str) -> Self {
        self.community_id = community_id.to_owned();
        self
    }
    pub fn with_client_token(mut self, client_token: &str) -> Self {
        self.client_token = client_token.to_owned();
        self
    }
    /// sets the span_annotations flag in the request. See Request docs for more information.
    pub fn results_per_sentence(mut self) -> Self {
        self.span_annotations = true;
        self
    }
}
