use super::{
    ai_agent::AIAgent,
    specialized_auditors::{
        create_access_control_agent, create_contract_validation_agent, create_events_agent,
        create_gas_agent, create_general_security_agent, create_integer_overflow_agent,
        create_reentrancy_agent,
    },
};
use crate::{
    agents::ai_agent::AIAgentTrait,
    config::config::{MODEL_ENV_KEY_NAME, MODEL_OPENAI, TEMPERATURE},
};
use genai::{chat::ChatOptions, Client, ClientConfig};
use std::{collections::HashMap, env, sync::Arc};

#[derive(Clone)]
pub struct MultiAIAgentSystem {
    agents: Vec<AIAgent>,
    pub client: Arc<Client>, // Arc for thread-safe sharing
}

impl MultiAIAgentSystem {
    fn create_agent_client() -> Arc<Client> {
        let _ = env::var(MODEL_ENV_KEY_NAME).expect("API KEY is not set");

        let client_config = ClientConfig::default()
            .with_chat_options(ChatOptions::default().with_temperature(TEMPERATURE));

        let client = Client::builder().with_config(client_config).build();

        let model = MODEL_OPENAI;

        let adapter_kind = client
            .resolve_service_target(model)
            .unwrap()
            .model
            .adapter_kind;

        println!("===== MODEL: {model} ({adapter_kind}) =====");

        Arc::new(client)
    }
}

pub trait MultiAIAgentSystemTrait {
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
