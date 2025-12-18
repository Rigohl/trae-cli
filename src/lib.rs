//! Biblioteca pública mínima de TRAE-CLI para reutilización por otros binarios/crates.
//! Reexporta módulos clave (jarvix client, metrics, core) con API estable mínima.

pub mod cli;
pub mod config;
pub mod jarvix;
pub mod metrics;
pub mod core;
pub mod utils;
pub mod commands;
pub mod performance_patterns;
pub mod api;

// Re-exportos útiles
pub use jarvix::client::JarvixClient;
pub use metrics::collector::MetricsCollector;
pub use core::analyzer::*;
pub use api::{analyze, repair, test_cmd, cargo_run};
