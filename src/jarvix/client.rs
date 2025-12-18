#![doc = " # JARVIX Client - Client for JARVIXSERVER integration"]
#![doc = ""]
#![doc = " Cliente para comunicación con JARVIXSERVER"]
use crate::metrics::collector::MetricsCollector;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct JarvixConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub timeout: u64,
}
#[doc = "Struct documentation added by AI refactor"]
pub struct JarvixClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
}
impl JarvixClient {
    #[doc = "Method documentation added by AI refactor"]
    pub fn load_config() -> Result<JarvixConfig> {
        if let Ok(endpoint) = std::env::var("JARVIX_ENDPOINT") {
            return Ok(JarvixConfig {
                endpoint,
                api_key: std::env::var("JARVIX_API_KEY").ok(),
                timeout: std::env::var("JARVIX_TIMEOUT")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(30),
            });
        }
        if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            let config_path = format!("{home}/.trae/config.toml");
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str::<JarvixConfig>(&content) {
                    return Ok(config);
                }
            }
        }
        Ok(JarvixConfig {
            endpoint: "http://localhost:8081".to_string(),
            api_key: None,
            timeout: 30,
        })
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn new() -> Result<Option<Self>> {
        let config = Self::load_config()?;
        println!("🔧 JARVIX configurado: {}", config.endpoint);
        Ok(Some(Self {
            client: Client::new(),
            base_url: config.endpoint,
            api_key: config.api_key,
            timeout: Duration::from_secs(config.timeout),
        }))
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_build_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "build_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_repair_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "repair_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_scan_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "scan_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () , "performance_boost" : 400 });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn submit_parallel_analysis_job(
        &self,
        analysis_type: &str,
        data: serde_json::Value,
    ) -> Result<String> {
        let job_payload = json ! ({ "type" : analysis_type , "payload" : data , "worker_preference" : match analysis_type { "security_scan" => "nim" , "dependency_analysis" => "rust" , "performance_benchmark" => "c" , "math_optimization" => "julia" , _ => "rust" } , "priority" : "high" , "timeout_seconds" : 300 });
        let url = format!("{}/jobs", self.base_url);
        let mut request = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .json(&job_payload);
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }
        let response = request.send().await?;
        let response_json: serde_json::Value = response.json().await?;
        if let Some(job_id) = response_json.get("id").and_then(|id| id.as_str()) {
            Ok(job_id.to_string())
        } else {
            Err(anyhow::anyhow!("Failed to get job ID from response"))
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn get_job_result(&self, job_id: &str) -> Result<Option<serde_json::Value>> {
        let url = format!("{}/jobs/{}", self.base_url, job_id);
        let mut request = self.client.get(&url).timeout(self.timeout);
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }
        let response = request.send().await?;
        let job_data: serde_json::Value = response.json().await?;
        if let Some(status) = job_data.get("status").and_then(|s| s.as_str()) {
            match status {
                "finished" => {
                    if let Some(result) = job_data.get("result") {
                        Ok(Some(result.clone()))
                    } else {
                        Ok(None)
                    }
                }
                "failed" => {
                    let error = job_data
                        .get("error")
                        .and_then(|e| e.as_str())
                        .unwrap_or("Unknown error");
                    Err(anyhow::anyhow!("Job failed: {error}"))
                }
                _ => Ok(None),
            }
        } else {
            Err(anyhow::anyhow!("Invalid job response"))
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn get_pool_stats(&self) -> Result<serde_json::Value> {
        let url = format!("{}/pool/stats", self.base_url);
        let mut request = self.client.get(&url).timeout(self.timeout);
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }
        let response = request.send().await?;
        let stats: serde_json::Value = response.json().await?;
        Ok(stats)
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_cargo_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "cargo_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_test_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "test_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_doc_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "doc_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_security_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "security_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    pub async fn report_clippy_metrics(&self, metrics: MetricsCollector) -> Result<()> {
        let payload = json ! ({ "type" : "clippy_metrics" , "data" : metrics . to_json () , "timestamp" : chrono :: Utc :: now () });
        self.send_metrics(payload).await
    }
    #[doc = "Method documentation added by AI refactor"]
    async fn send_metrics(&self, payload: serde_json::Value) -> Result<()> {
        let url = format!("{}/trae/api/metrics", self.base_url);
        let mut request = self.client.post(&url).timeout(self.timeout).json(&payload);
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }
        let response = request.send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to send metrics: {}",
                response.status()
            ))
        }
    }
}
