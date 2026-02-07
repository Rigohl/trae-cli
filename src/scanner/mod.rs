//! Enhanced scanner module for Jarvix CLI
//! Provides multilanguage scanning with chaos mathematics integration

use crate::chaos::{ChaosAnalyzer, FractalPattern};
use crate::i18n::{t, init_i18n, set_language};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub language: String,
    pub file_count: usize,
    pub total_lines: usize,
    pub complexity_score: f64,
    pub chaos_metrics: FractalPattern,
    pub detected_issues: Vec<ScanIssue>,
    pub entropy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanIssue {
    pub severity: IssueSeverity,
    pub message: String,
    pub file_path: String,
    pub line_number: Option<usize>,
    pub column_number: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

pub struct MultilangScanner {
    supported_languages: HashMap<String, Vec<String>>, // Extension mapping
    chaos_analyzer: ChaosAnalyzer,
}

impl MultilangScanner {
    pub fn new() -> Self {
        let mut supported_languages = HashMap::new();
        
        // Rust
        supported_languages.insert("rust".to_string(), vec![".rs".to_string()]);
        // JavaScript/TypeScript
        supported_languages.insert("javascript".to_string(), vec![".js".to_string(), ".ts".to_string(), ".jsx".to_string(), ".tsx".to_string()]);
        // Python
        supported_languages.insert("python".to_string(), vec![".py".to_string()]);
        // Go
        supported_languages.insert("go".to_string(), vec![".go".to_string()]);
        // Java
        supported_languages.insert("java".to_string(), vec![".java".to_string()]);
        // C/C++
        supported_languages.insert("cpp".to_string(), vec![".c".to_string(), ".cpp".to_string(), ".h".to_string(), ".hpp".to_string()]);
        // C#
        supported_languages.insert("csharp".to_string(), vec![".cs".to_string()]);
        // PHP
        supported_languages.insert("php".to_string(), vec![".php".to_string()]);
        // Ruby
        supported_languages.insert("ruby".to_string(), vec![".rb".to_string()]);
        // Kotlin
        supported_languages.insert("kotlin".to_string(), vec![".kt".to_string()]);
        // Swift
        supported_languages.insert("swift".to_string(), vec![".swift".to_string()]);
        // Scala
        supported_languages.insert("scala".to_string(), vec![".scala".to_string()]);
        // Shell
        supported_languages.insert("shell".to_string(), vec![".sh".to_string(), ".bash".to_string()]);
        // HTML/CSS
        supported_languages.insert("web".to_string(), vec![".html".to_string(), ".css".to_string()]);
        
        Self {
            supported_languages,
            chaos_analyzer: ChaosAnalyzer::new(0.7, 500), // High sensitivity for code analysis
        }
    }
    
    pub fn scan_project(&self, project_path: &str) -> HashMap<String, ScanResult> {
        let mut results = HashMap::new();
        
        for entry in WalkDir::new(project_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let lang = self.get_language_by_extension(ext);
                    
                    if let Some(language) = lang {
                        let content = std::fs::read_to_string(path).unwrap_or_default();
                        
                        // Skip if file is too large (more than 10MB)
                        if content.len() > 10 * 1024 * 1024 {
                            continue;
                        }
                        
                        let mut file_results = results.entry(language.clone()).or_insert_with(|| {
                            ScanResult {
                                language: language.clone(),
                                file_count: 0,
                                total_lines: 0,
                                complexity_score: 0.0,
                                chaos_metrics: self.chaos_analyzer.analyze_complexity(""),
                                detected_issues: Vec::new(),
                                entropy: 0.0,
                            }
                        });
                        
                        file_results.file_count += 1;
                        let lines = content.lines().count();
                        file_results.total_lines += lines;
                        
                        // Update chaos metrics with current file content
                        let file_pattern = self.chaos_analyzer.analyze_complexity(&content);
                        file_results.chaos_metrics = file_pattern;
                        
                        // Calculate combined complexity score
                        file_results.complexity_score = file_results.chaos_metrics.complexity;
                        
                        // Analyze entropy
                        file_results.entropy = file_results.chaos_metrics.entropy;
                        
                        // Scan for specific issues by language
                        let mut issues = self.scan_file_for_issues(path, &content, &language);
                        file_results.detected_issues.append(&mut issues);
                    }
                }
            }
        }
        
        // Normalize complexity scores based on project size
        for (_, result) in results.iter_mut() {
            if result.file_count > 0 {
                result.complexity_score = result.complexity_score / (result.file_count as f64).ln_1p();
            }
        }
        
        results
    }
    
    fn get_language_by_extension(&self, ext: &str) -> Option<String> {
        for (lang, extensions) in &self.supported_languages {
            if extensions.contains(&ext.to_string()) {
                return Some(lang.clone());
            }
        }
        None
    }
    
    fn scan_file_for_issues(&self, path: &Path, content: &str, language: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        let file_path = path.to_string_lossy().to_string();
        
        match language {
            "rust" => {
                issues.extend(self.scan_rust_issues(&file_path, content));
            },
            "javascript" => {
                issues.extend(self.scan_javascript_issues(&file_path, content));
            },
            "python" => {
                issues.extend(self.scan_python_issues(&file_path, content));
            },
            "go" => {
                issues.extend(self.scan_go_issues(&file_path, content));
            },
            "java" => {
                issues.extend(self.scan_java_issues(&file_path, content));
            },
            "cpp" => {
                issues.extend(self.scan_cpp_issues(&file_path, content));
            },
            "csharp" => {
                issues.extend(self.scan_csharp_issues(&file_path, content));
            },
            "php" => {
                issues.extend(self.scan_php_issues(&file_path, content));
            },
            "ruby" => {
                issues.extend(self.scan_ruby_issues(&file_path, content));
            },
            _ => {
                // Generic scanning for other languages
                issues.extend(self.scan_generic_issues(&file_path, content));
            }
        }
        
        issues
    }
    
    fn scan_rust_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for potentially dangerous patterns
            if line.contains("unsafe ") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Warning,
                    message: "Usage of unsafe code detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
            
            // Check for TODO comments
            if line.to_lowercase().contains("todo") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "TODO comment detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
            
            // Check for very long lines
            if line.len() > 120 {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "Line exceeds 120 characters".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_javascript_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for console.log statements
            if line.contains("console.log") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "console.log statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
            
            // Check for eval usage
            if line.contains("eval(") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Warning,
                    message: "eval() usage detected - potential security risk".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_python_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for print statements
            if line.contains("print(") && !line.trim().starts_with("#") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "print() statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
            
            // Check for eval usage
            if line.contains("eval(") && !line.trim().starts_with("#") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Warning,
                    message: "eval() usage detected - potential security risk".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_go_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for fmt.Println statements
            if line.contains("fmt.Println") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "fmt.Println statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_java_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for System.out.println statements
            if line.contains("System.out.println") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "System.out.println statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_cpp_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for cout statements
            if line.contains("std::cout") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "std::cout statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
            
            // Check for using namespace std
            if line.contains("using namespace std") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "using namespace std detected - potential naming conflicts".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_csharp_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for Console.WriteLine statements
            if line.contains("Console.WriteLine") && !line.trim().starts_with("//") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "Console.WriteLine statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_php_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for echo statements
            if line.contains("echo ") && !line.trim().starts_with("//") && !line.trim().starts_with("#") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "echo statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
            
            // Check for eval usage
            if line.contains("eval(") && !line.trim().starts_with("//") && !line.trim().starts_with("#") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Warning,
                    message: "eval() usage detected - potential security risk".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_ruby_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for puts statements
            if line.contains("puts ") && !line.trim().starts_with("#") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "puts statement detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    fn scan_generic_issues(&self, file_path: &str, content: &str) -> Vec<ScanIssue> {
        let mut issues = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Check for TODO comments
            if line.to_lowercase().contains("todo") && 
               !line.trim().starts_with("//") && 
               !line.trim().starts_with("#") &&
               !line.trim().starts_with("/*") &&
               !line.trim().starts_with("*") {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "TODO comment detected".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
            
            // Check for very long lines
            if line.len() > 120 {
                issues.push(ScanIssue {
                    severity: IssueSeverity::Info,
                    message: "Line exceeds 120 characters".to_string(),
                    file_path: file_path.to_string(),
                    line_number: Some(line_num + 1),
                    column_number: None,
                });
            }
        }
        
        issues
    }
    
    pub fn analyze_dependencies(&self, dependencies: &[String]) -> FractalPattern {
        self.chaos_analyzer.analyze_dependencies(dependencies)
    }
}

impl Default for MultilangScanner {
    fn default() -> Self {
        Self::new()
    }
}