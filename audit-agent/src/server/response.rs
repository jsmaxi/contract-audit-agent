use crate::models::report::VulnerabilityReport;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuditResponse {
    audit_id: String,
    vulnerabilities: VulnerabilityReport,
}
