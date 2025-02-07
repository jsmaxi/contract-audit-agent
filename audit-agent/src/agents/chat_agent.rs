use crate::config::config::{MODEL_ENV_KEY_NAME, TEMPERATURE};
use genai::{
    chat::{ChatMessage, ChatOptions, ChatRequest},
    Client, ClientConfig,
};
use std::{env, sync::Arc};

pub struct ChatAgent;

pub(crate) trait ChatAgentTrait {
    async fn chat(text: String, model: &str) -> Option<String>;
}

impl ChatAgent {
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

impl ChatAgentTrait for ChatAgent {
    async fn chat(text: String, model: &str) -> Option<String> {
        if text.trim().is_empty() {
            println!("Empty request text");
            return None;
        }

        println!("Asking AI");

        let client: Arc<Client> = ChatAgent::create_ai_client(model);

        let prompt: String = format!(
            r#"You are blockchain expert specializing in smart contracts. 
            Answer this question concisely and clearly:

            {}
            
            Try to answer with no more than 500 characters. Try to avoid bullet list."#,
            text
        );

        let chat_req = ChatRequest::new(vec![ChatMessage::system(prompt)]);

        match client.exec_chat(model, chat_req.clone(), None).await {
            Ok(response) => response.content_text_into_string(),
            Err(_e) => None,
        }
    }
}
