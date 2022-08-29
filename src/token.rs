use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum TokenType {
    Number,
    Function,
    Variable,
    Operation,
}

#[derive(Clone, Copy)]
pub enum OperationType {
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
pub struct Token {
    pub token_type: TokenType,
    pub number: Option<f32>,
    pub operation: Option<OperationType>,
    pub variable: Option<String>,
    pub function: Option<String>,
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
