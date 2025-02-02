use genai::{
    chat::{ChatMessage, ChatOptions, ChatRequest},
    Client, ClientConfig,
};
mod models;
use models::report::VulnerabilityReport;
use serde_json::json;
use std::{collections::HashMap, env, sync::Arc};
use tokio;

pub(crate) trait AIAgentTrait {
    fn new(name: &str, role_prompt: &str) -> Self;
    async fn analyze(&self, contract_code: &str, client: Arc<Client>) -> Option<String>;
}

#[derive(Clone)]
pub(crate) struct AIAgent {
    name: String,
    prompt: String,
}

impl AIAgent {
    fn get_output_prompt(&self) -> &str {
        "Return your findings in the following JSON format:
        {
            \"vulnerabilities\": [
                {
                    \"name\": \"vulnerability name\",
                    \"severity\": \"critical/high/medium/low\",
                    \"description\": \"detailed description\",
                    \"location\": \"function or line reference\",
                    \"impacted_code\": \"impacted code block\",
                    \"recommendations\": \"how to fix\"
                }
            ]
        }
        If no vulnerabilities are found, return an empty array.
        If the input is invalid Solidity code, the output should be a JSON object with an error message."
    }
}

impl AIAgentTrait for AIAgent {
    fn new(name: &str, prompt: &str) -> Self {
        AIAgent {
            name: name.to_string(),
            prompt: prompt.to_string(),
        }
    }

    async fn analyze(&self, contract_code: &str, client: Arc<Client>) -> Option<String> {
        let prompt: String = format!(
            "{} {} \n\nAnalyze this Solidity code for vulnerabilities:\n\n{}",
            self.prompt,
            self.get_output_prompt(),
            contract_code
        );

        let chat_req = ChatRequest::new(vec![ChatMessage::system(prompt)]);
        let model = MODEL_AND_KEY_ENV_NAME.0;

        match client.exec_chat(model, chat_req.clone(), None).await {
            Ok(response) => response.content_text_into_string(),
            Err(_e) => None,
        }
    }
}

// Define specialized agents

fn create_reentrancy_agent() -> AIAgent {
    let role_prompt = "
    You are a smart contract security expert specializing in detecting reentrancy vulnerabilities.
    Reentrancy occurs when a contract calls an external contract before updating its state, allowing the external contract to call back into the original contract.
    Analyze the provided Solidity code and identify any reentrancy vulnerabilities.
    Avoid suggestions for future development.
    ";
    AIAgent::new("Reentrancy Agent", role_prompt)
}

fn create_integer_overflow_agent() -> AIAgent {
    let role_prompt = "
    You are a smart contract security expert specializing in detecting integer overflow/underflow vulnerabilities.
    Integer overflow/underflow occurs when arithmetic operations exceed the maximum or minimum limits of the data type.
    Analyze the provided Solidity code and identify any integer overflow/underflow vulnerabilities. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ";
    AIAgent::new("Integer Overflow Agent", role_prompt)
}

fn create_access_control_agent() -> AIAgent {
    let role_prompt = "
    You are a smart contract security expert specializing in detecting access control vulnerabilities.
    Access control vulnerabilities occur when functions or state variables are not properly restricted, allowing unauthorized users to access or modify them.
    Analyze the provided Solidity code and identify any access control vulnerabilities. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ";
    AIAgent::new("Access Control Agent", role_prompt)
}

fn create_contract_validation_agent() -> AIAgent {
    let role_prompt = "
    You are a smart contract security expert specializing in detecting validation issues and vulnerabilities.
    Make sure all variables have valid values, validate amounts, suggest when to use Solidity requires if missing.
    Analyze the provided Solidity code and identify any validation vulnerabilities. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ";
    AIAgent::new("Validation Agent", role_prompt)
}

fn create_events_agent() -> AIAgent {
    let role_prompt = "
    You are a smart contract security expert specializing in detecting issues with events.
    Make sure to suggest when events should be emitted in the provided code or when events are redundant. 
    Analyze the provided Solidity code and identify any event issues. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ";
    AIAgent::new("Events Agent", role_prompt)
}

fn create_gas_agent() -> AIAgent {
    let role_prompt = "
    You are a smart contract security expert specializing in detecting gas optimisation issues.
    Analyze the provided Solidity code and identify any gas optimisation issues os suggestions. Provide detailed explanations and suggestions for fixes.
    Avoid suggestions for future development.
    ";
    AIAgent::new("Gas Agent", role_prompt)
}

fn create_general_security_agent() -> AIAgent {
    let role_prompt = "
    You are a smart contract security expert specializing in general security best practices.
    Analyze the provided Solidity code and identify any security vulnerabilities or bad practices. Provide detailed explanations and suggestions for fixes. 
    Avoid suggestions for future development.
    ";
    AIAgent::new("General Security Agent", role_prompt)
}

pub(crate) struct MultiAIAgentSystem {
    agents: Vec<AIAgent>,
    client: Arc<Client>, // Arc for thread-safe sharing
}

const MODEL_OPENAI: &str = "gpt-4o-mini"; // o1-mini, gpt-4o-mini
const MODEL_AND_KEY_ENV_NAME: (&str, &str) = (MODEL_OPENAI, "OPENAI_API_KEY");

impl MultiAIAgentSystem {
    fn create_agent_client() -> Arc<Client> {
        let _ = env::var(MODEL_AND_KEY_ENV_NAME.1).expect("API KEY is not set");

        // Temperature controls how creative/predictive the AI's output is
        let client_config =
            ClientConfig::default().with_chat_options(ChatOptions::default().with_temperature(0.0));

        let client = Client::builder().with_config(client_config).build();

        let model = MODEL_AND_KEY_ENV_NAME.0;

        let adapter_kind = client
            .resolve_service_target(model)
            .unwrap()
            .model
            .adapter_kind;

        println!("===== MODEL: {model} ({adapter_kind}) =====");

        Arc::new(client)
    }
}

pub(crate) trait MultiAIAgentSystemTrait {
    fn new() -> Self;
    async fn analyze_contract(&self, contract_code: &str) -> HashMap<String, String>;
}

impl MultiAIAgentSystemTrait for MultiAIAgentSystem {
    fn new() -> Self {
        MultiAIAgentSystem {
            agents: vec![
                create_reentrancy_agent(),
                create_integer_overflow_agent(),
                create_access_control_agent(),
                create_events_agent(),
                create_contract_validation_agent(),
                create_gas_agent(),
                create_general_security_agent(),
            ],
            client: Self::create_agent_client(),
        }
    }

    async fn analyze_contract(&self, contract_code: &str) -> HashMap<String, String> {
        let mut tasks = Vec::new();

        // Spawn a task for each agent to analyze the contract in parallel
        for agent in self.agents.clone() {
            let contract_code = contract_code.to_string();
            let client = Arc::clone(&self.client);
            let agent_name = agent.name.clone();

            tasks.push(tokio::spawn(async move {
                println!("Running {} ...", agent_name);
                match agent.analyze(&contract_code, client).await {
                    Some(response) => Some((agent_name, response)),
                    None => None,
                }
            }));
        }

        // Wait for all tasks to complete and collect results
        let mut results = HashMap::new();

        for task in tasks {
            match task.await {
                Ok(response) => match response {
                    Some((agent_name, output)) => results.insert(agent_name, output),
                    None => {
                        println!("Agent didn't return a response");
                        None
                    }
                },
                Err(e) => {
                    println!("Agent task failed: {:?}", e);
                    None
                }
            };
        }

        results
    }
}

struct FormatDeduplicationAgent;

pub(crate) trait FormatDeduplicationAgentTrait {
    fn new() -> Self;
    async fn format_and_deduplicate<'a>(
        &self,
        vulnerabilities: Vec<&'a String>,
        client: Arc<Client>,
    ) -> Option<String>;
    fn trim_json(input: &str) -> &str;
}

impl FormatDeduplicationAgentTrait for FormatDeduplicationAgent {
    fn new() -> Self {
        FormatDeduplicationAgent {}
    }

    async fn format_and_deduplicate<'a>(
        &self,
        vulnerabilities: Vec<&'a String>,
        client: Arc<Client>,
    ) -> Option<String> {
        if vulnerabilities.is_empty() {
            println!("Empty list of vulnerabilities");
            return None;
        }

        let combined_json = json!({
            "All": vulnerabilities
        });

        //println!("{:?}", combined_json);

        let format = "
        Make sure that the output JSON list adhere to this format:
        {
            \"vulnerabilities\": [
                {
                    \"name\": \"vulnerability name\",
                    \"severity\": \"critical/high/medium/low\",
                    \"description\": \"detailed description\",
                    \"location\": \"function or line reference\",
                    \"impacted_code\": \"impacted code block\",
                    \"recommendations\": \"how to fix\"
                }
            ]
        }
        ";

        let prompt: String = format!(
            r#"Here is a list of smart contract vulnerabilities. Some may be duplicates with slightly different wording.
            Please analyze and combine duplicate entries, keeping the most detailed and relevant description and recommendations.
            Return the deduplicated list in the same JSON format.

            {}
    
            {}

            Return only the JSON, no additional text."#,
            combined_json.to_string(),
            format,
        );

        let chat_req = ChatRequest::new(vec![ChatMessage::system(prompt)]);
        let model = MODEL_AND_KEY_ENV_NAME.0;

        match client.exec_chat(model, chat_req.clone(), None).await {
            Ok(response) => response.content_text_into_string(),
            Err(_e) => None,
        }
    }

    fn trim_json(input: &str) -> &str {
        // Remove the leading and trailing code block markers
        input
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim()
    }
}

fn process_result(json_dedup: Option<String>) -> () {
    if let Some(json_dedup) = json_dedup {
        // Trim
        let trimmed = FormatDeduplicationAgent::trim_json(json_dedup.as_str());

        // Parse
        let report: VulnerabilityReport = serde_json::from_str(trimmed).unwrap();

        // Print
        println!("{:#?}", report);
    }
}

#[tokio::main]
async fn main() {
    let contract_code = r#"
    pragma solidity ^0.8.0;

    contract VulnerableContract {
        mapping(address => uint) public balances;

        function deposit() public payable {
            balances[msg.sender] += msg.value;
        }

        function withdraw(uint _amount) public {
            require(balances[msg.sender] >= _amount);
            (bool success, ) = msg.sender.call{value: _amount}("");
            require(success);
            balances[msg.sender] -= _amount;
        }
    }
    "#;

    // Initialize the multi-AI agent system
    let system = MultiAIAgentSystem::new();

    // Analyze the contract in parallel
    let results = system.analyze_contract(contract_code).await;

    // Print the results
    // for (agent_name, output) in results {
    //     println!("=== {} ===", agent_name);
    //     println!("{}", output);
    //     println!();
    // }

    let values: Vec<&String> = results.values().collect();

    let fd_agent = FormatDeduplicationAgent::new();

    let json_dedup = fd_agent.format_and_deduplicate(values, system.client).await;

    // println!("Deduplicated: \n{:?}", json_dedup);

    process_result(json_dedup);

    println!();
}
