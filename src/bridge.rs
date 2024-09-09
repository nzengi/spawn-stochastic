use rand::rngs::OsRng;
use rand::Rng;

/// Brownian Bridge process simulates paths that start and end at fixed points.
pub struct BrownianBridge {
    pub alpha: f64,
    pub beta: f64,
    pub sigma: f64,
    pub n_paths: usize,
    pub n_steps: usize,
    pub t_end: f64,
}

impl BrownianBridge {
    /// Creates a new Brownian Bridge instance.
    pub fn new(alpha: f64, beta: f64, sigma: f64, n_paths: usize, n_steps: usize, t_end: f64) -> Self {
        Self { alpha, beta, sigma, n_paths, n_steps, t_end }
    }

    /// Simulates Brownian Bridge paths using the Euler-Maruyama method with secure random number generation.
    pub fn simulate(&self) -> Vec<Vec<f64>> {
        let dt = self.t_end / self.n_steps as f64;
        let mut rng = OsRng;
        let mut paths = vec![vec![self.alpha; self.n_steps + 1]; self.n_paths];

        for i in 0..self.n_paths {
            for j in 1..self.n_steps {
                let d_w = rng.gen::<f64>() * dt.sqrt();
                if let Some(prev) = paths[i].get(j - 1) {
                    paths[i][j] = prev + (self.beta - prev) / (self.n_steps - j + 1) as f64 + self.sigma * d_w;
                }
            }
            paths[i][self.n_steps] = self.beta; // Set final value to beta
        }

        paths
    }
}
