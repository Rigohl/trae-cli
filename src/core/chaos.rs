//! Chaos Theory Mathematics Module for performance simulation and chaos engineering.
//! "bio nickj" mode enabled.
//! Provides models like the Lorenz attractor to generate chaotic sequences.

use std::time::Duration;

/// Generates chaotic values based on the Lorenz system.
/// Represents a simplified weather/convection model that exhibits strange attractors.
#[derive(Debug, Clone)]
pub struct LorenzChaos {
    x: f64,
    y: f64,
    z: f64,
    sigma: f64,
    rho: f64,
    beta: f64,
    dt: f64,
}

impl Default for LorenzChaos {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0, 10.0, 28.0, 8.0 / 3.0, 0.01)
    }
}

impl LorenzChaos {
    /// Creates a new Lorenz system generator with specific parameters.
    pub fn new(x: f64, y: f64, z: f64, sigma: f64, rho: f64, beta: f64, dt: f64) -> Self {
        Self {
            x,
            y,
            z,
            sigma,
            rho,
            beta,
            dt,
        }
    }

    /// Advances the system by one step (`dt`) and returns the current state (x, y, z).
    pub fn next_step(&mut self) -> (f64, f64, f64) {
        let dx = (self.sigma * (self.y - self.x)) * self.dt;
        let dy = (self.x * (self.rho - self.z) - self.y) * self.dt;
        let dz = (self.x * self.y - self.beta * self.z) * self.dt;

        self.x += dx;
        self.y += dy;
        self.z += dz;

        (self.x, self.y, self.z)
    }

    /// Calculates a non-linear chaotic factor between 0.0 and 1.0
    /// Useful for modifying latency or concurrency dynamically.
    pub fn get_chaotic_factor(&mut self) -> f64 {
        let (x, _, _) = self.next_step();
        // The x value in standard Lorenz typically ranges roughly between -20 and 20.
        // Normalize it to [0.0, 1.0].
        let normalized = (x + 20.0) / 40.0;
        normalized.clamp(0.0, 1.0)
    }
}

/// Applies chaotic delay based on the Lorenz attractor for stress testing.
pub async fn apply_chaotic_delay(
    lorenz: &mut LorenzChaos,
    base_delay: Duration,
    max_chaos_delay: Duration,
) {
    let factor = lorenz.get_chaotic_factor();
    let additional_delay_micros = (max_chaos_delay.as_micros() as f64 * factor) as u64;
    let total_delay = base_delay + Duration::from_micros(additional_delay_micros);
    tokio::time::sleep(total_delay).await;
}
