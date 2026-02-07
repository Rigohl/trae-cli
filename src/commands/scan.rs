use crate::chaos::ChaosAnalyzer;
use crate::i18n::{t, init_i18n};
use crate::scanner::MultilangScanner;
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use serde_json::json;

#[doc = " Enhanced scan command with multilanguage support and chaos mathematics"]
#[derive(Args, Debug)]
pub struct ScanCommand {
    #[doc = " Project path to scan"]
    #[arg(short, long, default_value = ".")]
    pub path: String,

    #[doc = " Enable multilanguage scanning"]
    #[arg(short, long)]
    pub multilang: bool,

    #[doc = " Enable chaos mathematics analysis"]
    #[arg(long)]
    pub chaos: bool,

    #[doc = " Language to use for output"]
    #[arg(short, long, default_value = "en")]
    pub language: String,

    #[doc = " Export results to file"]
    #[arg(short, long)]
    pub export: Option<String>,

    #[doc = " Scan type: full, quick, security"]
    #[arg(long, default_value = "full")]
    pub scan_type: String,
}

impl ScanCommand {
    #[doc = " Execute the scan command with multilanguage support and chaos mathematics"]
    pub async fn execute(&self, jarvix_cli: &crate::cli::JarvixCli) -> Result<()> {
        // Initialize internationalization
        init_i18n(&self.language);
        
        println!(
            "{}",
            "🔍 JARVIX ENHANCED SCANNER - Multilanguage Analysis with Chaos Mathematics"
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "===================================================================".cyan()
        );

        // Show scan configuration
        println!("📁 Project Path: {}", self.path);
        println!("🌍 Language: {}", self.language);
        println!("📚 Multilang: {}", if self.multilang { "YES" } else { "NO" });
        println!("⚛️  Chaos Math: {}", if self.chaos { "YES" } else { "NO" });
        println!("📈 Scan Type: {}", self.scan_type);
        if let Some(ref export_path) = self.export {
            println!("💾 Export Path: {}", export_path);
        }
        println!();

        // Create scanner
        let scanner = MultilangScanner::new();
        
        // Perform scan
        println!("{}", "Scanning project...".yellow());
        let scan_results = scanner.scan_project(&self.path);

        // Display results
        println!("\n{}", "SCAN RESULTS".green().bold());
        println!("{}", "============".green());

        let mut total_files = 0;
        let mut total_lines = 0;
        let mut total_issues = 0;

        for (lang, result) in &scan_results {
            println!("\n🌐 Language: {}", lang.to_uppercase().yellow());
            println!("   Files: {}", result.file_count);
            println!("   Lines: {}", result.total_lines);
            println!("   Complexity Score: {:.2}", result.complexity_score);
            println!("   Entropy: {:.2}", result.entropy);
            
            if self.chaos {
                println!("   Fractal Dimension: {:.2}", result.chaos_metrics.dimension);
                println!("   Chaos Complexity: {:.2}", result.chaos_metrics.complexity);
            }
            
            println!("   Issues Found: {}", result.detected_issues.len());
            
            total_files += result.file_count;
            total_lines += result.total_lines;
            total_issues += result.detected_issues.len();
            
            // Show some sample issues if any exist
            if !result.detected_issues.is_empty() {
                println!("   Sample Issues:");
                for issue in result.detected_issues.iter().take(3) {
                    let severity_color = match issue.severity {
                        crate::scanner::IssueSeverity::Critical => "🔴",
                        crate::scanner::IssueSeverity::Error => "❌",
                        crate::scanner::IssueSeverity::Warning => "🟡",
                        crate::scanner::IssueSeverity::Info => "🔵",
                    };
                    
                    if let Some(line) = issue.line_number {
                        println!("     {} {} ({}:{})", 
                                severity_color, 
                                issue.message, 
                                issue.file_path, 
                                line);
                    } else {
                        println!("     {} {} ({})", 
                                severity_color, 
                                issue.message, 
                                issue.file_path);
                    }
                }
                
                if result.detected_issues.len() > 3 {
                    println!("     ... and {} more issues", result.detected_issues.len() - 3);
                }
            }
        }

        // Summary
        println!("\n{}", "SUMMARY".green().bold());
        println!("{}", "=======".green());
        println!("Total Files Scanned: {}", total_files);
        println!("Total Lines: {}", total_lines);
        println!("Total Issues: {}", total_issues);

        // Chaos mathematics analysis if enabled
        if self.chaos {
            println!("\n{}", "CHAOS MATHEMATICS ANALYSIS".purple().bold());
            println!("{}", "===========================".purple());
            
            let chaos_analyzer = ChaosAnalyzer::new(0.7, 1000);
            
            // Analyze overall project complexity using chaos theory
            let project_complexity = chaos_analyzer.analyze_complexity(
                &format!("Project with {} files and {} lines", total_files, total_lines)
            );
            
            println!("Fractal Dimension: {:.2}", project_complexity.dimension);
            println!("Complexity Score: {:.2}", project_complexity.complexity);
            println!("Entropy: {:.2}", project_complexity.entropy);
            println!("Stability: {:.2}", 
                project_complexity.points.iter()
                    .map(|p| p.stability)
                    .sum::<f64>() / project_complexity.points.len() as f64
            );
            
            // Generate some chaos points for visualization
            let chaos_points = chaos_analyzer.lorenz_attractor(
                10.0,   // Sigma
                28.0,   // Rho
                8.0/3.0, // Beta
                (1.0, 1.0, 1.0) // Initial conditions
            );
            
            println!("Generated {} chaos points for analysis", chaos_points.len());
        }

        // Export results if requested
        if let Some(export_path) = &self.export {
            let export_data = json!({
                "scan_results": scan_results,
                "summary": {
                    "total_files": total_files,
                    "total_lines": total_lines,
                    "total_issues": total_issues,
                },
                "chaos_enabled": self.chaos,
                "chaos_analysis": if self.chaos {
                    let chaos_analyzer = ChaosAnalyzer::new(0.7, 1000);
                    let project_complexity = chaos_analyzer.analyze_complexity(
                        &format!("Project with {} files and {} lines", total_files, total_lines)
                    );
                    Some(json!({
                        "dimension": project_complexity.dimension,
                        "complexity": project_complexity.complexity,
                        "entropy": project_complexity.entropy,
                        "stability": project_complexity.points.iter()
                            .map(|p| p.stability)
                            .sum::<f64>() / project_complexity.points.len() as f64,
                        "points_count": project_complexity.points.len(),
                    }))
                } else {
                    None
                }
            });

            std::fs::write(export_path, serde_json::to_string_pretty(&export_data)?)?;
            println!("\n{}", format!("Results exported to: {}", export_path).green());
        }

        // Final status
        if total_issues == 0 {
            println!("\n{}", "✅ No issues detected!".green().bold());
        } else {
            let critical_issues: usize = scan_results.values()
                .map(|r| r.detected_issues.iter()
                    .filter(|i| matches!(i.severity, crate::scanner::IssueSeverity::Critical))
                    .count())
                .sum();
                
            if critical_issues > 0 {
                println!("\n{}", format!("⚠️  {} critical issues detected!", critical_issues).red().bold());
            } else {
                println!("\n{}", format!("🟡 {} issues detected, but none critical", total_issues).yellow());
            }
        }

        Ok(())
    }
}