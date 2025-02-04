use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AuditRequest {
    #[validate(length(min = 1, message = "Contract code cannot be empty"))]
    pub contract_code: String,

    #[validate(length(min = 1, message = "Contract language must be specified"))]
    pub language: String,
}
