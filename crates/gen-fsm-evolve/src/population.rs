use crate::genome::Genome;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Population<const S: usize, const C: usize> {
    individuals: Vec<Genome<S, C>>,
}

impl<const S: usize, const C: usize> Population<S, C> {
    pub fn random(size: usize, rng: &mut impl Rng) -> Self {
        let individuals = (0..size).map(|_| Genome::random(rng)).collect();
        Self { individuals }
    }

    pub fn from_individuals(individuals: Vec<Genome<S, C>>) -> Self {
        Self { individuals }
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    pub fn individuals(&self) -> &[Genome<S, C>] {
        &self.individuals
    }

    pub fn individuals_mut(&mut self) -> &mut [Genome<S, C>] {
        &mut self.individuals
    }

    pub fn sort_by_fitness(&mut self) {
        self.individuals.sort_by(|a, b| {
            let fa = a.fitness().unwrap_or(f64::NEG_INFINITY);
            let fb = b.fitness().unwrap_or(f64::NEG_INFINITY);
            fb.partial_cmp(&fa).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn best(&self) -> Option<&Genome<S, C>> {
        self.individuals.iter().max_by(|a, b| {
            let fa = a.fitness().unwrap_or(f64::NEG_INFINITY);
            let fb = b.fitness().unwrap_or(f64::NEG_INFINITY);
            fa.partial_cmp(&fb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn top_n(&self, n: usize) -> &[Genome<S, C>] {
        &self.individuals[..n.min(self.individuals.len())]
    }

    pub fn replace(&mut self, individuals: Vec<Genome<S, C>>) {
        self.individuals = individuals;
    }

    pub fn fitness_stats(&self) -> (f64, f64, f64) {
        let fitnesses: Vec<f64> = self
            .individuals
            .iter()
            .filter_map(|g| g.fitness())
            .collect();

        if fitnesses.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let best = fitnesses.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let worst = fitnesses.iter().cloned().fold(f64::INFINITY, f64::min);
        let avg = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;

        (best, avg, worst)
    }
}
