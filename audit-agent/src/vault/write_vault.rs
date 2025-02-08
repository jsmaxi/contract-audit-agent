use crate::models::report::VulnerabilityReport;
use std::panic::catch_unwind;
use std::process::{Command, Output};

// const WRITE_SCRIPT_PATH: &str = "../vault/writeReport.js";

pub fn try_write_report_to_vault(report: &VulnerabilityReport) -> Option<String> {
    let result = catch_unwind(|| {
        return write_report_to_vault(report);
    });

    match result {
        Ok(r) => Some(r),
        Err(_) => {
            println!("vault write panicked!");
            None
        }
    }
}

fn write_report_to_vault(report: &VulnerabilityReport) -> String {
    let vault_path = "vault/writeReport.js";

    println!("write path {}", vault_path);

    let input_json = serde_json::to_string(&report.vulnerabilities);

    if input_json.is_err() {
        panic!("Failed to convert the report into JSON for vault");
    }

    let input_json: String = input_json.unwrap();

    let _output = Command::new("node")
        .arg(vault_path)
        .arg(&input_json)
        .output();

    if _output.is_err() {
        let error_string = _output.unwrap_err().to_string();
        println!("Write error: {}", error_string);
        panic!("Write error: {}", error_string);
    }

    let output: Output = _output.unwrap();

    if output.status.success() {
        let resuld_id: String = String::from_utf8(output.stdout).unwrap().trim().to_string();
        resuld_id
    } else {
        panic!("Script failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}
