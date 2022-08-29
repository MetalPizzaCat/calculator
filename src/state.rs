use std::{collections::HashMap};

pub struct State {
    pub variables: HashMap<String, f32>,
}

impl State {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
