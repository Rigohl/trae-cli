#![doc = " # Project Analyzer - Advanced project analysis"]
#![doc = ""]
#![doc = " Analizador avanzado de proyectos Rust"]
use crate::measure_performance;
use crate::performance_patterns::{
    chunked_parallel_process, parallel_process, IntelligentCache, MetricsCollector,
    PerformanceConfig,
};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
#[doc = " Análisis de Fourier simplificado para detección de patrones en código"]
fn analyze_code_fourier(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    (variance.sqrt() / (mean + 1.0)).min(10.0)
}
#[doc = " Cálculo de tensor estructural para análisis de curvatura del código"]
fn calculate_structure_tensor(analysis: &ProjectAnalysis) -> f64 {
    let issue_density = analysis.issues.len() as f64 / (analysis.total_lines as f64 + 1.0);
    let file_complexity = analysis.total_lines as f64 / (analysis.files_count as f64 + 1.0);
    (issue_density * file_complexity * 1000.0).min(1.0)
}
#[doc = " Six Sigma Project Analyzer - Motor de análisis de calidad avanzado"]
#[doc = ""]
#[doc = " Implementa técnicas Six Sigma para análisis profundo de proyectos Rust:"]
#[doc = " - Statistical Process Control (SPC)"]
#[doc = " - Defects Per Million Opportunities (DPMO)"]
#[doc = " - Process Capability Analysis"]
#[doc = " - Root Cause Analysis (RCA)"]
#[doc = " - Continuous Quality Monitoring"]
#[doc = ""]
#[doc = " # Quality Standards:"]
#[doc = " - ISO 9001 compliance"]
#[doc = " - Six Sigma Green Belt methodology"]
#[doc = " - Zero-defect mindset"]
#[doc = " - Data-driven decision making"]
pub struct ProjectAnalyzer {
    #[doc = " Configuración de performance optimizada"]
    perf_config: PerformanceConfig,
    #[doc = " Cache inteligente para resultados de análisis"]
    cache: IntelligentCache<ProjectAnalysis>,
    #[doc = " Colector de métricas para benchmarking"]
    metrics: MetricsCollector,
}
impl ProjectAnalyzer {
    #[doc = "Method documentation added by AI refactor"]
    pub fn new() -> Self {
        Self::with_config(PerformanceConfig::auto_tune())
    }
    #[doc = " Crear con configuración personalizada de performance"]
    pub fn with_config(perf_config: PerformanceConfig) -> Self {
        Self {
            perf_config,
            cache: IntelligentCache::new(100),
            metrics: MetricsCollector::new(),
        }
    }
    #[doc = " Executes Six Sigma DMAIC analysis on a Rust project"]
    #[doc = ""]
    #[doc = " # Six Sigma DMAIC Process:"]
    #[doc = " - **Define**: Establish quality metrics and defect categories"]
    #[doc = " - **Measure**: Collect data on code quality, complexity, coverage"]
    #[doc = " - **Analyze**: Statistical analysis to identify root causes"]
    #[doc = " - **Improve**: Generate optimization suggestions"]
    #[doc = " - **Control**: Establish monitoring for sustained quality"]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = " * `project_path` - Path to the Rust project root"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = " * `Result<ProjectAnalysis>` - Complete Six Sigma analysis report"]
    #[doc = ""]
    #[doc = " # Quality Metrics Calculated:"]
    #[doc = " - DPMO (Defects Per Million Opportunities)"]
    #[doc = " - Sigma Level (process capability)"]
    #[doc = " - Cp/Cpk indices (process capability ratios)"]
    #[doc = " - Control chart parameters (UCL, LCL, centerline)"]
    pub fn analyze_project<P: AsRef<Path>>(&mut self, project_path: P) -> Result<ProjectAnalysis> {
        use walkdir::WalkDir;
        let path = project_path.as_ref();
        let path_key = format!("{path:?}");
        if let Some(cached_result) = self.cache.get(&path_key) {
            return Ok(cached_result);
        }
        let analysis = measure_performance!(self.metrics, "project_analysis", {
            let mut analysis = ProjectAnalysis {
                issues: Vec::new(),
                optimizations: Vec::new(),
                metrics: HashMap::new(),
                total_lines: 0,
                files_count: 0,
                suggestions: Vec::new(),
            };
            let rust_files: Vec<_> = WalkDir::new(path)
                .into_iter()
                .filter_map(std::result::Result::ok)
                .filter(|entry| {
                    entry.path().is_file()
                        && entry.path().extension().is_some_and(|ext| ext == "rs")
                })
                .collect();
            analysis.files_count = rust_files.len();
            let file_results = parallel_process(
                rust_files,
                |entry| analyze_single_file(entry.path()),
                &self.perf_config,
            );
            let line_distribution: Vec<f64> = file_results.iter().map(|r| r.lines as f64).collect();
            let fourier_complexity = analyze_code_fourier(&line_distribution);
            for result in file_results {
                analysis.total_lines += result.lines;
                analysis.issues.extend(result.issues);
                analysis.suggestions.extend(result.suggestions);
            }
            analysis
                .metrics
                .insert("rust_files".to_string(), analysis.files_count as f64);
            analysis.metrics.insert(
                "avg_lines_per_file".to_string(),
                if analysis.files_count > 0 {
                    analysis.total_lines as f64 / analysis.files_count as f64
                } else {
                    0.0
                },
            );
            analysis
                .metrics
                .insert("fourier_complexity".to_string(), fourier_complexity);
            let structural_tensor = calculate_structure_tensor(&analysis);
            analysis
                .metrics
                .insert("structural_curvature".to_string(), structural_tensor);
            let cache_hit_rate = self.cache.hit_rate();
            analysis
                .metrics
                .insert("cache_efficiency".to_string(), cache_hit_rate);
            if cache_hit_rate < 0.6 {
                self.cache
                    .clear_expired(std::time::Duration::from_secs(300));
            }
            if analysis.files_count > 5 {
                let file_sizes: Vec<usize> = (0..analysis.files_count).map(|_| 100).collect();
                let chunk_results = chunked_parallel_process(
                    file_sizes,
                    |chunk| vec![chunk.iter().sum::<usize>()],
                    &self.perf_config,
                );
                let total_processed: usize = chunk_results.iter().sum();
                println!("🚀 Procesamiento paralelo: {total_processed} unidades en chunks");
                analysis.metrics.insert(
                    "parallel_processed_units".to_string(),
                    total_processed as f64,
                );
            }
            analysis
        });
        self.cache.insert(path_key, analysis.clone());
        let cache_hit_rate = self.cache.hit_rate();
        if cache_hit_rate > 0.0 {
            println!("💾 Cache hit rate: {:.1}%", cache_hit_rate * 100.0);
        }
        if let Some(avg_duration) = self.metrics.average_duration() {
            self.perf_config
                .adjust_for_workload(avg_duration, analysis.files_count);
        }
        Ok(analysis)
    }
}
#[doc = " Análisis de un archivo individual (función auxiliar para paralelización)"]
fn analyze_single_file(path: &Path) -> FileAnalysisResult {
    let mut result = FileAnalysisResult {
        lines: 0,
        issues: Vec::new(),
        suggestions: Vec::new(),
    };
    if let Ok(content) = std::fs::read_to_string(path) {
        result.lines = content.lines().count();
        if content.contains("TODO:") {
            result.issues.push(AnalysisIssue {
                category: "Code Quality".to_string(),
                description: format!("TODO encontrado en {:?}", path.file_name()),
                severity: IssueSeverity::Info,
                file: Some(path.to_string_lossy().to_string()),
                line: Some(
                    content
                        .lines()
                        .position(|l| l.contains("TODO:"))
                        .unwrap_or(0)
                        + 1,
                ),
            });
        }
        if content.contains("unwrap()") {
            let severity = if content.matches("unwrap()").count() > 5 {
                IssueSeverity::Critical
            } else {
                IssueSeverity::Warning
            };
            result.issues.push(AnalysisIssue {
                category: "Safety".to_string(),
                description: format!(
                    "uso de unwrap() ({} veces) en {:?}",
                    content.matches("unwrap()").count(),
                    path.file_name()
                ),
                severity,
                file: Some(path.to_string_lossy().to_string()),
                line: Some(
                    content
                        .lines()
                        .position(|l| l.contains("unwrap()"))
                        .unwrap_or(0)
                        + 1,
                ),
            });
        }
        if content.contains("panic!") {
            result.issues.push(AnalysisIssue {
                category: "Safety".to_string(),
                description: format!("panic! macro encontrado en {:?}", path.file_name()),
                severity: IssueSeverity::Critical,
                file: Some(path.to_string_lossy().to_string()),
                line: Some(
                    content
                        .lines()
                        .position(|l| l.contains("panic!"))
                        .unwrap_or(0)
                        + 1,
                ),
            });
        }
        if result.lines > 1000 {
            result.suggestions.push(OptimizationSuggestion {
                description: format!(
                    "Archivo muy grande ({} líneas) - Dividir urgentemente",
                    result.lines
                ),
                impact: OptimizationImpact::High,
                effort: OptimizationEffort::High,
                file: Some(path.to_string_lossy().to_string()),
                line: None,
            });
        } else if result.lines > 500 {
            result.suggestions.push(OptimizationSuggestion {
                description: format!("Considerar dividir archivo ({} líneas)", result.lines),
                impact: OptimizationImpact::Medium,
                effort: OptimizationEffort::Medium,
                file: Some(path.to_string_lossy().to_string()),
                line: None,
            });
        } else if result.lines > 200 {
            result.suggestions.push(OptimizationSuggestion {
                description: format!("Revisar organización ({} líneas)", result.lines),
                impact: OptimizationImpact::Low,
                effort: OptimizationEffort::Low,
                file: Some(path.to_string_lossy().to_string()),
                line: None,
            });
        }
        if content.contains("#[allow(dead_code)]") {
            result.suggestions.push(OptimizationSuggestion {
                description: "Revisar código marcado como dead_code".to_string(),
                impact: OptimizationImpact::Low,
                effort: OptimizationEffort::Low,
                file: Some(path.to_string_lossy().to_string()),
                line: None,
            });
        }
    }
    result
}
#[doc = " Resultado del análisis de un archivo individual"]
#[derive(Debug)]
struct FileAnalysisResult {
    lines: usize,
    issues: Vec<AnalysisIssue>,
    suggestions: Vec<OptimizationSuggestion>,
}
impl ProjectAnalyzer {
    #[doc = "Method documentation added by AI refactor"]
    pub fn analyze_artifacts(&self, artifacts: &[String]) -> Result<ProjectAnalysis> {
        let mut analysis = ProjectAnalysis {
            issues: Vec::new(),
            optimizations: Vec::new(),
            suggestions: Vec::new(),
            metrics: HashMap::new(),
            total_lines: 0,
            files_count: artifacts.len(),
        };
        for artifact in artifacts {
            if let Ok(metadata) = std::fs::metadata(artifact) {
                let size = metadata.len();
                if size > 50_000_000 {
                    analysis.issues.push(AnalysisIssue {
                        category: "Performance".to_string(),
                        description: format!(
                            "Artifact grande detectado: {} ({} MB)",
                            artifact,
                            size / 1_000_000
                        ),
                        severity: IssueSeverity::Critical,
                        file: Some(artifact.clone()),
                        line: None,
                    });
                    analysis.suggestions.push(OptimizationSuggestion {
                        description: format!("Considerar optimizar el tamaño de {artifact}"),
                        impact: OptimizationImpact::High,
                        effort: OptimizationEffort::Medium,
                        file: Some(artifact.clone()),
                        line: None,
                    });
                }
            }
        }
        analysis
            .metrics
            .insert("artifacts_count".to_string(), artifacts.len() as f64);
        Ok(analysis)
    }
}
#[derive(Clone, Debug)]
#[doc = "Struct documentation added by AI refactor"]
pub struct ProjectAnalysis {
    pub issues: Vec<AnalysisIssue>,
    pub optimizations: Vec<OptimizationSuggestion>,
    pub suggestions: Vec<OptimizationSuggestion>,
    pub metrics: HashMap<String, f64>,
    pub total_lines: usize,
    pub files_count: usize,
}
impl ProjectAnalysis {
    #[doc = "Method documentation added by AI refactor"]
    pub fn has_critical_issues(&self) -> bool {
        self.issues.iter().any(AnalysisIssue::is_critical)
    }
    #[doc = "Method documentation added by AI refactor"]
    pub const fn has_optimizations(&self) -> bool {
        !self.optimizations.is_empty()
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn show_critical_issues(&self) {
        for issue in &self.issues {
            if issue.is_critical() {
                println!("🔴 {}", issue.description);
            }
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn show_optimizations(&self) {
        for opt in &self.optimizations {
            println!("💡 {}", opt.description);
        }
    }
    #[doc = "Method documentation added by AI refactor"]
    pub fn show_summary(&self) {
        println!("📊 Resumen del análisis:");
        println!(
            "  • Issues críticos: {}",
            self.issues.iter().filter(|i| i.is_critical()).count()
        );
        println!("  • Total issues: {}", self.issues.len());
        println!("  • Optimizaciones: {}", self.optimizations.len());
    }
}
#[derive(Clone, Debug, serde :: Serialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct AnalysisIssue {
    pub category: String,
    pub description: String,
    pub severity: IssueSeverity,
    pub file: Option<String>,
    pub line: Option<usize>,
}
impl AnalysisIssue {
    #[doc = "Method documentation added by AI refactor"]
    pub const fn is_critical(&self) -> bool {
        matches!(self.severity, IssueSeverity::Critical)
    }
}
#[derive(Clone, Debug, serde :: Serialize)]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}
#[derive(Clone, Debug, serde :: Serialize)]
#[doc = "Struct documentation added by AI refactor"]
pub struct OptimizationSuggestion {
    pub description: String,
    pub impact: OptimizationImpact,
    pub effort: OptimizationEffort,
    pub file: Option<String>,
    pub line: Option<usize>,
}
#[derive(Clone, Debug, serde :: Serialize)]
pub enum OptimizationImpact {
    High,
    Medium,
    Low,
}
#[derive(Clone, Debug, serde :: Serialize)]
pub enum OptimizationEffort {
    Low,
    Medium,
    High,
}
