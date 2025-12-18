use std::fs;
use std::path::PathBuf;

use uuid::Uuid;

use trae_cli::commands::analyze::AnalyzeCommand;

#[tokio::test]
async fn analyze_cache_and_force_refresh_and_profile() {
    // Create temp workspace
    let mut dir = std::env::temp_dir();
    dir.push(format!("trae_test_{}", Uuid::new_v4()));
    fs::create_dir_all(&dir).expect("create temp dir");

    // add a simple source file so analyzer has something to scan
    let mut src = dir.clone();
    src.push("main.rs");
    fs::write(&src, "fn main() { println!(\"hello\"); }\n").expect("write main.rs");

    // switch cwd for the test
    let orig = std::env::current_dir().expect("pwd");
    std::env::set_current_dir(&dir).expect("chdir");

    // Run analyze without forcing refresh -> should create cache
    let res = AnalyzeCommand::run_simple(false, false, false, true, None, false, None).await;
    assert!(res.is_ok());

    let cache_dir = PathBuf::from(".trae").join("cache");
    assert!(cache_dir.exists(), "cache dir not created");
    let entries: Vec<_> = fs::read_dir(&cache_dir).expect("read cache dir").filter_map(|e| e.ok()).collect();
    assert!(!entries.is_empty(), "no cache files created");

    // pick the first cache file and get modified time
    let cache_file = entries[0].path();
    let _meta = fs::metadata(&cache_file).expect("stat cache");

    // Remove cache file and force refresh; ensure new cache is created
    let _ = fs::remove_file(&cache_file);
    // Sleep to ensure FS settles
    std::thread::sleep(std::time::Duration::from_millis(200));
    let res2 = AnalyzeCommand::run_simple(false, false, false, true, None, true, None).await;
    assert!(res2.is_ok());
    // there should be at least one cache file now
    let entries_after: Vec<_> = fs::read_dir(&cache_dir).expect("read cache dir after").filter_map(|e| e.ok()).collect();
    assert!(!entries_after.is_empty(), "cache not created after force_refresh");

    // Test profile output writing
    let out_path = "analysis_out.json";
    let res3 = AnalyzeCommand::run_simple(false, false, false, true, Some("fast".to_string()), true, Some(out_path.to_string())).await;
    assert!(res3.is_ok());
    let full = fs::read_to_string(out_path).expect("read output");
    let v: serde_json::Value = serde_json::from_str(&full).expect("parse json");
    let profile = v.get("analysis").and_then(|a| a.get("profile")).and_then(|p| p.as_str()).unwrap_or("");
    assert_eq!(profile, "fast");

    // cleanup and restore cwd
    let _ = fs::remove_file(out_path);
    let _ = fs::remove_dir_all(dir);
    std::env::set_current_dir(orig).expect("restore cwd");
}
