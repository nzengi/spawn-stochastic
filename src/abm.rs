use rand::rngs::OsRng;
use rand::Rng;

/// Arithmetic Brownian Motion (ABM) simulates the asset price over time using the formula:
/// dS = μ * dt + σ * dW
pub struct ArithmeticBrownianMotion {
    pub mu: f64,
    pub sigma: f64,
    pub n_paths: usize,
    pub n_steps: usize,
    pub t_end: f64,
    pub s_0: f64,
}

impl ArithmeticBrownianMotion {
    /// Creates a new ABM instance.
    pub fn new(mu: f64, sigma: f64, n_paths: usize, n_steps: usize, t_end: f64, s_0: f64) -> Self {
        Self { mu, sigma, n_paths, n_steps, t_end, s_0 }
    }

    /// Simulates the ABM paths using the Euler-Maruyama method with secure random number generation.
    pub fn simulate(&self) -> Vec<Vec<f64>> {
        let dt = self.t_end / self.n_steps as f64;
        let mut rng = OsRng; // Güvenli rastgele sayı üretici
        let mut paths = vec![vec![self.s_0; self.n_steps + 1]; self.n_paths];

        for i in 0..self.n_paths {
            for j in 1..=self.n_steps {
                let d_w = rng.gen::<f64>() * dt.sqrt();
                if let Some(prev) = paths[i].get(j - 1) {
                    paths[i][j] = prev + self.mu * dt + self.sigma * d_w;
                }
            }
        }

        paths
    }
}
