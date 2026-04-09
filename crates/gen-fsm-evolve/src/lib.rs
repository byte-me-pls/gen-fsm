pub mod config;
pub mod crossover;
pub mod evolution;
pub mod export;
pub mod fitness;
pub mod genome;
pub mod mutation;
pub mod population;
pub mod selection;
pub mod stats;

pub use config::EvolutionConfig;
pub use evolution::{EvolutionEngine, EvolutionResult};
pub use export::DnaExporter;
pub use fitness::FitnessEvaluator;
pub use genome::Genome;
pub use population::Population;
pub use stats::GenerationStats;
