use gen_fsm::FsmDna;
use rand::Rng;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Genome<const S: usize, const C: usize> {
    genes: Vec<f32>,
    fitness: Option<f64>,
}

impl<const S: usize, const C: usize> Genome<S, C> {
    pub const GENE_COUNT: usize = C * S * S;

    pub fn random(rng: &mut impl Rng) -> Self {
        let mut genes = Vec::with_capacity(Self::GENE_COUNT);

        for _ in 0..C {
            for _ in 0..S {
                let mut row: Vec<f32> = (0..S).map(|_| rng.gen::<f32>().max(0.01)).collect();
                let sum: f32 = row.iter().sum();
                for v in &mut row {
                    *v /= sum;
                }
                genes.extend_from_slice(&row);
            }
        }

        Self {
            genes,
            fitness: None,
        }
    }

    pub fn from_genes(genes: Vec<f32>) -> Self {
        assert_eq!(
            genes.len(),
            Self::GENE_COUNT,
            "gene count mismatch: expected {}, got {}",
            Self::GENE_COUNT,
            genes.len()
        );
        Self {
            genes,
            fitness: None,
        }
    }

    pub fn genes(&self) -> &[f32] {
        &self.genes
    }

    pub fn genes_mut(&mut self) -> &mut [f32] {
        &mut self.genes
    }

    pub fn fitness(&self) -> Option<f64> {
        self.fitness
    }

    pub fn set_fitness(&mut self, fitness: f64) {
        self.fitness = Some(fitness);
    }

    pub fn invalidate_fitness(&mut self) {
        self.fitness = None;
    }

    pub fn normalize(&mut self) {
        const EPSILON: f32 = 1e-6;

        for ctx in 0..C {
            for row in 0..S {
                let start = ctx * S * S + row * S;
                let end = start + S;

                for i in start..end {
                    if self.genes[i] < EPSILON {
                        self.genes[i] = EPSILON;
                    }
                }

                let sum: f32 = self.genes[start..end].iter().sum();
                if sum > 0.0 {
                    let inv = 1.0 / sum;
                    for i in start..end {
                        self.genes[i] *= inv;
                    }
                }
            }
        }
    }

    pub fn to_dna(&self) -> FsmDna<S, C> {
        FsmDna::<S, C>::from_flat_slice(&self.genes)
            .expect("genome gene count should always match DNA size")
    }

    pub fn from_dna(dna: &FsmDna<S, C>) -> Self {
        let genes = dna.to_flat_vec();
        Self {
            genes,
            fitness: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_genome_is_normalized() {
        let mut rng = rand::thread_rng();
        let genome = Genome::<4, 3>::random(&mut rng);

        assert_eq!(genome.genes().len(), Genome::<4, 3>::GENE_COUNT);

        for ctx in 0..3 {
            for row in 0..4 {
                let start = ctx * 16 + row * 4;
                let sum: f32 = genome.genes()[start..start + 4].iter().sum();
                assert!((sum - 1.0).abs() < 1e-4, "row sum = {}", sum);
            }
        }
    }

    #[test]
    fn to_dna_roundtrip() {
        let mut rng = rand::thread_rng();
        let genome = Genome::<3, 2>::random(&mut rng);
        let dna = genome.to_dna();
        let genome2 = Genome::<3, 2>::from_dna(&dna);

        assert_eq!(genome.genes().len(), genome2.genes().len());
        for (a, b) in genome.genes().iter().zip(genome2.genes().iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }
}
