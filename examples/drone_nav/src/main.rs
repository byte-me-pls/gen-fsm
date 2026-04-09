mod drone;
mod sensor;
mod visualize;
mod world;

use drone::{Direction, DroneContext, DroneState};
use gen_fsm::{FsmDna, StochasticFsm, Xorshift32};
use gen_fsm_evolve::{DnaExporter, EvolutionConfig, EvolutionEngine, FitnessEvaluator};
use world::World;

const MAX_STEPS: usize = 200;
const EVAL_RUNS: usize = 3;

struct DroneEvaluator {
    world: World,
}

impl FitnessEvaluator<4, 4> for DroneEvaluator {
    fn evaluate(&self, dna: &FsmDna<4, 4>) -> f64 {
        let mut total_fitness = 0.0;

        for seed in 1..=EVAL_RUNS {
            let result = simulate(dna, &self.world, seed as u32 * 137);
            total_fitness += result.fitness;
        }

        total_fitness / EVAL_RUNS as f64
    }
}

struct SimResult {
    fitness: f64,
    path: Vec<(usize, usize)>,
    reached_goal: bool,
    collisions: usize,
}

fn simulate(dna: &FsmDna<4, 4>, world: &World, rng_seed: u32) -> SimResult {
    let mut fsm = StochasticFsm::<DroneState, DroneContext, 4, 4>::new(
        dna.clone(),
        DroneState::Cruise,
        rng_seed,
    );

    let mut pos = world.start;
    let mut path = vec![pos];
    let mut collisions = 0;
    let mut visited = std::collections::HashSet::new();
    visited.insert(pos);

    let mut move_rng = Xorshift32::new(rng_seed.wrapping_add(1000));

    let max_distance = World::manhattan_distance(world.start, world.goal);

    for _step in 0..MAX_STEPS {
        if pos == world.goal {
            break;
        }

        let context = sensor::read_context(world, pos);

        let state = fsm.step(context);

        let new_pos = execute_behavior(state, pos, world, &mut move_rng);

        if let Some(np) = new_pos {
            if world.is_walkable(np.0, np.1) {
                pos = np;
                visited.insert(pos);
            } else {
                collisions += 1;
            }
        }

        path.push(pos);
    }

    let current_distance = World::manhattan_distance(pos, world.goal);
    let distance_score = 1.0 - (current_distance / max_distance.max(1.0));
    let collision_penalty = collisions as f64 * 0.05;
    let goal_bonus = if pos == world.goal { 3.0 } else { 0.0 };
    let efficiency = if pos == world.goal {
        (MAX_STEPS as f64 - path.len() as f64) / MAX_STEPS as f64
    } else {
        0.0
    };
    let exploration_bonus = (visited.len() as f64 / 20.0).min(0.5);

    let fitness =
        (distance_score * 2.0 + goal_bonus + efficiency + exploration_bonus - collision_penalty)
            .max(0.0);

    SimResult {
        fitness,
        path,
        reached_goal: pos == world.goal,
        collisions,
    }
}

fn execute_behavior(
    state: DroneState,
    pos: (usize, usize),
    world: &World,
    rng: &mut Xorshift32,
) -> Option<(usize, usize)> {
    match state {
        DroneState::Cruise => {
            let (gr, gc) = world.goal;
            let dr = (gr as isize - pos.0 as isize).signum();
            let dc = (gc as isize - pos.1 as isize).signum();

            let row_dist = (gr as isize - pos.0 as isize).abs();
            let col_dist = (gc as isize - pos.1 as isize).abs();

            if row_dist > col_dist {
                try_move(pos, dr, 0, world)
                    .or_else(|| try_move(pos, 0, dc, world))
            } else {
                try_move(pos, 0, dc, world)
                    .or_else(|| try_move(pos, dr, 0, world))
            }
        }
        DroneState::Avoid => {
            let mut options: Vec<(usize, usize)> = Vec::new();
            for dir in &Direction::ALL {
                if let Some(np) = dir.apply(pos.0, pos.1, world.height, world.width) {
                    if world.is_walkable(np.0, np.1) {
                        options.push(np);
                    }
                }
            }
            if options.is_empty() {
                None
            } else {
                let idx = (rng.next_u32() as usize) % options.len();
                Some(options[idx])
            }
        }
        DroneState::Search => {
            let r = (rng.next_u32() % 4) as usize;
            Direction::ALL[r].apply(pos.0, pos.1, world.height, world.width)
        }
        DroneState::Emergency => {
            Some(pos)
        }
    }
}

fn try_move(
    pos: (usize, usize),
    dr: isize,
    dc: isize,
    world: &World,
) -> Option<(usize, usize)> {
    if dr == 0 && dc == 0 {
        return None;
    }
    let nr = pos.0 as isize + dr;
    let nc = pos.1 as isize + dc;
    if nr >= 0 && nc >= 0 {
        let nr = nr as usize;
        let nc = nc as usize;
        if nr < world.height && nc < world.width && world.is_walkable(nr, nc) {
            return Some((nr, nc));
        }
    }
    None
}

fn main() {
    visualize::print_evolution_header();

    let world = World::test_map();

    println!("  World Map:");
    print!("{}", visualize::render_world(&world, &[], true));
    visualize::print_separator();

    let config = EvolutionConfig {
        population_size: 150,
        max_generations: 200,
        mutation_rate: 0.08,
        mutation_sigma: 0.12,
        crossover_rate: 0.85,
        tournament_size: 4,
        elite_count: 8,
        target_fitness: Some(5.5),
        seed: 42,
    };

    println!("  Evolution Config:");
    println!("    Population: {}", config.population_size);
    println!("    Generations: {}", config.max_generations);
    println!("    Mutation rate: {}", config.mutation_rate);
    println!("    Crossover rate: {}", config.crossover_rate);
    println!("    Target fitness: {:?}", config.target_fitness);
    visualize::print_separator();
    println!();
    println!("  Evolving...");
    println!();

    let evaluator = DroneEvaluator {
        world: world.clone(),
    };

    let engine = EvolutionEngine::<4, 4, _>::new(config, evaluator);

    let result = engine.run(|stats| {
        if stats.generation % 10 == 0 || stats.generation < 5 {
            println!("  {}", stats);
        }
    });

    visualize::print_separator();
    println!();
    println!("  ✅ Evolution Complete!");
    println!("    Generations: {}", result.generations_run);
    println!("    Best fitness: {:.4}", result.best_fitness);
    println!(
        "    Target reached: {}",
        if result.target_reached { "YES" } else { "NO" }
    );

    let best_dna = result.best_dna();
    println!();
    println!("  Evolved Transition Matrices:");
    visualize::print_matrices(&best_dna);

    visualize::print_separator();
    println!();
    println!("  Demo Run (seed=1):");
    let demo = simulate(&best_dna, &world, 1);
    println!("    Steps: {}", demo.path.len());
    println!("    Collisions: {}", demo.collisions);
    println!(
        "    Goal reached: {}",
        if demo.reached_goal { "YES ✅" } else { "NO ❌" }
    );
    println!();
    print!("{}", visualize::render_world(&world, &demo.path, true));

    visualize::print_separator();
    println!();
    println!("  Exported DNA (Rust const):");
    println!();
    let rust_code = DnaExporter::to_rust_const(&best_dna, "DRONE_DNA");
    println!("{}", rust_code);
}
