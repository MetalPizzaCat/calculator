#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{collections::HashMap, str::FromStr};

/**Simple macro for generating HashMaps */
macro_rules! map {
    ($(($k:expr , $v:expr)),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

lazy_static! {
    static ref CONSTANTS: HashMap<String, f32> = map! {
        ("pi".to_owned() , std::f32::consts::PI),
        ("e".to_owned() , std::f32::consts::E),
    };
}
enum TokenType {
    Number,
    Function,
    Constant,
    Operation,
}

#[derive(Clone, Copy)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for OperationType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Add" | "+" | "add" => Ok(OperationType::Add),
            "Sub" | "-" | "sub" => Ok(OperationType::Sub),
            "Mul" | "*" | "mul" => Ok(OperationType::Mul),
            "Div" | "/" | "div" => Ok(OperationType::Div),
            _ => Err("Invalid enum string passed".to_owned()),
        }
    }
}

struct Token {
    pub token_type: TokenType,
    pub number: Option<f32>,
    pub operation: Option<OperationType>,
    pub constant: Option<String>,
    pub function: Option<String>,
}


impl Token {
    pub fn new_number(number: f32) -> Self {
        Self {
            token_type: TokenType::Number,
            number: Some(number),
            operation: None,
            constant: None,
            function: None,
        }
    }
    pub fn new_operation(op: OperationType) -> Self {
        Self {
            token_type: TokenType::Operation,
            number: None,
            operation: Some(op),
            constant: None,
            function: None,
        }
    }

    pub fn new_constant(name: String) -> Self {
        Self {
            token_type: TokenType::Constant,
            number: None,
            operation: None,
            constant: Some(name),
            function: None,
        }
    }

    pub fn new_function(name: String) -> Self {
        Self {
            token_type: TokenType::Function,
            number: None,
            operation: None,
            constant: None,
            function: Some(name),
        }
    }
}

const FUNCTION_NAMES: [&str; 5] = ["sin", "max", "cos", "min", "pi"];
const CONSTANT_NAMES: [&str; 2] = ["pi", "e"];

fn calculate(input: &Vec<Token>) -> Option<f32> {
    let mut numbers: Vec<f32> = Vec::new();
    for token in input {
        match token.token_type {
            TokenType::Constant => {
                numbers.push(CONSTANTS[token.constant.as_ref().unwrap()]);
            }
            TokenType::Function => {
                let mut result: f32 = 0.0;
                match token.function.as_ref().unwrap().as_str() {
                    "sin" => {
                        let a = numbers.pop().unwrap();
                        result = a.sin();
                    }
                    "max" => {
                        let a = numbers.pop().unwrap();
                        let b = numbers.pop().unwrap();
                        result = max!(a, b);
                    }
                    _ => {}
                };
                numbers.push(result);
            }
            TokenType::Number => {
                numbers.push(token.number.unwrap());
            }
            TokenType::Operation => {
                let mut result: f32 = 0.0;
                match token.operation.unwrap() {
                    OperationType::Add => {
                        let a = numbers.pop().unwrap();
                        let b = numbers.pop().unwrap();
                        result = a + b;
                    }
                    OperationType::Div => {
                        let a = numbers.pop().unwrap();
                        let b = numbers.pop().unwrap();
                        result = a / b;
                    }
                    OperationType::Mul => {
                        let a = numbers.pop().unwrap();
                        let b = numbers.pop().unwrap();
                        result = a * b;
                    }
                    OperationType::Sub => {
                        let a = numbers.pop().unwrap();
                        let b = numbers.pop().unwrap();
                        result = a - b;
                    }
                    _ => {}
                };
                numbers.push(result);
            }
        }
    }
    numbers.pop()
}

fn parse(input: &String) -> Result<Vec<Token>, String> {
    //simple list for converting symbols to u8
    let priorities: HashMap<&str, u8> = map! {
        ("+" ,  0),
        ("-",   0),
        ("*",   1),
        ("/",   1),
        ("(",   2),
        (")",   2),
    };

    let mut result: Vec<Token> = Vec::new();
    let mut out: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    //This regex matches either any floating point number
    //(([0-9])+(\.[0-9]+)?)
    //(\.[0-9]+)?) this is for optional usage of dot
    //or any of the operation symbols
    //(\+|\-|\*|/|\(|\)))
    //this is for words(which are function names)
    //([a-z]+)
    let reg_ex = Regex::new(r"((\d)+(\.\d+)?)|([+-/*()])|([a-z]+)")
        .map_err(|e| e.to_string())?;
    let matches = reg_ex.find_iter(input.as_str());
    for token in matches {
        let val = token.as_str();
        //println!("{}", token.as_str());
        if let Ok(num) = token.as_str().parse::<f32>() {
            println!("{}", token.as_str());
            result.push(Token::new_number(num));
            continue;
        }
        if CONSTANT_NAMES.contains(&token.as_str()) {
            println!("{}", token.as_str());
            result.push(Token::new_constant(token.as_str().to_owned()));
            continue;
        }
        match token.as_str() {
            "(" => {
                stack.push("(".to_owned());
            }
            ")" => {
                while let Some(op) = stack.pop() {
                    if op == "(" {
                        if let Some(func) = stack.pop() {
                            if FUNCTION_NAMES.contains(&func.as_str()) {
                                println!("{}", func);
                                result.push(Token::new_function(func));
                            }
                        }
                        break;
                    }
                    println!("{}", op);
                    if let Ok(operation) = OperationType::from_str(&op) {
                        result.push(Token::new_operation(operation));
                    }
                }
            }
            _ => {
                let priority: u8 = *priorities.get(token.as_str()).unwrap_or(&99);
                while let Some(op) = stack.pop() {
                    if FUNCTION_NAMES.contains(&op.as_str())
                        || op == "("
                        || op == ")"
                        || priorities[&op.as_str()] < priority
                    {
                        stack.push(op);
                        break;
                    }
                    println!("{}", op);
                    out.push(op);
                }
                stack.push(token.as_str().to_owned());
            }
        }
    }
    Ok(result)
}

fn main() {
    println!("Input equation: ");
    let input: String = text_io::read!("{}\n");
    let result = parse(&input).unwrap_or_else(|e| panic!("{}", e.to_string()));
    if let Some(result) = calculate(&result) {
        println!("Result :{}", result);
    } else {
        println!("Failed to execute");
    }
}
