use crate::genome::Genome;
use rand::Rng;

pub fn gaussian_mutate<const S: usize, const C: usize>(
    genome: &mut Genome<S, C>,
    mutation_rate: f64,
    sigma: f64,
    rng: &mut impl Rng,
) {
    let mut mutated = false;

    let genes = genome.genes_mut();

    for gene in genes.iter_mut() {
        if rng.gen::<f64>() < mutation_rate {
            let u1: f64 = rng.gen::<f64>().max(1e-10);
            let u2: f64 = rng.gen::<f64>();
            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            let perturbation = (z * sigma) as f32;
            *gene += perturbation;
            if *gene < 0.0 {
                *gene = 1e-6;
            }
            mutated = true;
        }
    }

    if mutated {
        genome.normalize();
        genome.invalidate_fitness();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutation_maintains_normalization() {
        let mut rng = rand::thread_rng();
        let mut genome = Genome::<4, 3>::random(&mut rng);

        gaussian_mutate(&mut genome, 1.0, 0.5, &mut rng);

        for ctx in 0..3 {
            for row in 0..4 {
                let start = ctx * 16 + row * 4;
                let sum: f32 = genome.genes()[start..start + 4].iter().sum();
                assert!(
                    (sum - 1.0).abs() < 1e-4,
                    "row [{},{}] sum = {}",
                    ctx,
                    row,
                    sum
                );
            }
        }
    }

    #[test]
    fn zero_mutation_rate_no_change() {
        let mut rng = rand::thread_rng();
        let original = Genome::<3, 2>::random(&mut rng);
        let mut mutated = original.clone();

        gaussian_mutate(&mut mutated, 0.0, 0.1, &mut rng);

        for (a, b) in original.genes().iter().zip(mutated.genes().iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }
}
