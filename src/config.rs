use crate::macros::*;
use std::{collections::HashMap};

lazy_static! {
    /**All of the mathematical constants that can be used in equation*/
    pub static ref CONSTANTS: HashMap<String, f32> = map! {
        ("pi".to_owned() , std::f32::consts::PI),
        ("e".to_owned() , std::f32::consts::E),
    };
    /**All of the standard functions and how many arguments they have*/
    pub static ref FUNCTIONS: HashMap<String, i32> = map! {
        ("sin".to_owned() , 1),
        ("max".to_owned() , 2),
        ("min".to_owned(),2),
        ("cos".to_owned(),1)
    };

    //simple list for converting symbols to u8
    pub static ref PRIORITIES: HashMap<String, u8> = map! {
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
