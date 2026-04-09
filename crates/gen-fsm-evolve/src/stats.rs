#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerationStats {
    pub generation: usize,
    pub best_fitness: f64,
    pub avg_fitness: f64,
    pub worst_fitness: f64,
    pub fitness_std_dev: f64,
    pub evaluations: usize,
}

impl GenerationStats {
    pub fn new(
        generation: usize,
        best: f64,
        avg: f64,
        worst: f64,
        std_dev: f64,
        evaluations: usize,
    ) -> Self {
        Self {
            generation,
            best_fitness: best,
            avg_fitness: avg,
            worst_fitness: worst,
            fitness_std_dev: std_dev,
            evaluations,
        }
    }
}

impl std::fmt::Display for GenerationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Gen {:>4} │ Best: {:>8.4} │ Avg: {:>8.4} │ Worst: {:>8.4} │ σ: {:.4}",
            self.generation,
            self.best_fitness,
            self.avg_fitness,
            self.worst_fitness,
            self.fitness_std_dev,
        )
    }
}
