use std::sync::{Arc, Mutex};

use serde_json::Value;
use std::thread;
use tiny_http::{Response, Server};
use trae_cli::jarvix::client::JarvixClient;
use trae_cli::metrics::collector::MetricsCollector;
use trae_cli::commands::analyze::AnalyzeCommand;

#[tokio::test]
async fn jarvix_client_reports_scan_metrics_to_local_server() {
    // Shared state to capture last payload
    let received: Arc<Mutex<Option<Value>>> = Arc::new(Mutex::new(None));
    let received_clone = received.clone();

    // Start a tiny_http server on an available port
    let server = Server::http("127.0.0.1:0").expect("failed to bind tiny_http");
    let local_addr = server.server_addr();
    let s = Arc::new(server);
    let s_thread = s.clone();
    let handler_received = received_clone.clone();
    let handle = thread::spawn(move || {
        for mut request in s_thread.incoming_requests() {
            if request.url() == "/trae/api/metrics" && request.method().as_str() == "POST" {
                let mut body = String::new();
                request.as_reader().read_to_string(&mut body).ok();
                if let Ok(json) = serde_json::from_str::<Value>(&body) {
                    *handler_received.lock().unwrap() = Some(json);
                }
                let response = Response::from_string("ok").with_status_code(200);
                let _ = request.respond(response);
                break; // stop after first
            } else {
                let _ = request.respond(Response::from_string("not found").with_status_code(404));
            }
        }
    });

    // Set env so JarvixClient picks it up
    std::env::set_var("JARVIX_ENDPOINT", format!("http://{}", local_addr));

    // Build a simple metrics collector
    let mut metrics = MetricsCollector::new("test_metrics".to_string());
    metrics.add_custom_metric("foo".to_string(), 42);

    // Create client and report
    let client = JarvixClient::new().expect("client new").expect("client present");
    let res = client.report_scan_metrics(metrics).await;
    assert!(res.is_ok(), "report_scan_metrics failed: {:?}", res.err());

    // Allow server thread to process briefly
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Check received payload
    let guard = received.lock().unwrap();
    assert!(guard.is_some(), "Server did not receive payload");
    if let Some(v) = &*guard {
        assert!(v.get("type").is_some(), "payload missing type");
    }

    // Ensure server thread finishes
    let _ = handle.join();
}

#[tokio::test]
async fn analyze_command_run_simple_executes() {
    // Should run without requiring JARVIX
    let res = AnalyzeCommand::run_simple(false, false, false, true, None, false, None).await;
    assert!(res.is_ok(), "Analyze run_simple failed: {:?}", res.err());
}
