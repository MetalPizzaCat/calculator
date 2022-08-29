use crate::macros::*;
use crate::state::*;
use crate::token::*;

pub struct Node {
    pub children: Vec<Node>,
    pub token: Token,
}

impl Node {
    /**Get value of the child node by index */
    pub fn get_child_value(&self, index: usize, state: &mut State) -> Result<Option<f32>, String> {
        if let Some(child) = self.children.get(index) {
            return child.get_value(state);
        }
        Err("No child node with given index".to_owned())
    }

    /**Executes every children node
     * This is used for nodes that are not supposed to have return types
     * Like loop bodies
     * If a node that returns a value(such as math equation without assignment)
     * then this value is treated as result of the block and execution stops
     */
    pub fn execute(&self, state: &mut State) -> Result<Option<f32>, String> {
        
        for child in &self.children {
            if let Some(result) = child.get_value(state)? {
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    /**Recursively gets value for the node
     *
     */
    pub fn get_value(&self, state: &mut State) -> Result<Option<f32>, String> {
        match &self.token.token_type {
            TokenType::Number => {
                return Ok(self.token.number);
            }
            TokenType::Function => {
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
                };
            }
            _ => {/*ignore every other token because they will be used for other reasons */}
        }
        Ok(None)
    }

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
