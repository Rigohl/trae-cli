//! Chaos Mathematics module for Jarvix CLI
//! Implements chaos theory and fractal mathematics for code analysis

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub iteration: usize,
    pub stability: f64, // Value between 0.0 and 1.0 representing stability
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractalPattern {
    pub name: String,
    pub dimension: f64, // Fractal dimension
    pub complexity: f64,
    pub entropy: f64,
    pub points: Vec<ChaosPoint>,
}

pub struct ChaosAnalyzer {
    patterns: HashMap<String, FractalPattern>,
    sensitivity: f64, // Sensitivity to initial conditions (0.0 to 1.0)
    iterations: usize,
}

impl ChaosAnalyzer {
    pub fn new(sensitivity: f64, iterations: usize) -> Self {
        Self {
            patterns: HashMap::new(),
            sensitivity: sensitivity.min(1.0).max(0.0),
            iterations,
        }
    }
    
    /// Generate Lorenz attractor points for code analysis
    pub fn lorenz_attractor(&self, sigma: f64, rho: f64, beta: f64, initial: (f64, f64, f64)) -> Vec<ChaosPoint> {
        let mut points = Vec::new();
        let mut x = initial.0;
        let mut y = initial.1;
        let mut z = initial.2;
        
        for i in 0..self.iterations {
            let dx = sigma * (y - x);
            let dy = x * (rho - z) - y;
            let dz = x * y - beta * z;
            
            x += dx * 0.01; // Time step
            y += dy * 0.01;
            z += dz * 0.01;
            
            // Calculate stability as distance from origin normalized
            let stability = 1.0 / (1.0 + (x*x + y*y + z*z).sqrt());
            
            points.push(ChaosPoint {
                x,
                y,
                z,
                iteration: i,
                stability,
            });
        }
        
        points
    }
    
    /// Generate Mandelbrot set approximation for code structure analysis
    pub fn mandelbrot_set(&self, width: usize, height: usize, max_iterations: usize) -> Vec<Vec<f64>> {
        let mut result = vec![vec![0.0; width]; height];
        
        for y in 0..height {
            for x in 0..width {
                let cx = (x as f64 - width as f64 / 2.0) * 4.0 / width as f64;
                let cy = (y as f64 - height as f64 / 2.0) * 4.0 / height as f64;
                
                let mut zx = 0.0;
                let mut zy = 0.0;
                let mut iter = 0;
                
                while iter < max_iterations {
                    let zx_new = zx * zx - zy * zy + cx;
                    zy = 2.0 * zx * zy + cy;
                    zx = zx_new;
                    
                    if zx * zx + zy * zy > 4.0 {
                        break;
                    }
                    iter += 1;
                }
                
                result[y][x] = iter as f64 / max_iterations as f64;
            }
        }
        
        result
    }
    
    /// Analyze code complexity using chaos mathematics
    pub fn analyze_complexity(&self, code: &str) -> FractalPattern {
        // Calculate various metrics based on chaos theory
        let entropy = self.calculate_entropy(code);
        let complexity = self.estimate_fractal_dimension(code);
        let dimension = self.calculate_lyapunov_exponent(code);
        
        // Generate representative chaos points
        let points = self.lorenz_attractor(
            10.0, // Sigma
            28.0, // Rho
            8.0 / 3.0, // Beta
            (1.0, 1.0, 1.0), // Initial conditions
        );
        
        FractalPattern {
            name: "Code Complexity Attractor".to_string(),
            dimension,
            complexity,
            entropy,
            points,
        }
    }
    
    /// Calculate Shannon entropy of the code
    fn calculate_entropy(&self, code: &str) -> f64 {
        let mut char_counts = HashMap::new();
        
        for ch in code.chars() {
            *char_counts.entry(ch).or_insert(0) += 1;
        }
        
        let total_chars = code.len() as f64;
        let mut entropy = 0.0;
        
        for count in char_counts.values() {
            let probability = *count as f64 / total_chars;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
    
    /// Estimate fractal dimension based on code structure
    fn estimate_fractal_dimension(&self, code: &str) -> f64 {
        // A simplified approach: calculate the ratio of unique tokens to total tokens
        let tokens: Vec<&str> = code.split_whitespace().collect();
        let unique_tokens: std::collections::HashSet<&str> = tokens.iter().cloned().collect();
        
        let token_diversity = unique_tokens.len() as f64 / tokens.len() as f64;
        
        // Apply logarithmic scaling to represent fractal properties
        1.0 + token_diversity.ln_1p()
    }
    
    /// Calculate Lyapunov exponent approximation for code
    fn calculate_lyapunov_exponent(&self, code: &str) -> f64 {
        // Simplified approach: measure how sensitive the code structure is to small changes
        let original_hash = calculate_simple_hash(code);
        
        // Make small modifications and measure differences
        let mut deviations = Vec::new();
        
        for i in 0..std::cmp::min(10, code.len()) {
            let mut modified_code = code.to_string();
            if let Some(ch) = modified_code.chars().nth(i) {
                let new_char = match ch {
                    'a'..='z' => ((ch as u8 + 1) % 26 + b'a') as char,
                    'A'..='Z' => ((ch as u8 + 1) % 26 + b'A') as char,
                    _ => ch,
                };
                modified_code.replace_range(i..i+1, &new_char.to_string());
                
                let modified_hash = calculate_simple_hash(&modified_code);
                let deviation = (original_hash as i64 - modified_hash as i64).abs() as f64;
                deviations.push(deviation);
            }
        }
        
        if !deviations.is_empty() {
            deviations.iter().sum::<f64>() / deviations.len() as f64
        } else {
            0.0
        }
    }
    
    /// Analyze dependency networks using chaos theory
    pub fn analyze_dependencies(&self, dependencies: &[String]) -> FractalPattern {
        // Create a network representation of dependencies
        let size = dependencies.len();
        if size == 0 {
            return FractalPattern {
                name: "Empty Dependencies".to_string(),
                dimension: 0.0,
                complexity: 0.0,
                entropy: 0.0,
                points: vec![],
            };
        }
        
        // Calculate connection density and complexity
        let mut connections = 0;
        for i in 0..size {
            for j in (i + 1)..size {
                // In a real implementation, this would check actual dependency relationships
                // For now, we'll simulate based on name similarity
                if dependencies[i].chars().zip(dependencies[j].chars())
                    .filter(|(a, b)| a == b)
                    .count() > 2
                {
                    connections += 1;
                }
            }
        }
        
        let density = (connections as f64) / ((size * (size - 1) / 2) as f64);
        let complexity = density * size as f64;
        
        // Generate chaos points representing the network
        let points = self.lorenz_attractor(
            12.0, // Adjusted sigma for dependency network
            30.0, // Adjusted rho for dependency network
            2.5,  // Adjusted beta for dependency network
            (0.5, 0.5, 0.5), // Starting point for dependency analysis
        );
        
        FractalPattern {
            name: "Dependency Network Attractor".to_string(),
            dimension: 1.0 + density.ln_1p(),
            complexity,
            entropy: self.calculate_dependency_entropy(dependencies),
            points,
        }
    }
    
    /// Calculate entropy of dependency relationships
    fn calculate_dependency_entropy(&self, dependencies: &[String]) -> f64 {
        if dependencies.is_empty() {
            return 0.0;
        }
        
        let mut counts = HashMap::new();
        for dep in dependencies {
            *counts.entry(dep).or_insert(0) += 1;
        }
        
        let total = dependencies.len() as f64;
        let mut entropy = 0.0;
        
        for count in counts.values() {
            let p = *count as f64 / total;
            if p > 0.0 {
                entropy -= p * p.ln();
            }
        }
        
        entropy
    }
}

/// Simple hash function for demonstration purposes
fn calculate_simple_hash(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaos_analyzer_creation() {
        let analyzer = ChaosAnalyzer::new(0.5, 100);
        assert_eq!(analyzer.sensitivity, 0.5);
        assert_eq!(analyzer.iterations, 100);
    }

    #[test]
    fn test_lorenz_attractor_generation() {
        let analyzer = ChaosAnalyzer::new(0.5, 10);
        let points = analyzer.lorenz_attractor(10.0, 28.0, 8.0/3.0, (1.0, 1.0, 1.0));
        assert_eq!(points.len(), 10);
    }

    #[test]
    fn test_entropy_calculation() {
        let analyzer = ChaosAnalyzer::new(0.5, 100);
        let entropy = analyzer.calculate_entropy("hello world");
        assert!(entropy >= 0.0);
    }
}