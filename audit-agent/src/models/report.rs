use super::vulnerability::Vulnerability;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VulnerabilityReport {
    vulnerabilities: Vec<Vulnerability>,
}
