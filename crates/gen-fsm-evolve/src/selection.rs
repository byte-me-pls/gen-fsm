use crate::genome::Genome;
use rand::Rng;

pub fn tournament_select<const S: usize, const C: usize>(
    population: &[Genome<S, C>],
    tournament_size: usize,
    rng: &mut impl Rng,
) -> usize {
    let mut best_idx = rng.gen_range(0..population.len());
    let mut best_fitness = population[best_idx].fitness().unwrap_or(f64::NEG_INFINITY);

    for _ in 1..tournament_size {
        let idx = rng.gen_range(0..population.len());
        let fitness = population[idx].fitness().unwrap_or(f64::NEG_INFINITY);
        if fitness > best_fitness {
            best_idx = idx;
            best_fitness = fitness;
        }
    }

    best_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tournament_selects_best() {
        let mut rng = rand::thread_rng();
        let mut pop: Vec<Genome<2, 1>> = (0..10).map(|_| Genome::random(&mut rng)).collect();

        for (i, ind) in pop.iter_mut().enumerate() {
            ind.set_fitness(if i == 3 { 100.0 } else { 0.0 });
        }

        let winner = tournament_select(&pop, pop.len(), &mut rng);
        assert_eq!(winner, 3);
    }
}
