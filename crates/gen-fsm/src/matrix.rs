#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransitionMatrix<const S: usize> {
    data: [[f32; S]; S],
}

impl<const S: usize> TransitionMatrix<S> {
    pub fn uniform() -> Self {
        let prob = 1.0 / S as f32;
        Self {
            data: [[prob; S]; S],
        }
    }

    pub fn from_array(data: [[f32; S]; S]) -> Self {
        Self { data }
    }

    pub fn from_slice(slice: &[f32]) -> Option<Self> {
        if slice.len() != S * S {
            return None;
        }
        let mut data = [[0.0f32; S]; S];
        for row in 0..S {
            for col in 0..S {
                data[row][col] = slice[row * S + col];
            }
        }
        Some(Self { data })
    }

    #[inline]
    pub fn probability(&self, from: usize, to: usize) -> f32 {
        self.data[from][to]
    }

    #[inline]
    pub fn set_probability(&mut self, from: usize, to: usize, value: f32) {
        self.data[from][to] = value;
    }

    #[inline]
    pub fn next_state(&self, current: usize, random_value: f32) -> usize {
        let row = &self.data[current];
        let mut cumulative = 0.0f32;

        for (i, &prob) in row.iter().enumerate() {
            cumulative += prob;
            if random_value < cumulative {
                return i;
            }
        }

        S - 1
    }

    pub fn normalize(&mut self) {
        const EPSILON: f32 = 1e-6;

        for row in &mut self.data {
            for val in row.iter_mut() {
                if *val < EPSILON {
                    *val = EPSILON;
                }
            }

            let sum: f32 = row.iter().sum();
            if sum > 0.0 {
                let inv_sum = 1.0 / sum;
                for val in row.iter_mut() {
                    *val *= inv_sum;
                }
            }
        }
    }

    pub fn write_to_slice(&self, buf: &mut [f32]) -> Option<usize> {
        let total = S * S;
        if buf.len() < total {
            return None;
        }
        for row in 0..S {
            for col in 0..S {
                buf[row * S + col] = self.data[row][col];
            }
        }
        Some(total)
    }

    pub const fn gene_count() -> usize {
        S * S
    }

    pub fn as_array(&self) -> &[[f32; S]; S] {
        &self.data
    }

    pub fn as_array_mut(&mut self) -> &mut [[f32; S]; S] {
        &mut self.data
    }

    pub fn is_valid(&self, tolerance: f32) -> bool {
        for row in &self.data {
            let mut sum = 0.0f32;
            for &val in row {
                if val < 0.0 {
                    return false;
                }
                sum += val;
            }
            if (sum - 1.0).abs() > tolerance {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_matrix_is_valid() {
        let m = TransitionMatrix::<4>::uniform();
        assert!(m.is_valid(1e-5));

        for from in 0..4 {
            for to in 0..4 {
                assert!((m.probability(from, to) - 0.25).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn next_state_deterministic_edge() {
        let mut data = [[0.0f32; 4]; 4];
        data[0][2] = 1.0;
        data[1] = [0.25; 4];
        data[2][2] = 1.0;
        data[3][3] = 1.0;

        let m = TransitionMatrix::<4>::from_array(data);

        assert_eq!(m.next_state(0, 0.0), 2);
        assert_eq!(m.next_state(0, 0.5), 2);
        assert_eq!(m.next_state(0, 0.99), 2);
    }

    #[test]
    fn normalize_clamps_negatives() {
        let data = [[-1.0, 0.0], [3.0, 1.0]];
        let mut m = TransitionMatrix::<2>::from_array(data);
        m.normalize();
        assert!(m.is_valid(1e-4));
    }

    #[test]
    fn from_slice_correct_length() {
        let val = 1.0 / 3.0f32;
        let data = vec![val; 9];
        let m = TransitionMatrix::<3>::from_slice(&data);
        assert!(m.is_some());
        assert!(m.unwrap().is_valid(1e-5));
    }

    #[test]
    fn from_slice_wrong_length() {
        let data = vec![0.25f32; 8];
        let m = TransitionMatrix::<3>::from_slice(&data);
        assert!(m.is_none());
    }

    #[test]
    fn write_to_slice_roundtrip() {
        let m = TransitionMatrix::<3>::uniform();
        let mut buf = [0.0f32; 9];
        let written = m.write_to_slice(&mut buf);
        assert_eq!(written, Some(9));

        let m2 = TransitionMatrix::<3>::from_slice(&buf).unwrap();
        for r in 0..3 {
            for c in 0..3 {
                assert!((m.probability(r, c) - m2.probability(r, c)).abs() < 1e-6);
            }
        }
    }
}
