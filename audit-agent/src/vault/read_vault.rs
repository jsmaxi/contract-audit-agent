use crate::models::vulnerability::Vulnerability;
use std::panic::catch_unwind;
use std::process::{Command, Output};

const READ_SCRIPT_PATH: &str = "../vault/readReport.js";

pub fn try_read_report_from_vault(id: &str) -> Option<Vec<Vulnerability>> {
    let result = catch_unwind(|| {
        return read_report_from_vault(id);
    });

    match result {
        Ok(r) => Some(r),
        Err(_) => {
            println!("vault read panicked!");
            None
        }
    }
}

fn read_report_from_vault(id: &str) -> Vec<Vulnerability> {
    let output: Output = Command::new("node")
        .arg(READ_SCRIPT_PATH)
        .arg(&id)
        .output()
        .unwrap();

    if output.status.success() {
        let result: String = String::from_utf8(output.stdout).unwrap().trim().to_string();
        println!("Read result: {}", result);
        let parsed: Result<Vec<Vulnerability>, serde_json::Error> = serde_json::from_str(&result);
        match parsed {
            Ok(v) => {
                println!("Found {}", v.len());
                v
            }
            _ => {
                println!("Found empty");
                vec![]
            }
        }
    } else {
        panic!("Script failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}
