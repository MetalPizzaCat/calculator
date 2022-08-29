/**
    This file contains functions for converting text into memory structure that can be executed by the program
    Few notes:
    1) While converting into RPN then into tree could be skipped, this project is simply a build upon previous iterations
    so i just reused parser that already worked
    2) ???

 */
use crate::token::*;
use crate::node::*;
use crate::config::*;

use std::str::FromStr;

use regex::Regex;

/**Converts RPN into something that resembles abstract syntax tree */
pub fn make_tree(input: &Vec<Token>) -> Option<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    for token in input {
        match token.token_type {
            TokenType::Variable => nodes.push(Node::new(token.clone())),
            TokenType::Function => {
                if let Some(func_name) = &token.function {
                    if let Some(arg_count) = FUNCTIONS.get(&func_name.clone()) {
                        if *arg_count == 1 {
                            let a: Node = nodes.pop().unwrap();
                            nodes.push(Node::new_with(vec![a], token.clone()));
                        } else if *arg_count == 2 {
                            let a: Node = nodes.pop().unwrap();
                            let b: Node = nodes.pop().unwrap();
                            nodes.push(Node::new_with(vec![a, b], token.clone()));
                        }
                    }
                }
            }
            TokenType::Number => nodes.push(Node::new(token.clone())),
            TokenType::Operation => {
                if matches!(token.operation?, OperationType::Create) {
                    let a: Node = nodes.pop().unwrap();
                    nodes.push(Node::new_with(vec![a], token.clone()));
                } else {
                    let a: Node = nodes.pop().unwrap();
                    let b: Node = nodes.pop().unwrap();
                    nodes.push(Node::new_with(vec![b, a], token.clone()));
                }
            }
        }
    }
    nodes.pop()
}

/**Convert given string into operation token, ignoring special symbols */
pub fn get_operation_token(input: String) -> Result<Option<Token>, String> {
    if input != "(" && input != ")" && input != "," {
        if let Ok(operation) = OperationType::from_str(&input) {
            return Ok(Some(Token::new_operation(operation)));
        } else if FUNCTIONS.contains_key(&input) {
            return Ok(Some(Token::new_function(input)));
        } else {
            return Err("Invalid operation".to_owned());
        }
    }
    Ok(None)
}

/**Parses line into reverse polish notation */
pub fn parse_line(input: &String) -> Result<Vec<Token>, String> {
    let mut result: Vec<Token> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    //This regex matches either any floating point number
    //(([0-9])+(\.[0-9]+)?)
    //(\.[0-9]+)?) this is for optional usage of dot
    //or any of the operation symbols
    //(\+|\-|\*|/|\(|\)))
    //this is for words(which are function names)
    //([a-z]+)
    //(==)|((\d)+(\.\d+)?)|([+-/*()=])|(\$?(\w+))
    let reg_ex =
        Regex::new(r"(==)|((\d)+(\.\d+)?)|([+-/*()=])|(\$?(\w+))").map_err(|e| e.to_string())?;
    let var_regex = Regex::new(r"(\$(\w+))").map_err(|e| e.to_string())?;
    let matches = reg_ex.find_iter(input.as_str());
    for token in matches {
        //println!("{}", token.as_str());
        if let Ok(num) = token.as_str().parse::<f32>() {
            //println!("{}", token.as_str());
            result.push(Token::new_number(num));
            continue;
        }
        if CONSTANTS.contains_key(&token.as_str().to_owned()) {
            //println!("{}", token.as_str());
            result.push(Token::new_number(CONSTANTS[&token.as_str().to_owned()]));
            continue;
        }
        if var_regex.is_match(&token.as_str()) {
            result.push(Token::new_variable(token.as_str().to_owned()));
            //println!("{}", token.as_str());
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
                            if FUNCTIONS.contains_key(&func) {
                                println!("{}", func);
                                result.push(Token::new_function(func));
                            }
                        }
                        break;
                    }
                    if let Some(token) = get_operation_token(op)? {
                        result.push(token)
                    }
                    //println!("{}", op);
                }
            }
            _ => {
                let priority: u8 = *PRIORITIES.get(token.as_str()).unwrap_or(&99);
                while let Some(op) = stack.pop() {
                    if FUNCTIONS.contains_key(&op)
                        || op == "("
                        || op == ")"
                        || PRIORITIES[&op.as_str().to_owned()] < priority
                    {
                        stack.push(op);
                        break;
                    }
                    if let Some(token) = get_operation_token(op)? {
                        result.push(token)
                    }
                }
                stack.push(token.as_str().to_owned());
            }
        }
    }
    Ok(result)
}
