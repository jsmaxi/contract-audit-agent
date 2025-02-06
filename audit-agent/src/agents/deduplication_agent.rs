use crate::models::report::VulnerabilityReport;
use genai::{
    chat::{ChatMessage, ChatRequest},
    Client,
};
use serde_json::json;
use std::sync::Arc;

pub struct FormatDeduplicationAgent;

pub(crate) trait FormatDeduplicationAgentTrait {
    fn new() -> Self;
    async fn format_and_deduplicate<'a>(
        &self,
        vulnerabilities: Vec<&'a String>,
        client: Arc<Client>,
        model: &str,
        language: &str,
    ) -> Option<String>;
    fn trim_json(input: &str) -> &str;
    fn process_result(json_dedup: Option<String>) -> VulnerabilityReport;
}

impl FormatDeduplicationAgentTrait for FormatDeduplicationAgent {
    fn new() -> Self {
        FormatDeduplicationAgent {}
    }

    async fn format_and_deduplicate<'a>(
        &self,
        vulnerabilities: Vec<&'a String>,
        client: Arc<Client>,
        model: &str,
        language: &str,
    ) -> Option<String> {
        if vulnerabilities.is_empty() {
            println!("Empty list of vulnerabilities");
            return None;
        }

        println!("Deduplicating vulnerabilities");

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
            r#"Here is a list of {} smart contract vulnerabilities. Some may be duplicates with slightly different wording.
            Please analyze and combine duplicate entries, keeping the most detailed and relevant description and recommendations.
            Return the deduplicated list in the same JSON format.

            {}
    
            {}

            Return only the JSON, no additional text."#,
            language,
            combined_json.to_string(),
            format,
        );

        let chat_req = ChatRequest::new(vec![ChatMessage::system(prompt)]);

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

    fn process_result(json_dedup: Option<String>) -> VulnerabilityReport {
        println!("Generating a report");
        if let Some(json_dedup) = json_dedup {
            // Trim
            let trimmed = FormatDeduplicationAgent::trim_json(json_dedup.as_str());

            // Parse
            let report: VulnerabilityReport = serde_json::from_str(trimmed).unwrap();

            // Return
            report
        } else {
            // Empty
            let report: VulnerabilityReport = VulnerabilityReport {
                vulnerabilities: Vec::new(),
            };
            report
        }
    }
}
