use agents::{
    deduplication_agent::{FormatDeduplicationAgent, FormatDeduplicationAgentTrait},
    multi_agent_system::{MultiAIAgentSystem, MultiAIAgentSystemTrait},
};
mod agents;
mod config;
mod examples;
mod models;
use examples::deposit_withdraw_contract::_DEPOSIT_WITHDRAW_CONTRACT;
use tokio;

#[tokio::main]
async fn main() {
    println!("Start");

    // Initialize the multi-AI agent system
    let system = MultiAIAgentSystem::new();

    // Analyze the contract in parallel
    let results = system.analyze_contract(&_DEPOSIT_WITHDRAW_CONTRACT).await;

    // Print the intermediate results
    // for (agent_name, output) in results {
    //     println!("=== {} ===", agent_name);
    //     println!("{}", output);
    //     println!();
    // }

    // Deduplicate and format vulnerabilities list
    let fd_agent = FormatDeduplicationAgent::new();
    let values: Vec<&String> = results.values().collect();
    let json_dedup = fd_agent.format_and_deduplicate(values, system.client).await;

    // Print after deduplication
    // println!("Deduplicated: \n{:?}", json_dedup);

    // Parse into a report
    let report = FormatDeduplicationAgent::process_result(json_dedup);

    // Print a report
    println!("{:#?}", report);

    println!("Done!");
}
