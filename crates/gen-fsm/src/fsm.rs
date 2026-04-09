use crate::context::Context;
use crate::dna::FsmDna;
use crate::rng::Xorshift32;
use crate::state::State;
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct StochasticFsm<S: State, C: Context, const NS: usize, const NC: usize> {
    dna: FsmDna<NS, NC>,
    current_state: S,
    rng: Xorshift32,
    step_count: u32,
    _context: PhantomData<C>,
}

impl<S, C, const NS: usize, const NC: usize> StochasticFsm<S, C, NS, NC>
where
    S: State,
    C: Context,
{
    pub fn new(dna: FsmDna<NS, NC>, initial_state: S, rng_seed: u32) -> Self {
        assert_eq!(NS, S::COUNT, "NS must equal State::COUNT");
        assert_eq!(NC, C::COUNT, "NC must equal Context::COUNT");

        Self {
            dna,
            current_state: initial_state,
            rng: Xorshift32::new(rng_seed),
            step_count: 0,
            _context: PhantomData,
        }
    }

    #[inline]
    pub fn current_state(&self) -> S {
        self.current_state
    }

    #[inline]
    pub fn step_count(&self) -> u32 {
        self.step_count
    }

    #[inline]
    pub fn step(&mut self, context: C) -> S {
        let ctx_idx = context.to_index();
        let state_idx = self.current_state.to_index();
        let random = self.rng.next_f32();

        let next_idx = self.dna.matrix(ctx_idx).next_state(state_idx, random);

        self.current_state = S::from_index(next_idx).unwrap_or(self.current_state);
        self.step_count = self.step_count.wrapping_add(1);
        self.current_state
    }

    #[inline]
    pub fn force_transition(&mut self, state: S) {
        self.current_state = state;
    }

    pub fn reset(&mut self, state: S, rng_seed: u32) {
        self.current_state = state;
        self.step_count = 0;
        self.rng = Xorshift32::new(rng_seed);
    }

    pub fn dna(&self) -> &FsmDna<NS, NC> {
        &self.dna
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestState {
        Idle,
        Active,
        Done,
    }

    impl State for TestState {
        const COUNT: usize = 3;
        fn to_index(&self) -> usize {
            *self as usize
        }
        fn from_index(index: usize) -> Option<Self> {
            match index {
                0 => Some(Self::Idle),
                1 => Some(Self::Active),
                2 => Some(Self::Done),
                _ => None,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestCtx {
        Normal,
        Alert,
    }

    impl Context for TestCtx {
        const COUNT: usize = 2;
        fn to_index(&self) -> usize {
            *self as usize
        }
        fn from_index(index: usize) -> Option<Self> {
            match index {
                0 => Some(Self::Normal),
                1 => Some(Self::Alert),
                _ => None,
            }
        }
    }

    #[test]
    fn fsm_starts_in_initial_state() {
        let dna = FsmDna::<3, 2>::uniform();
        let fsm = StochasticFsm::<TestState, TestCtx, 3, 2>::new(dna, TestState::Idle, 1);
        assert_eq!(fsm.current_state(), TestState::Idle);
        assert_eq!(fsm.step_count(), 0);
    }

    #[test]
    fn fsm_step_changes_state() {
        let dna = FsmDna::<3, 2>::uniform();
        let mut fsm = StochasticFsm::<TestState, TestCtx, 3, 2>::new(dna, TestState::Idle, 42);

        let mut saw_different = false;
        for _ in 0..100 {
            let state = fsm.step(TestCtx::Normal);
            if state != TestState::Idle {
                saw_different = true;
                break;
            }
        }
        assert!(saw_different, "FSM never left initial state after 100 uniform steps");
    }

    #[test]
    fn fsm_deterministic_with_same_seed() {
        let dna = FsmDna::<3, 2>::uniform();
        let mut fsm1 = StochasticFsm::<TestState, TestCtx, 3, 2>::new(dna.clone(), TestState::Idle, 99);
        let mut fsm2 = StochasticFsm::<TestState, TestCtx, 3, 2>::new(dna, TestState::Idle, 99);

        for _ in 0..50 {
            let s1 = fsm1.step(TestCtx::Alert);
            let s2 = fsm2.step(TestCtx::Alert);
            assert_eq!(s1, s2, "Same seed should produce identical sequences");
        }
    }

    #[test]
    fn force_transition_works() {
        let dna = FsmDna::<3, 2>::uniform();
        let mut fsm = StochasticFsm::<TestState, TestCtx, 3, 2>::new(dna, TestState::Idle, 1);
        fsm.force_transition(TestState::Done);
        assert_eq!(fsm.current_state(), TestState::Done);
    }

    #[test]
    fn reset_clears_state() {
        let dna = FsmDna::<3, 2>::uniform();
        let mut fsm = StochasticFsm::<TestState, TestCtx, 3, 2>::new(dna, TestState::Idle, 1);

        for _ in 0..10 {
            fsm.step(TestCtx::Normal);
        }
        assert!(fsm.step_count() > 0);

        fsm.reset(TestState::Active, 1);
        assert_eq!(fsm.current_state(), TestState::Active);
        assert_eq!(fsm.step_count(), 0);
    }
}
