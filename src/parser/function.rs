use crate::parser::constant::*;
use crate::parser::helper::*;
use crate::parser::turtle::*;

use std::error::Error;
use std::collections::HashMap;

pub fn parse_func_arguments(turtle: &Turtle, f_map: &mut HashMap<&str, f32>, f: &Func) -> bool {
    return false;
}

pub fn define_procedure(turtle: &mut Turtle, commands: &Vec<String>, i: usize) -> Result<usize, Box<dyn Error>> {
    Ok(i)
}
