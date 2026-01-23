//! Biblioteca pública mínima de TRAE-CLI para reutilización por otros binarios/crates.
//! Reexporta módulos clave (jarvix client, metrics, core) con API estable mínima.

pub mod api;
pub mod cli;
pub mod commands;
pub mod config;
pub mod core;
pub mod jarvix;
pub mod metrics;
pub mod performance_patterns;
pub mod utils;

// Re-exportos útiles
pub use api::{analyze, cargo_run, repair, test_cmd};
pub use core::analyzer::*;
pub use jarvix::client::JarvixClient;
pub use metrics::collector::MetricsCollector;
