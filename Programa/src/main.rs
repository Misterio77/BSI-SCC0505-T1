use scc050_t1::automaton::{AutomatonBuilder, AutomatonError};
use std::io;

fn main() -> Result<(), AutomatonError> {
    let automaton = AutomatonBuilder {
        states: vec![0, 1, 2],
        symbols: vec!['a', 'b'],
        initial_states: vec![1],
        accepted_states: vec![2],
        transitions: vec![
            (0, 'a', 1),
            (0, 'b', 1),
            (1, 'a', 1),
            (1, 'b', 2),
            (2, 'a', 0),
            (2, 'b', 2),
        ],
    }
    .build()?;

    loop {
        let mut input = String::new();
        let chain: Vec<char> = match io::stdin().read_line(&mut input) {
            Ok(_) => input.chars().collect(),
            Err(_) => return Ok(()),
        };

        if automaton.verify_chain(&chain) {
            println!("aceita");
        } else {
            println!("rejeita")
        }
    }
}
