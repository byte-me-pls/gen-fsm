use crate::genome::Genome;
use rand::Rng;

pub fn uniform_crossover<const S: usize, const C: usize>(
    parent_a: &Genome<S, C>,
    parent_b: &Genome<S, C>,
    rng: &mut impl Rng,
) -> (Genome<S, C>, Genome<S, C>) {
    let genes_a = parent_a.genes();
    let genes_b = parent_b.genes();
    let len = genes_a.len();

    let mut child_a_genes = Vec::with_capacity(len);
    let mut child_b_genes = Vec::with_capacity(len);

    for i in 0..len {
        if rng.gen_bool(0.5) {
            child_a_genes.push(genes_a[i]);
            child_b_genes.push(genes_b[i]);
        } else {
            child_a_genes.push(genes_b[i]);
            child_b_genes.push(genes_a[i]);
        }
    }

    let mut child_a = Genome::from_genes(child_a_genes);
    let mut child_b = Genome::from_genes(child_b_genes);

    child_a.normalize();
    child_b.normalize();

    (child_a, child_b)
}

pub fn single_point_crossover<const S: usize, const C: usize>(
    parent_a: &Genome<S, C>,
    parent_b: &Genome<S, C>,
    rng: &mut impl Rng,
) -> (Genome<S, C>, Genome<S, C>) {
    let genes_a = parent_a.genes();
    let genes_b = parent_b.genes();
    let len = genes_a.len();

    let cut = rng.gen_range(1..len);

    let mut child_a_genes = Vec::with_capacity(len);
    let mut child_b_genes = Vec::with_capacity(len);

    child_a_genes.extend_from_slice(&genes_a[..cut]);
    child_a_genes.extend_from_slice(&genes_b[cut..]);

    child_b_genes.extend_from_slice(&genes_b[..cut]);
    child_b_genes.extend_from_slice(&genes_a[cut..]);

    let mut child_a = Genome::from_genes(child_a_genes);
    let mut child_b = Genome::from_genes(child_b_genes);

    child_a.normalize();
    child_b.normalize();

    (child_a, child_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_crossover_produces_valid_children() {
        let mut rng = rand::thread_rng();
        let a = Genome::<3, 2>::random(&mut rng);
        let b = Genome::<3, 2>::random(&mut rng);

        let (c1, c2) = uniform_crossover(&a, &b, &mut rng);

        assert_eq!(c1.genes().len(), Genome::<3, 2>::GENE_COUNT);
        assert_eq!(c2.genes().len(), Genome::<3, 2>::GENE_COUNT);

        for ctx in 0..2 {
            for row in 0..3 {
                let start = ctx * 9 + row * 3;
                let sum1: f32 = c1.genes()[start..start + 3].iter().sum();
                let sum2: f32 = c2.genes()[start..start + 3].iter().sum();
                assert!((sum1 - 1.0).abs() < 1e-4);
                assert!((sum2 - 1.0).abs() < 1e-4);
            }
        }
    }
}
