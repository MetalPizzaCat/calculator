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

fn main() -> Result<(), String> {
    println!("Input equation: ");
    let code: String = "(let $a = 3)
    (let $b = max($a,3))
    (out($b))
    "
    .to_owned();

    //let input: String = text_io::read!s"{}\n");
    let input = "$a = 2; $b = 2 + $a*2;$c = max($a,$b);$a + 2".to_owned();
    println!("{}", input);
    let result = parse_block(&input).unwrap_or_else(|e| panic!("{}", e.to_string()));

    let mut state: State = State::new();
    if let Some(output) = result.execute(&mut state).map_err(|e| e.to_string())? {
        println!("Program returned:  {}", output);
    }
    println!("Variable state");
    for (name, value) in state.variables {
        println!("{} : {}", name, value);
    }

    Ok(())
}
//($a = 2 * max((2*3),1))
