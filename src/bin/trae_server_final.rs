use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::Command;
use tiny_http::Header;
use tiny_http::{Method, Response, Server};
use walkdir::WalkDir;
#[derive(Debug, Serialize, Deserialize)]
#[doc = "Struct documentation added by AI refactor"]
struct ApiResponse<T> {
    status: String,
    data: Option<T>,
    error: Option<String>,
    timestamp: i64,
}
impl<T> ApiResponse<T> {
    #[doc = "Method documentation added by AI refactor"]
    fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    fn error(msg: String) -> ApiResponse<String> {
        ApiResponse {
            status: "error".to_string(),
            data: None,
            error: Some(msg),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}
#[derive(Debug, Serialize)]
#[doc = "Struct documentation added by AI refactor"]
struct AnalyzeResponse {
    total_files: usize,
    total_lines: usize,
    rust_files: usize,
    issues: Vec<Issue>,
    quality_score: f64,
}
#[derive(Debug, Serialize)]
#[doc = "Struct documentation added by AI refactor"]
struct Issue {
    file: String,
    line: usize,
    severity: String,
    message: String,
}
#[doc = "Function documentation added by AI refactor"]
fn analyze_project() -> Result<AnalyzeResponse, String> {
    let mut total_files = 0;
    let mut total_lines = 0;
    let mut rust_files = 0;
    let mut issues = Vec::new();
    for entry in WalkDir::new(".")
        .max_depth(5)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        if entry.file_type().is_file() {
            total_files += 1;
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    rust_files += 1;
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        let lines = content.lines().count();
                        total_lines += lines;
                        for (idx, line) in content.lines().enumerate() {
                            if line.contains("unwrap()") && !line.trim_start().starts_with("//") {
                                issues.push(Issue {
                                    file: entry.path().display().to_string(),
                                    line: idx + 1,
                                    severity: "warning".to_string(),
                                    message:
                                        "Consider using proper error handling instead of unwrap()"
                                            .to_string(),
                                });
                            }
                            if line.contains("panic!") && !line.trim_start().starts_with("//") {
                                issues.push(Issue {
                                    file: entry.path().display().to_string(),
                                    line: idx + 1,
                                    severity: "error".to_string(),
                                    message:
                                        "Panic detected - use Result/Option for error handling"
                                            .to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    let quality_score = if rust_files > 0 {
        (issues.len() as f64 / rust_files as f64).mul_add(-20.0, 100.0)
    } else {
        0.0
    };
    Ok(AnalyzeResponse {
        total_files,
        total_lines,
        rust_files,
        issues,
        quality_score: quality_score.clamp(0.0, 100.0),
    })
}
#[doc = "Function documentation added by AI refactor"]
fn run_repair() -> Result<String, String> {
    match Command::new("cargo")
        .arg("fix")
        .arg("--allow-dirty")
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let fixed = stdout.matches("Fixed").count();
            Ok(format!("Repair completed: {fixed} issues fixed"))
        }
        Err(e) => Err(format!("Repair failed: {e}")),
    }
}
#[doc = "Function documentation added by AI refactor"]
fn main() {
    println!("🚀 Starting TRAE CLI HTTP Server (Full Version)...");
    let server = Server::http("0.0.0.0:3001").expect("Failed to start server");
    println!("✅ TRAE CLI Server listening on http://0.0.0.0:3001");
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║            🚀 TRAE CLI HTTP Server                               ║");
    println!("║            Integrated with JARVIXSERVER                          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("Available endpoints:");
    println!("  GET  /health       - Health check");
    println!("  GET  /status       - Service status");
    println!("  POST /api/analyze  - Analyze project");
    println!("  POST /api/repair   - Repair issues");
    println!("  GET  /api/metrics  - System metrics");
    println!();
    fn make_json_response<T: serde::Serialize>(
        api: ApiResponse<T>,
    ) -> Response<std::io::Cursor<Vec<u8>>> {
        match serde_json::to_string(&api) {
            Ok(s) => match Header::from_bytes("Content-Type", "application/json") {
                Ok(h) => Response::from_string(s).with_header(h),
                Err(e) => {
                    eprintln!("Failed to build header: {:?}", e);
                    Response::from_string(
                        json ! ({ "status" : "error" , "error" : "header creation failed" })
                            .to_string(),
                    )
                    .with_status_code(500)
                }
            },
            Err(e) => {
                eprintln!("Failed to serialize response: {:?}", e);
                Response::from_string(
                    json ! ({ "status" : "error" , "error" : "serialization failed" }).to_string(),
                )
                .with_status_code(500)
            }
        }
    }
    for request in server.incoming_requests() {
        println!("📨 {} {}", request.method(), request.url());
        let response = match (request.method(), request.url()) {
            (&Method::Get, "/health") => Response::from_string("OK"),
            (&Method::Get, "/status") => make_json_response(ApiResponse::success(
                serde_json :: json ! ({ "service" : "trae-cli" , "version" : "0.1.0" , "status" : "operational" , "endpoints" : ["/health" , "/status" , "/api/analyze" , "/api/repair" , "/api/metrics"] }),
            )),
            (&Method::Post, "/api/analyze") => match analyze_project() {
                Ok(result) => make_json_response(ApiResponse::success(result)),
                Err(e) => make_json_response(ApiResponse::<String>::error(e)).with_status_code(500),
            },
            (&Method::Post, "/api/repair") => match run_repair() {
                Ok(result) => make_json_response(ApiResponse::success(result)),
                Err(e) => make_json_response(ApiResponse::<String>::error(e)).with_status_code(500),
            },
            (&Method::Get, "/api/metrics") => make_json_response(ApiResponse::success(
                serde_json :: json ! ({ "cpu_usage" : 0.0 , "memory_mb" : 0 , "build_time_ms" : 0 , "active_tasks" : 0 }),
            )),
            _ => Response::from_string("Not Found").with_status_code(404),
        };
        let _ = request.respond(response);
    }
}
