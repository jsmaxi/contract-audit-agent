use crate::models::vulnerability::Vulnerability;
use std::panic::catch_unwind;
use std::process::{Command, Output};

// const READ_SCRIPT_PATH: &str = "../vault/readReport.js";

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
    let mut exe_path = std::env::current_exe().unwrap();
    exe_path.pop(); // Remove the executable name
    exe_path.push("vault");
    exe_path.push("readReport.js");

    // let vault_path = "/vault/readReport.js";

    let vault_path = exe_path.into_os_string().into_string().unwrap();

    println!("read path {}", vault_path);

    let current_dir = std::env::current_dir().unwrap();
    println!("Current working directory: {:?}", current_dir);
    println!("Current {}", current_dir.as_path().to_str().unwrap());

    let _output = Command::new("node").arg(vault_path).arg(&id).output();

    if _output.is_err() {
        let error_string = _output.unwrap_err().to_string();
        println!("Read error: {}", error_string);
        panic!("Read error: {}", error_string);
    }

    let output: Output = _output.unwrap();

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
