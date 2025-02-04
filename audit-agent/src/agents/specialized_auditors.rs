use super::ai_agent::{AIAgent, AIAgentTrait};
use crate::config::config::LANGUAGE;

pub fn create_reentrancy_agent() -> AIAgent {
    let role_prompt = format!("
    You are a {} smart contract security expert specializing in detecting reentrancy vulnerabilities.
    Reentrancy occurs when a contract calls an external contract before updating its state, allowing the external contract to call back into the original contract.
    Analyze the provided {} code and identify any reentrancy vulnerabilities.
    Avoid suggestions for future development.
    ", LANGUAGE, LANGUAGE);
    AIAgent::new("Reentrancy Agent", role_prompt)
}

pub fn create_integer_overflow_agent() -> AIAgent {
    let role_prompt = format!("
    You are a {} smart contract security expert specializing in detecting integer overflow/underflow vulnerabilities.
    Integer overflow/underflow occurs when arithmetic operations exceed the maximum or minimum limits of the data type.
    Analyze the provided {} code and identify any integer overflow/underflow vulnerabilities. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ", LANGUAGE, LANGUAGE);
    AIAgent::new("Integer Overflow Agent", role_prompt)
}

pub fn create_access_control_agent() -> AIAgent {
    let role_prompt = format!("
    You are a {} smart contract security expert specializing in detecting access control vulnerabilities.
    Access control vulnerabilities occur when functions or state variables are not properly restricted, allowing unauthorized users to access or modify them.
    Analyze the provided {} code and identify any access control vulnerabilities. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ", LANGUAGE, LANGUAGE);
    AIAgent::new("Access Control Agent", role_prompt)
}

pub fn create_contract_validation_agent() -> AIAgent {
    let role_prompt = format!("
    You are a {} smart contract security expert specializing in detecting validation issues and vulnerabilities.
    Make sure all variables have valid values, validate amounts, suggest when to use require checks if missing.
    Analyze the provided {} code and identify any validation vulnerabilities. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ", LANGUAGE, LANGUAGE);
    AIAgent::new("Validation Agent", role_prompt)
}

pub fn create_events_agent() -> AIAgent {
    let role_prompt = format!("
    You are a {} smart contract security expert specializing in detecting issues with events.
    Make sure to suggest when events should be emitted in the provided code or when events are redundant. 
    Analyze the provided {} code and identify any event issues. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ", LANGUAGE, LANGUAGE);
    AIAgent::new("Events Agent", role_prompt)
}

pub fn create_gas_agent() -> AIAgent {
    let role_prompt = format!("
    You are a {} smart contract security expert specializing in detecting gas optimisation issues.
    Analyze the provided {} code and identify any gas optimisation issues os suggestions. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ", LANGUAGE, LANGUAGE);
    AIAgent::new("Gas Agent", role_prompt)
}

pub fn create_general_security_agent() -> AIAgent {
    let role_prompt = format!("
    You are a {} smart contract security expert specializing in general security best practices.
    Analyze the provided {} code and identify any security vulnerabilities or bad practices. Provide detailed explanations and suggestions for fixes. 
    Avoid suggestions for future development.
    ", LANGUAGE, LANGUAGE);
    AIAgent::new("General Security Agent", role_prompt)
}
