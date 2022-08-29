#[macro_use]
extern crate lazy_static;
pub mod config;
pub mod macros;
pub mod node;
pub mod parser;
pub mod state;
pub mod token;
use parser::*;
use state::*;

fn main() {
    println!("Input equation: ");
    let code: String = "(let $a = 3)
    (let $b = max($a,3))
    (out($b))
    "
    .to_owned();

    let input: String = text_io::read!("{}\n");
    let result = parse(&input).unwrap_or_else(|e| panic!("{}", e.to_string()));

    if let Some(tree) = make_tree(&result) {
        let mut state: State = State::new();
        let output = tree.get_value(&mut state);
        for (name, value) in state.variables {
            println!("{} : {}", name, value);
        }
        //println!("Tree! {}", output);
    }
}
//($a = 2 * max((2*3),1))
