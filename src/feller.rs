use rand::rngs::OsRng;
use rand::Rng;

/// Feller Square Root Process simulates asset prices using:
/// dS = α * (μ - S) * dt + σ * sqrt(S) * dW
pub struct FellerSquareRoot {
    pub alpha: f64,
    pub mu: f64,
    pub sigma: f64,
    pub n_paths: usize,
    pub n_steps: usize,
    pub t_end: f64,
    pub s_0: f64,
}

impl FellerSquareRoot {
    /// Creates a new Feller Square Root instance.
    pub fn new(alpha: f64, mu: f64, sigma: f64, n_paths: usize, n_steps: usize, t_end: f64, s_0: f64) -> Self {
        Self { alpha, mu, sigma, n_paths, n_steps, t_end, s_0 }
    }

    /// Simulates the Feller Square Root process paths using the Euler-Maruyama method with secure random number generation.
    pub fn simulate(&self) -> Vec<Vec<f64>> {
        let dt = self.t_end / self.n_steps as f64;
        let mut rng = OsRng;
        let mut paths = vec![vec![self.s_0; self.n_steps + 1]; self.n_paths];

        for i in 0..self.n_paths {
            for j in 1..=self.n_steps {
                let d_w = rng.gen::<f64>() * dt.sqrt();
                if let Some(prev) = paths[i].get(j - 1) {
                    let sqrt_s = if *prev >= 0.0 { prev.sqrt() } else { 0.0 }; // Negatif sayı kontrolü
                    paths[i][j] = prev + self.alpha * (self.mu - prev) * dt + self.sigma * sqrt_s * d_w;
                }
            }
        }

        paths
    }
}
