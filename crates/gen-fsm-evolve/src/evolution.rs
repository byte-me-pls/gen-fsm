use crate::config::EvolutionConfig;
use crate::crossover::uniform_crossover;
use crate::fitness::FitnessEvaluator;
use crate::genome::Genome;
use crate::mutation::gaussian_mutate;
use crate::population::Population;
use crate::selection::tournament_select;
use crate::stats::GenerationStats;
use gen_fsm::FsmDna;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct EvolutionResult<const S: usize, const C: usize> {
    pub best_genome: Genome<S, C>,
    pub best_fitness: f64,
    pub history: Vec<GenerationStats>,
    pub generations_run: usize,
    pub target_reached: bool,
}

impl<const S: usize, const C: usize> EvolutionResult<S, C> {
    pub fn best_dna(&self) -> FsmDna<S, C> {
        self.best_genome.to_dna()
    }
}

pub struct EvolutionEngine<const S: usize, const C: usize, F>
where
    F: FitnessEvaluator<S, C>,
{
    config: EvolutionConfig,
    evaluator: F,
}

impl<const S: usize, const C: usize, F> EvolutionEngine<S, C, F>
where
    F: FitnessEvaluator<S, C>,
{
    pub fn new(config: EvolutionConfig, evaluator: F) -> Self {
        config.validate().expect("invalid evolution config");
        Self { config, evaluator }
    }

    pub fn run<CB>(&self, mut callback: CB) -> EvolutionResult<S, C>
    where
        CB: FnMut(&GenerationStats),
    {
        let mut rng = StdRng::seed_from_u64(self.config.seed);
        let mut population = Population::<S, C>::random(self.config.population_size, &mut rng);
        let mut history = Vec::with_capacity(self.config.max_generations);

        let mut overall_best: Option<Genome<S, C>> = None;
        let mut overall_best_fitness = f64::NEG_INFINITY;
        let mut target_reached = false;

        for gen in 0..self.config.max_generations {
            let evaluations = self.evaluate_population(&mut population);

            population.sort_by_fitness();

            let (best, avg, worst) = population.fitness_stats();
            let std_dev = self.compute_std_dev(&population, avg);
            let stats = GenerationStats::new(gen, best, avg, worst, std_dev, evaluations);
            callback(&stats);
            history.push(stats);

            if let Some(current_best) = population.best() {
                let bf = current_best.fitness().unwrap_or(f64::NEG_INFINITY);
                if bf > overall_best_fitness {
                    overall_best_fitness = bf;
                    overall_best = Some(current_best.clone());
                }
            }

            if let Some(target) = self.config.target_fitness {
                if best >= target {
                    target_reached = true;
                    break;
                }
            }

            let mut next_gen = Vec::with_capacity(self.config.population_size);

            for elite in population.top_n(self.config.elite_count) {
                next_gen.push(elite.clone());
            }

            while next_gen.len() < self.config.population_size {
                let parent_a_idx = tournament_select(
                    population.individuals(),
                    self.config.tournament_size,
                    &mut rng,
                );
                let parent_b_idx = tournament_select(
                    population.individuals(),
                    self.config.tournament_size,
                    &mut rng,
                );

                let parent_a = &population.individuals()[parent_a_idx];
                let parent_b = &population.individuals()[parent_b_idx];

                let (mut child_a, mut child_b) = if rng.gen::<f64>() < self.config.crossover_rate {
                    uniform_crossover(parent_a, parent_b, &mut rng)
                } else {
                    (parent_a.clone(), parent_b.clone())
                };

                gaussian_mutate(
                    &mut child_a,
                    self.config.mutation_rate,
                    self.config.mutation_sigma,
                    &mut rng,
                );
                gaussian_mutate(
                    &mut child_b,
                    self.config.mutation_rate,
                    self.config.mutation_sigma,
                    &mut rng,
                );

                next_gen.push(child_a);
                if next_gen.len() < self.config.population_size {
                    next_gen.push(child_b);
                }
            }

            population.replace(next_gen);
        }

        let generations_run = history.len();

        EvolutionResult {
            best_genome: overall_best.unwrap_or_else(|| {
                population.best().cloned().unwrap_or_else(|| Genome::random(&mut rng))
            }),
            best_fitness: overall_best_fitness,
            history,
            generations_run,
            target_reached,
        }
    }

    fn evaluate_population(&self, population: &mut Population<S, C>) -> usize {
        let needs_eval: Vec<usize> = population
            .individuals()
            .iter()
            .enumerate()
            .filter(|(_, g)| g.fitness().is_none())
            .map(|(i, _)| i)
            .collect();

        let count = needs_eval.len();

        let results: Vec<(usize, f64)> = needs_eval
            .par_iter()
            .map(|&idx| {
                let dna = population.individuals()[idx].to_dna();
                let fitness = self.evaluator.evaluate(&dna);
                (idx, fitness)
            })
            .collect();

        for (idx, fitness) in results {
            population.individuals_mut()[idx].set_fitness(fitness);
        }

        count
    }

    fn compute_std_dev(&self, population: &Population<S, C>, mean: f64) -> f64 {
        let fitnesses: Vec<f64> = population
            .individuals()
            .iter()
            .filter_map(|g| g.fitness())
            .collect();

        if fitnesses.len() < 2 {
            return 0.0;
        }

        let variance =
            fitnesses.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / fitnesses.len() as f64;

        variance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fitness::FnEvaluator;

    #[test]
    fn evolution_improves_fitness() {
        let evaluator = FnEvaluator::new(|dna: &FsmDna<2, 1>| {
            let p = dna.matrix(0).probability(0, 0) as f64;
            p
        });

        let config = EvolutionConfig {
            population_size: 50,
            max_generations: 50,
            mutation_rate: 0.1,
            mutation_sigma: 0.15,
            crossover_rate: 0.7,
            tournament_size: 3,
            elite_count: 5,
            target_fitness: None,
            seed: 42,
        };

        let engine = EvolutionEngine::<2, 1, _>::new(config, evaluator);
        let result = engine.run(|_| {});

        let best = result.best_dna();
        let p00 = best.matrix(0).probability(0, 0) as f64;
        assert!(p00 > 0.7, "Expected p(0→0) > 0.7, got {p00:.4}");
    }
}
