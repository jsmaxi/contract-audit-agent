use crate::config::config::LANGUAGE;
use genai::{
    chat::{ChatMessage, ChatRequest},
    Client,
};
use std::sync::Arc;

pub(crate) trait AIAgentTrait {
    fn new(name: &str, prompt: String) -> Self;
    async fn analyze(
        &self,
        contract_code: &str,
        client: Arc<Client>,
        model: &str,
    ) -> Option<String>;
}

#[derive(Clone)]
pub(crate) struct AIAgent {
    pub name: String,
    pub prompt: String,
}

impl AIAgent {
    fn get_output_prompt(&self) -> String {
        format!(
            r#"Return your findings in the following JSON format:
            {{
                "vulnerabilities": [
                    {{
                        "name": "vulnerability name",
                        "severity": "critical/high/medium/low",
                        "description": "detailed description",
                        "location": "function or line reference",
                        "impacted_code": "impacted code block",
                        "recommendations": "how to fix"
                    }}
                ]
            }}
            If no vulnerabilities are found, return an empty array.
            If the input is invalid {} code, the output should be a JSON object with an error message."#,
            LANGUAGE
        )
    }
}

impl AIAgentTrait for AIAgent {
    fn new(name: &str, prompt: String) -> Self {
        AIAgent {
            name: name.to_string(),
            prompt,
        }
    }

    async fn analyze(
        &self,
        contract_code: &str,
        client: Arc<Client>,
        model: &str,
    ) -> Option<String> {
        let prompt: String = format!(
            "{} {} \n\nAnalyze this {} code for vulnerabilities:\n\n{}",
            self.prompt,
            self.get_output_prompt(),
            LANGUAGE,
            contract_code
        );

        let chat_req = ChatRequest::new(vec![ChatMessage::system(prompt)]);

        match client.exec_chat(model, chat_req.clone(), None).await {
            Ok(response) => response.content_text_into_string(),
            Err(_e) => None,
        }
    }
}
