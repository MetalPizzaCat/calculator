use std::{collections::HashMap};

/**Current state of the program
 * This covers current variables and if added custom defined functions
 */
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
