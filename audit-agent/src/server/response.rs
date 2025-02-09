use crate::models::{report::VulnerabilityReport, vulnerability::Vulnerability};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuditResponse {
    pub report: VulnerabilityReport,
    pub _id: String,
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

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub report: Vec<Vulnerability>,
}
