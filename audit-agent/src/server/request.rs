use serde::Deserialize;
use validator::Validate;

use crate::models::vulnerability::Vulnerability;

#[derive(Debug, Deserialize, Validate)]
pub struct AuditRequest {
    #[validate(length(min = 1, message = "Contract code cannot be empty"))]
    pub contract_code: String,

    #[validate(length(min = 1, message = "Contract language must be specified"))]
    pub language: String,

    #[validate(length(min = 1, message = "AI model must be specified"))]
    pub model: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct FixRequest {
    #[validate(length(min = 1, message = "Contract code cannot be empty"))]
    pub contract_code: String,

    #[validate(length(min = 1, message = "Contract language must be specified"))]
    pub language: String,

    #[validate(length(min = 1, message = "AI model must be specified"))]
    pub model: String,

    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChatRequest {
    #[validate(length(min = 1, message = "Chat text cannot be empty"))]
    pub text: String,

    #[validate(length(min = 1, message = "AI model must be specified"))]
    pub model: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct HistoryRequest {
    #[validate(length(min = 1, message = "Report ID cannot be empty"))]
    pub id: String,
}
