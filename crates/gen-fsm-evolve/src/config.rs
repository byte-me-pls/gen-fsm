#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionConfig {
    pub population_size: usize,
    pub max_generations: usize,
    pub mutation_rate: f64,
    pub mutation_sigma: f64,
    pub crossover_rate: f64,
    pub tournament_size: usize,
    pub elite_count: usize,
    pub target_fitness: Option<f64>,
    pub seed: u64,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            population_size: 200,
            max_generations: 500,
            mutation_rate: 0.05,
            mutation_sigma: 0.1,
            crossover_rate: 0.8,
            tournament_size: 3,
            elite_count: 10,
            target_fitness: None,
            seed: 42,
        }
    }
}

impl EvolutionConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.population_size < 4 {
            return Err("population_size must be >= 4".into());
        }
        if self.elite_count >= self.population_size {
            return Err("elite_count must be < population_size".into());
        }
        if self.tournament_size < 2 || self.tournament_size > self.population_size {
            return Err("tournament_size must be in [2, population_size]".into());
        }
        if !(0.0..=1.0).contains(&self.mutation_rate) {
            return Err("mutation_rate must be in [0.0, 1.0]".into());
        }
        if !(0.0..=1.0).contains(&self.crossover_rate) {
            return Err("crossover_rate must be in [0.0, 1.0]".into());
        }
        if self.mutation_sigma <= 0.0 {
            return Err("mutation_sigma must be > 0.0".into());
        }
        Ok(())
    }
}
