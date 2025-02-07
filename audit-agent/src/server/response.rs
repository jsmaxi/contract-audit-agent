use crate::models::report::VulnerabilityReport;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuditResponse {
    pub report: VulnerabilityReport,
}

#[derive(Debug, Serialize)]
pub struct AuditErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct FixResponse {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub text: String,
}
