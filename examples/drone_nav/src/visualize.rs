use crate::world::{Cell, World};

pub fn render_world(world: &World, path: &[(usize, usize)], show_legend: bool) -> String {
    let mut display = vec![vec![' '; world.width]; world.height];

    for r in 0..world.height {
        for c in 0..world.width {
            display[r][c] = match world.grid[r][c] {
                Cell::Empty => '·',
                Cell::Obstacle => '█',
                Cell::Start => 'S',
                Cell::Goal => 'G',
            };
        }
    }

    for (i, &(r, c)) in path.iter().enumerate() {
        if world.grid[r][c] == Cell::Start || world.grid[r][c] == Cell::Goal {
            continue;
        }
        display[r][c] = if i == path.len() - 1 { 'D' } else { '○' };
    }

    let mut out = String::new();

    out.push_str("  ┌");
    for _ in 0..world.width * 2 + 1 {
        out.push('─');
    }
    out.push_str("┐\n");

    for r in 0..world.height {
        out.push_str(&format!("{:>2}│ ", r));
        for c in 0..world.width {
            out.push(display[r][c]);
            out.push(' ');
        }
        out.push_str("│\n");
    }

    out.push_str("  └");
    for _ in 0..world.width * 2 + 1 {
        out.push('─');
    }
    out.push_str("┘\n");

    out.push_str("    ");
    for c in 0..world.width {
        out.push_str(&format!("{:<2}", c % 10));
    }
    out.push('\n');

    if show_legend {
        out.push_str("\n  Legend: S=Start  G=Goal  █=Obstacle  ○=Path  D=Drone\n");
    }

    out
}

pub fn print_evolution_header() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║           gen-fsm: Drone Navigation Evolution                ║");
    println!("║        Genetic Finite State Machine — Study Case             ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  States: Cruise | Avoid | Search | Emergency                 ║");
    println!("║  Contexts: Clear | ObstacleNear | ObstacleFar | DeadEnd      ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
}

pub fn print_separator() {
    println!("────────────────────────────────────────────────────────────────");
}

pub fn print_matrices(dna: &gen_fsm::FsmDna<4, 4>) {
    let states = ["Cruise", "Avoid ", "Search", "Emerg."];
    let contexts = ["Clear", "ObstNear", "ObstFar", "DeadEnd"];

    for ctx in 0..4 {
        println!("\n  Context: {} ({})", contexts[ctx], ctx);
        println!("  {:>8} │ {:>8} {:>8} {:>8} {:>8}", "", states[0], states[1], states[2], states[3]);
        println!("  ─────────┼──────────────────────────────────────");

        let matrix = dna.matrix(ctx);
        for from in 0..4 {
            print!("  {:>8} │", states[from]);
            for to in 0..4 {
                let p = matrix.probability(from, to);
                print!(" {:>8.4}", p);
            }
            println!();
        }
    }
}
