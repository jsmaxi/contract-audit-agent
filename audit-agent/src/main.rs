use actix_cors::Cors;
use actix_web::{http, middleware, web, /*App,*/ HttpResponse, /*HttpServer,*/ Responder,};
use actix_web::{/*get,*/ web::ServiceConfig};
use agents::chat_agent::{ChatAgent, ChatAgentTrait};
use agents::fixing_agent::{FixingAgent, FixingAgentTrait};
use agents::{
    deduplication_agent::{FormatDeduplicationAgent, FormatDeduplicationAgentTrait},
    multi_agent_system::{MultiAIAgentSystem, MultiAIAgentSystemTrait},
};
use config::config::{VALID_LANGUAGES, VALID_MODELS};
use models::report::VulnerabilityReport;
use server::request::{ChatRequest, FixRequest};
use server::response::{ChatResponse, FixResponse};
use server::{
    request::AuditRequest,
    response::{AuditErrorResponse, AuditResponse},
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use std::path::Path;
use std::time::Instant;
use std::{env, fs};
use vault::read_vault::try_read_report_from_vault;
use vault::write_vault::try_write_report_to_vault;
mod agents;
mod config;
mod models;
mod server;
mod vault;

#[actix_web::post("/audit")]
async fn audit_contract(request: web::Json<AuditRequest>) -> impl Responder {
    println!("Audit called [{}] [{}]", request.model, request.language);

    // Measure execution time
    let start = Instant::now();

    let request: AuditRequest = request.into_inner();

    if request.contract_code.trim().is_empty()
        || request.language.trim().is_empty()
        || request.model.trim().is_empty()
    {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Request parameters cannot be empty".to_string(),
        });
    }

    if !VALID_MODELS.contains(&request.model.as_str()) {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Selected AI model is not supported".to_string(),
        });
    }

    if !VALID_LANGUAGES.contains(&request.language.as_str()) {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Selected smart contract language is not supported".to_string(),
        });
    }

    // Initialize the multi-AI agent system. Arc / Mutex for the future
    let system: MultiAIAgentSystem =
        MultiAIAgentSystem::new(request.model.clone(), &request.language);

    // Analyze the contract in parallel
    let results = system
        .analyze_contract(&request.contract_code, &request.model, &request.language)
        .await;

    // Print the intermediate results (uncomment if debugging)
    // for (agent_name, output) in results {
    //     println!("=== {} ===", agent_name);
    //     println!("{}", output);
    //     println!();
    // }

    // Deduplicate and format vulnerabilities list
    let fd_agent = FormatDeduplicationAgent::new();
    let values: Vec<&String> = results.values().collect();
    let json_dedup = fd_agent
        .format_and_deduplicate(
            values,
            system.client.clone(),
            &request.model.clone(),
            &request.language,
        )
        .await;

    // Print after deduplication (uncomment if debugging)
    // println!("Deduplicated: \n{:?}", json_dedup);

    // Parse into a report
    let report: VulnerabilityReport = FormatDeduplicationAgent::process_result(json_dedup);

    // Print a report (uncomment if debugging)
    println!("{:#?}", report);

    println!("Audit Done");

    let duration = start.elapsed();
    println!("Execution Time: {:?}", duration);

    println!("Start persisting in vault");

    let id = try_write_report_to_vault(&report);
    let _id = id.unwrap_or_else(|| "no ID generated".to_string());

    println!("End persisting in vault");

    println!("Start reading from vault");

    let r = try_read_report_from_vault(&_id);
    let _report = r.unwrap_or_else(|| vec![]);

    println!("End reading from vault");

    HttpResponse::Ok().json(AuditResponse { report, _id })
}

#[actix_web::post("/fix")]
async fn fix_contract(request: web::Json<FixRequest>) -> impl Responder {
    println!("Fix called [{}] [{}]", request.model, request.language);

    // Measure execution time
    let start = Instant::now();

    let request: FixRequest = request.into_inner();

    if request.contract_code.trim().is_empty()
        || request.language.trim().is_empty()
        || request.model.trim().is_empty()
        || request.vulnerabilities.is_empty()
    {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Request parameters cannot be empty".to_string(),
        });
    }

    if !VALID_MODELS.contains(&request.model.as_str()) {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Selected AI model is not supported".to_string(),
        });
    }

    if !VALID_LANGUAGES.contains(&request.language.as_str()) {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Selected smart contract language is not supported".to_string(),
        });
    }

    let fixed_code: Option<String> = FixingAgent::fix(
        request.contract_code,
        request.vulnerabilities,
        request.model.as_str(),
        &request.language,
    )
    .await;

    // Print the result (uncomment if debugging)
    println!("{}", fixed_code.clone().unwrap_or("None".to_string()));

    println!("Fix Done");

    let duration = start.elapsed();
    println!("Execution Time: {:?}", duration);

    match fixed_code {
        Some(f) => HttpResponse::Ok().json(FixResponse { code: f }),
        None => HttpResponse::InternalServerError().json(AuditErrorResponse {
            error: "No fixed code returned".to_string(),
        }),
    }
}

#[actix_web::post("/chat")]
async fn chat_ai(request: web::Json<ChatRequest>) -> impl Responder {
    println!("Chat called [{}]", request.model);

    // Measure execution time
    let start = Instant::now();

    let request: ChatRequest = request.into_inner();

    if request.text.trim().is_empty() || request.model.trim().is_empty() {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Request parameters cannot be empty".to_string(),
        });
    }

    if !VALID_MODELS.contains(&request.model.as_str()) {
        let duration = start.elapsed();
        println!("Execution Time: {:?}", duration);
        return HttpResponse::BadRequest().json(AuditErrorResponse {
            error: "Selected AI model is not supported".to_string(),
        });
    }

    let ai_response: Option<String> = ChatAgent::chat(request.text, request.model.as_str()).await;

    // Print the result (uncomment if debugging)
    println!("{}", ai_response.clone().unwrap_or("None".to_string()));

    println!("Chat Done");

    let duration = start.elapsed();
    println!("Execution Time: {:?}", duration);

    match ai_response {
        Some(t) => HttpResponse::Ok().json(ChatResponse { text: t }),
        None => HttpResponse::InternalServerError().json(ChatResponse {
            text: "No response from AI".to_string(),
        }),
    }
}

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
    }))
}

#[actix_web::get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome. Please call (post) '/audit' endpoint with contract code string, contract language string and agent model string.")
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/plain")
        .body("API route not found")
}

fn print_folder_structure(path: &Path, indent: usize) {
    if let Some(file_name) = path.file_name() {
        if file_name == "node_modules" {
            return; // Skip
        }
    }

    if path.is_dir() {
        println!(
            "{:indent$}📁 {}",
            "",
            path.file_name().unwrap().to_string_lossy(),
            indent = indent
        );

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    // Recursively print the contents of the directory
                    print_folder_structure(&entry_path, indent + 4);
                }
            }
        }
    } else {
        println!(
            "{:indent$}📄 {}",
            "",
            path.file_name().unwrap().to_string_lossy(),
            indent = indent
        );
    }
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // let port = 8080;
    // let server = "127.0.0.1";
    // println!("Starting server {server} on port {port}");

    // Get secret from server
    let api_key: String = secrets
        .get("OPENAI_API_KEY")
        .expect("Secret API KEY is not set");
    // Set environment variable for GenAI
    std::env::set_var("OPENAI_API_KEY", api_key);

    let root_path: &Path = Path::new(".");
    println!("Folder structure for: {:?}", root_path);
    print_folder_structure(root_path, 0);

    // Allowed caller
    let allowed_origin1 = "http://localhost:3000";
    let allowed_origin2 = "https://contract-audit-ui.vercel.app";
    let allowed_origin3 = "https://contract-audit-ui-production.up.railway.app";

    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
            .allowed_origin(allowed_origin1)
            .allowed_origin(allowed_origin2)
            .allowed_origin(allowed_origin3)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(86400);

        cfg.service(
            web::scope("/")
                .wrap(cors) // Apply the CORS middleware
                .wrap(middleware::Compress::default()),
        )
        .service(home)
        .service(audit_contract)
        .service(fix_contract)
        .service(chat_ai)
        .service(health_check)
        .default_service(web::route().to(not_found));
    };

    Ok(config.into())

    // HttpServer::new(move || {
    //     let cors = Cors::default()
    //         .allowed_origin(allowed_origin)
    //         .allowed_methods(vec!["GET", "POST"])
    //         .allowed_headers(vec![
    //             http::header::AUTHORIZATION,
    //             http::header::CONTENT_TYPE,
    //         ])
    //         .max_age(3600);

    //     App::new()
    //         .app_data(web::Data::new(system.clone()))
    //         .wrap(middleware::Compress::default())
    //         .wrap(cors) // Apply the CORS middleware
    //         .route("/", web::get().to(home))
    //         .route("/audit", web::post().to(audit_contract))
    //         .service(health_check)
    //         .default_service(web::route().to(not_found))
    // })
    // .bind((server, port))?
    // .run()
    // .await
}
