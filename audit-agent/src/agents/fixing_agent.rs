use genai::{
    chat::{ChatMessage, ChatOptions, ChatRequest},
    Client, ClientConfig,
};
use std::{env, sync::Arc};

use crate::{
    config::config::{MODEL_ENV_KEY_NAME, TEMPERATURE},
    models::vulnerability::Vulnerability,
};

pub struct FixingAgent;

pub(crate) trait FixingAgentTrait {
    async fn fix(
        code: String,
        vulnerabilities: Vec<Vulnerability>,
        model: &str,
        language: &str,
    ) -> Option<String>;
}

impl FixingAgent {
    fn create_ai_client(model: &str) -> Arc<Client> {
        let _ = env::var(MODEL_ENV_KEY_NAME).expect("API KEY is not set");

        let client_config = ClientConfig::default()
            .with_chat_options(ChatOptions::default().with_temperature(TEMPERATURE));

        let client = Client::builder().with_config(client_config).build();

        let adapter_kind = client
            .resolve_service_target(model)
            .unwrap()
            .model
            .adapter_kind;

        println!("===== MODEL: {model} ({adapter_kind}) =====");

        Arc::new(client)
    }
}

impl FixingAgentTrait for FixingAgent {
    async fn fix(
        code: String,
        vulnerabilities: Vec<Vulnerability>,
        model: &str,
        language: &str,
    ) -> Option<String> {
        if vulnerabilities.is_empty() {
            println!("Empty list of vulnerabilities");
            return None;
        }

        println!("Fixing vulnerabilities");

        let json_string: String = serde_json::to_string(&vulnerabilities).unwrap();

        //println!("{:?}", json_string);

        let client: Arc<Client> = FixingAgent::create_ai_client(model);

        let prompt: String = format!(
            r#" Analyze the provided {} smart contract code and provided its vulnerability list in JSON. 
            Create a fixed version of the contract that addresses all listed vulnerabilities. 
            Fix only vulnerabilities listed in provided JSON. 
            Implement all fixes inline without suggesting alternatives. 
            Make no changes beyond fixing the listed vulnerabilities.

            Here is a list of vulnerabilities in JSON format:

            {}
    
            Return only the complete, fixed smart contract code. 
            Ensure the fixed code is fully functional and maintains the original contract's intended behavior.
            If you cannot fix or change the code, then return the original code.
            
            Here is a smart contract code:

            {}
            "#,
            language, json_string, code
        );

        let chat_req = ChatRequest::new(vec![ChatMessage::system(prompt)]);

        match client.exec_chat(model, chat_req.clone(), None).await {
            Ok(response) => response.content_text_into_string(),
            Err(_e) => None,
        }
    }
}
