#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    pub const fn new(seed: u32) -> Self {
        assert!(seed != 0, "Xorshift32 seed must be non-zero");
        Self { state: seed }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> 9) as f32 / (1u32 << 23) as f32
    }

    pub fn reseed(&mut self, seed: u32) {
        assert!(seed != 0, "Xorshift32 seed must be non-zero");
        self.state = seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_sequence() {
        let mut rng1 = Xorshift32::new(12345);
        let mut rng2 = Xorshift32::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next_u32(), rng2.next_u32());
        }
    }

    #[test]
    fn f32_range() {
        let mut rng = Xorshift32::new(42);
        for _ in 0..10_000 {
            let v = rng.next_f32();
            assert!(v >= 0.0 && v < 1.0, "value out of range: {}", v);
        }
    }

    #[test]
    fn no_immediate_repeat() {
        let mut rng = Xorshift32::new(7);
        let first = rng.next_u32();
        let second = rng.next_u32();
        assert_ne!(first, second);
    }

    #[test]
    #[should_panic(expected = "non-zero")]
    fn zero_seed_panics() {
        let _ = Xorshift32::new(0);
    }
}
