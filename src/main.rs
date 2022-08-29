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
    /**All of the mathematical constants that can be used in equation*/
    static ref CONSTANTS: HashMap<String, f32> = map! {
        ("pi".to_owned() , std::f32::consts::PI),
        ("e".to_owned() , std::f32::consts::E),
    };
    /**All of the standard functions and how many arguments they have*/
    static ref FUNCTIONS: HashMap<String, i32> = map! {
        ("sin".to_owned() , 1),
        ("max".to_owned() , 2),
        ("min".to_owned(),2),
        ("cos".to_owned(),1)
    };

    //simple list for converting symbols to u8
    static ref PRIORITIES: HashMap<String, u8> = map! {
        ("+".to_owned(),   2),
        ("-".to_owned(),   2),
        ("*".to_owned(),   3),
        ("/".to_owned(),   3),
        ("(".to_owned(),   4),
        (")".to_owned(),   4),
        ("let".to_owned(), 1),
        ("=".to_owned(),   0)
    };
}

#[derive(Clone, Copy)]
enum TokenType {
    Number,
    Function,
    Variable,
    Operation,
}

#[derive(Clone, Copy)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Create,
}

impl FromStr for OperationType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Add" | "+" | "add" => Ok(OperationType::Add),
            "Sub" | "-" | "sub" => Ok(OperationType::Sub),
            "Mul" | "*" | "mul" => Ok(OperationType::Mul),
            "Div" | "/" | "div" => Ok(OperationType::Div),
            "Assign" | "=" | "assign" => Ok(OperationType::Assign),
            "Create" | "let" | "create" => Ok(OperationType::Create),
            _ => Err("Invalid enum string passed".to_owned()),
        }
    }
}

#[derive(Clone)]
struct Token {
    pub token_type: TokenType,
    pub number: Option<f32>,
    pub operation: Option<OperationType>,
    pub variable: Option<String>,
    pub function: Option<String>,
}

struct Node {
    pub children: Vec<Node>,
    pub token: Token,
}

struct State {
    pub variables: HashMap<String, f32>,
}

impl State {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl Node {
    /**Get value of the child node by index */
    pub fn get_child_value(&self, index: usize, state: &mut State) -> Result<Option<f32>, String> {
        if let Some(child) = self.children.get(index) {
            let b = child;
            return child.get_value(state);
        }
        Err("No child node with given index".to_owned())
    }

    /**Executes every children node
     * This is used for nodes that are not supposed to have return types
     * Like loop bodies
     */
    pub fn execute(&self, state: &mut State) {}

    /**Recursively gets value for the node
     *
     */
    pub fn get_value(&self, state: &mut State) -> Result<Option<f32>, String> {
        match &self.token.token_type {
            TokenType::Number => {
                return Ok(self.token.number);
            }
            TokenType::Function => {
                let mut result: f32 = 0.0;
                match self.token.function.as_ref().unwrap().as_str() {
                    "sin" => {
                        if let Some(a) = &self.get_child_value(0, state)? {
                            return Ok(Some(a.sin()));
                        }
                    }
                    "max" => {
                        if let Some(a) = self.get_child_value(0, state)? {
                            if let Some(b) = self.get_child_value(1, state)? {
                                return Ok(Some(max!(a, b)));
                            }
                        }
                    }
                    _ => {}
                };
            }
            TokenType::Variable => {
                if let Some(name) = &self.token.variable.as_ref() {
                    if let Some(var) = state.variables.get(name.clone()) {
                        return Ok(Some(*var));
                    }
                }
                return Err("Variable was not present in the state".to_owned());
            }
            TokenType::Operation => {
                let mut result: f32 = 0.0;
                match self.token.operation.unwrap() {
                    OperationType::Add => {
                        if let Some(a) = self.get_child_value(0, state)? {
                            if let Some(b) = self.get_child_value(1, state)? {
                                return Ok(Some(a + b));
                            }
                        }
                    }
                    OperationType::Div => {
                        if let Some(a) = self.get_child_value(0, state)? {
                            if let Some(b) = self.get_child_value(1, state)? {
                                return Ok(Some(a / b));
                            }
                        }
                    }
                    OperationType::Mul => {
                        if let Some(a) = self.get_child_value(0, state)? {
                            if let Some(b) = self.get_child_value(1, state)? {
                                return Ok(Some(a * b));
                            }
                        }
                    }
                    OperationType::Sub => {
                        if let Some(a) = self.get_child_value(0, state)? {
                            if let Some(b) = self.get_child_value(1, state)? {
                                return Ok(Some(a - b));
                            }
                        }
                    }
                    OperationType::Assign => {
                        //this is an arrow
                        if let Some(child) = self.children.get(0) {
                            if matches!(child.token.token_type, TokenType::Variable) {
                                if let Some(var_name) = &child.token.variable {
                                    if let Some(value) = self.get_child_value(1, state)? {
                                        if let Some(var) =
                                            state.variables.get_mut(&var_name.clone())
                                        {
                                            *var = value;
                                        } else {
                                            state.variables.insert(var_name.clone(), value);
                                        }
                                    } else {
                                        return Err(
                                            "Right side of the operation has no value".to_owned()
                                        );
                                    }
                                } else {
                                    return Err("No variable info provided".to_owned());
                                }
                            }
                        } else {
                            return Err("Left side of the assignment operation much be a variable"
                                .to_owned());
                        }
                    }
                    OperationType::Create => {
                        //this is an arrow
                        if let Some(child) = self.children.get(0) {
                            if matches!(child.token.token_type, TokenType::Variable) {
                                if let Some(var_name) = &child.token.variable {
                                    state.variables.insert(var_name.clone(), 0.0);
                                }
                            }
                        }
                        return Err("Need variable name to create a variable".to_owned());
                    }
                    _ => return Ok(None),
                };
            }
            _ => {}
        }
        Ok(None)
    }
}

impl Node {
    pub fn new(token: Token) -> Self {
        Self {
            children: Vec::new(),
            token,
        }
    }

    pub fn new_with(children: Vec<Node>, token: Token) -> Self {
        Self { children, token }
    }
}

impl Token {
    pub fn new_number(number: f32) -> Self {
        Self {
            token_type: TokenType::Number,
            number: Some(number),
            operation: None,
            variable: None,
            function: None,
        }
    }
    pub fn new_operation(op: OperationType) -> Self {
        Self {
            token_type: TokenType::Operation,
            number: None,
            operation: Some(op),
            variable: None,
            function: None,
        }
    }

    pub fn new_variable(name: String) -> Self {
        Self {
            token_type: TokenType::Variable,
            number: None,
            operation: None,
            variable: Some(name),
            function: None,
        }
    }

    pub fn new_function(name: String) -> Self {
        Self {
            token_type: TokenType::Function,
            number: None,
            operation: None,
            variable: None,
            function: Some(name),
        }
    }
}

fn make_tree(input: &Vec<Token>) -> Option<Node> {
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

fn get_operation_token(input: String) -> Result<Option<Token>, String> {
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

fn parse(input: &String) -> Result<Vec<Token>, String> {
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
    //(==)|((\d)+(\.\d+)?)|([+-/*()=])|(\$?(\w+))
    let reg_ex =
        Regex::new(r"(==)|((\d)+(\.\d+)?)|([+-/*()=])|(\$?(\w+))").map_err(|e| e.to_string())?;
    let var_regex = Regex::new(r"(\$(\w+))").map_err(|e| e.to_string())?;
    let matches = reg_ex.find_iter(input.as_str());
    for token in matches {
        let val = token.as_str();
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
                    let a = &op;
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
        let c = &tree;
        let mut state: State = State::new();
        let output = tree.get_value(&mut state);
        for (name, value) in state.variables {
            println!("{} : {}", name, value);
        }
        //println!("Tree! {}", output);
    }
}
//($a = 2 * max((2*3),1))
