#![doc = " # Metrics Collector - Collect and structure metrics"]
#![doc = ""]
#![doc = " Recolector de métricas para comandos TRAE"]
use crate::commands::repair::RepairResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct MetricsCollector {
    pub id: Uuid,
    pub command: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
    pub metrics: HashMap<String, Value>,
    pub success: Option<bool>,
    pub error: Option<String>,
}
impl MetricsCollector {
    #[doc = "Method documentation added by AI refactor"]
    pub fn new(command: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            command,
            start_time: Utc::now(),
            end_time: None,
            duration: None,
            metrics: HashMap::new(),
            success: None,
            error: None,
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn record_build_time(&mut self, duration: Duration) {
        self.metrics.insert(
            "build_time_ms".to_string(),
            Value::Number(serde_json::Number::from(duration.as_millis() as u64)),
        );
        self.duration = Some(duration);
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn record_build_result(&mut self, success: bool) {
        self.success = Some(success);
        self.metrics
            .insert("build_success".to_string(), Value::Bool(success));
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn record_repair_time(&mut self, duration: Duration) {
        self.metrics.insert(
            "repair_time_ms".to_string(),
            Value::Number(serde_json::Number::from(duration.as_millis() as u64)),
        );
        self.duration = Some(duration);
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn record_repairs_applied(&mut self, results: &[RepairResult]) {
        let successful = results.iter().filter(|r| r.success).count();
        let failed = results.len() - successful;
        self.metrics.insert(
            "repairs_successful".to_string(),
            Value::Number(serde_json::Number::from(successful)),
        );
        self.metrics.insert(
            "repairs_failed".to_string(),
            Value::Number(serde_json::Number::from(failed)),
        );
        self.metrics.insert(
            "total_repairs".to_string(),
            Value::Number(serde_json::Number::from(results.len())),
        );
        self.success = Some(successful > 0);
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn add_custom_metric<T: Into<Value>>(&mut self, key: String, value: T) {
        self.metrics.insert(key, value.into());
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn finish(&mut self) {
        self.end_time = Some(Utc::now());
        if self.duration.is_none() {
            if let Some(end) = self.end_time {
                self.duration = end.signed_duration_since(self.start_time).to_std().ok();
            }
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_else(|e| {
            log::warn!("Failed to serialize metrics to JSON: {e}");
            serde_json :: json ! ({ "error" : "serialization_failed" })
        })
    }
}
