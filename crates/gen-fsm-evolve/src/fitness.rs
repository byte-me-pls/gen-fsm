use gen_fsm::FsmDna;

pub trait FitnessEvaluator<const S: usize, const C: usize>: Send + Sync {
    fn evaluate(&self, dna: &FsmDna<S, C>) -> f64;
}

pub struct FnEvaluator<F, const S: usize, const C: usize>
where
    F: Fn(&FsmDna<S, C>) -> f64 + Send + Sync,
{
    func: F,
}

impl<F, const S: usize, const C: usize> FnEvaluator<F, S, C>
where
    F: Fn(&FsmDna<S, C>) -> f64 + Send + Sync,
{
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F, const S: usize, const C: usize> FitnessEvaluator<S, C> for FnEvaluator<F, S, C>
where
    F: Fn(&FsmDna<S, C>) -> f64 + Send + Sync,
{
    fn evaluate(&self, dna: &FsmDna<S, C>) -> f64 {
        (self.func)(dna)
    }
}
