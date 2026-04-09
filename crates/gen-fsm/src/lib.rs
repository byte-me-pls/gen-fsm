#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]

mod context;
mod dna;
mod fsm;
mod matrix;
mod rng;
mod state;

pub use context::Context;
pub use dna::FsmDna;
pub use fsm::StochasticFsm;
pub use matrix::TransitionMatrix;
pub use rng::Xorshift32;
pub use state::State;
