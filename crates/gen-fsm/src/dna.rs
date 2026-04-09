use crate::matrix::TransitionMatrix;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FsmDna<const S: usize, const C: usize> {
    matrices: [TransitionMatrix<S>; C],
}

impl<const S: usize, const C: usize> FsmDna<S, C> {
    pub const fn total_genes() -> usize {
        C * S * S
    }

    pub fn from_matrices(matrices: [TransitionMatrix<S>; C]) -> Self {
        Self { matrices }
    }

    pub fn from_flat_slice(data: &[f32]) -> Option<Self> {
        if data.len() != Self::total_genes() {
            return None;
        }

        let matrices: [TransitionMatrix<S>; C] = core::array::from_fn(|i| {
            let start = i * S * S;
            let end = start + S * S;
            TransitionMatrix::<S>::from_slice(&data[start..end])
                .expect("slice bounds verified above")
        });

        Some(Self { matrices })
    }

    pub fn uniform() -> Self {
        Self {
            matrices: core::array::from_fn(|_| TransitionMatrix::<S>::uniform()),
        }
    }

    #[inline]
    pub fn matrix(&self, context_index: usize) -> &TransitionMatrix<S> {
        &self.matrices[context_index]
    }

    #[inline]
    pub fn matrix_mut(&mut self, context_index: usize) -> &mut TransitionMatrix<S> {
        &mut self.matrices[context_index]
    }

    pub fn matrices(&self) -> &[TransitionMatrix<S>; C] {
        &self.matrices
    }

    pub fn normalize(&mut self) {
        for matrix in &mut self.matrices {
            matrix.normalize();
        }
    }

    #[cfg(any(feature = "std", test))]
    pub fn to_flat_vec(&self) -> Vec<f32> {
        let mut out = Vec::with_capacity(Self::total_genes());
        let mut buf = [0.0f32; 256];
        for m in &self.matrices {
            let count = m.write_to_slice(&mut buf).expect("buffer large enough");
            out.extend_from_slice(&buf[..count]);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_dna_valid() {
        let dna = FsmDna::<4, 3>::uniform();
        for i in 0..3 {
            assert!(dna.matrix(i).is_valid(1e-5));
        }
    }

    #[test]
    fn round_trip_flat() {
        let dna = FsmDna::<3, 2>::uniform();
        let flat = dna.to_flat_vec();
        assert_eq!(flat.len(), FsmDna::<3, 2>::total_genes());

        let dna2 = FsmDna::<3, 2>::from_flat_slice(&flat).unwrap();
        for i in 0..2 {
            for from in 0..3 {
                for to in 0..3 {
                    let diff = (dna.matrix(i).probability(from, to)
                        - dna2.matrix(i).probability(from, to))
                    .abs();
                    assert!(diff < 1e-6);
                }
            }
        }
    }

    #[test]
    fn from_flat_slice_wrong_length() {
        let data = vec![0.0f32; 10];
        assert!(FsmDna::<3, 2>::from_flat_slice(&data).is_none());
    }
}
