// use agents::{
//     deduplication_agent::{FormatDeduplicationAgent, FormatDeduplicationAgentTrait},
//     multi_agent_system::{MultiAIAgentSystem, MultiAIAgentSystemTrait},
// };
// mod agents;
// mod config;
// mod examples;
// mod models;
// use examples::deposit_withdraw_contract::_DEPOSIT_WITHDRAW_CONTRACT;
// use tokio;

// mod server1;
// mod server2;
// mod server3;

// mod server;

// #[tokio::main]
// async fn main() {
//     server.ma
//     // println!("Start");

//     // // Initialize the multi-AI agent system
//     // let system = MultiAIAgentSystem::new();

//     // // Analyze the contract in parallel
//     // let results = system.analyze_contract(&_DEPOSIT_WITHDRAW_CONTRACT).await;

//     // // Print the intermediate results
//     // // for (agent_name, output) in results {
//     // //     println!("=== {} ===", agent_name);
//     // //     println!("{}", output);
//     // //     println!();
//     // // }

//     // // Deduplicate and format vulnerabilities list
//     // let fd_agent = FormatDeduplicationAgent::new();
//     // let values: Vec<&String> = results.values().collect();
//     // let json_dedup = fd_agent.format_and_deduplicate(values, system.client).await;

//     // // Print after deduplication
//     // // println!("Deduplicated: \n{:?}", json_dedup);

//     // // Parse into a report
//     // let report = FormatDeduplicationAgent::process_result(json_dedup);

//     // // Print a report
//     // println!("{:#?}", report);

//     // println!("Done!");
// }

use actix_web::{web, App, HttpResponse, HttpServer};
use agents::multi_agent_system::{MultiAIAgentSystem, MultiAIAgentSystemTrait};
mod agents;
mod config;
mod models;

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
    }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let server = "127.0.0.1";

    println!("Starting server {server} on port {port}");

    let system: MultiAIAgentSystem = MultiAIAgentSystem::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(system.clone()))
            .service(health_check)
    })
    .bind((server, port))?
    .run()
    .await
}
