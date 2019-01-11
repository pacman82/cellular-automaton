use rand::prelude::*;
use structopt::StructOpt;

/// Prints the evolution of the state of a cellular automaton to standard out.
///
/// The cellular automaton is one dimensional with each Cell being in one of two states. The next
/// state of each cell is only influenced by its state and the state of its two neighbours. The
/// leftmost and rightmost cells at the borders are considered to be neighbours.
/// Each cells next state therefore depends on 3 current states. This means  2^3=8 rules are
/// required to evolve the state of the automaton. Each Rule can have two outcomes, which amounts
/// to 2^8=256 rule sets in total.
#[derive(StructOpt)]
struct Opt {
    /// The ruleset used to evolve the automatons state. Any number between 0 and 255.
    #[structopt()]
    rule: u8,
    /// Number of steps to evolve the cellular automaton
    #[structopt(long = "steps", short = "s", default_value = "100")]
    steps: usize,
    /// Number of cells within the Automaton
    #[structopt(long = "width", short = "w", default_value = "80")]
    width: usize,
    /// Random initial state
    #[structopt(long = "random", short = "r")]
    random: bool,
}

#[derive(Clone, Copy)]
enum Cell {
    X,
    O,
}

fn rule_from_byte(byte: u8) -> [Cell; 8] {
    let mut rule = [Cell::O; 8];
    for i in 0..8 {
        rule[i] = if 2_u8.pow(i as u32) & byte != 0 {
            Cell::X
        } else {
            Cell::O
        }
    }
    rule
}

fn main() {
    let Opt {
        rule,
        steps,
        width,
        random,
    } = Opt::from_args();

    let rule = rule_from_byte(rule);

    let mut state1;
    let mut state2 = vec![Cell::O; width];

    // set initial state
    if random {
        state1 = rand::thread_rng()
            .sample_iter(&rand::distributions::Standard)
            .take(width)
            .collect();
    } else {
        state1 = vec![Cell::O; width];
        state1[width / 2] = Cell::X;
    }
    print_state(&state1);

    for generation in 0..steps {
        let (current, previous) = if generation % 2 == 0 {
            (&mut state2, &state1)
        } else {
            (&mut state1, &state2)
        };
        apply_rules(previous, current, &rule);
        print_state(current);
    }
}

fn apply_rules(previous: &[Cell], next: &mut [Cell], rule: &[Cell; 8]) {
    for (index, cell) in next.iter_mut().enumerate() {
        let neighbours = [
            previous[(index + previous.len() - 1) % previous.len()],
            previous[index],
            previous[(index + 1) % previous.len()],
        ];
        *cell = apply_rule(neighbours, &rule);
    }
}

fn apply_rule(neighbours: [Cell; 3], rule: &[Cell; 8]) -> Cell {
    use crate::Cell::*;
    match neighbours {
        [O, O, O] => rule[0],
        [O, O, X] => rule[1],
        [O, X, O] => rule[2],
        [O, X, X] => rule[3],
        [X, O, O] => rule[4],
        [X, O, X] => rule[5],
        [X, X, O] => rule[6],
        [X, X, X] => rule[7],
    }
}

fn print_state(state: &[Cell]) {
    for cell in state {
        match cell {
            Cell::O => print!("░"),
            Cell::X => print!("█"),
        }
    }
    print!("\n");
}

impl Distribution<Cell> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        if rng.gen() {
            Cell::X
        } else {
            Cell::O
        }
    }
}
