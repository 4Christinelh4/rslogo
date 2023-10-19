use crate::parser::constant::*;

use crate::parser::turtle::*;

use std::collections::HashMap;
use std::error::Error;

pub fn parse_func_arguments(
    turtle: &mut Turtle,
    f_map: &mut HashMap<&str, f32>,
    f: &Func,
    call_line: &Vec<&str>,
) -> bool {
    return false;
}

// if ok, return the line where END is
pub fn define_procedure<'a, 'b: 'a>(
    turtle: &'a mut Turtle<'b>,
    commands: &'b Vec<String>,
    i: usize,
) -> Result<usize, Box<dyn Error>> {
    // find the last_line
    let total = commands.len();
    let mut last_line: usize = 0;

    for k in i..total {
        if commands[k] == String::from("END") {
            last_line = k;
            break;
        }
    }

    if 0 == last_line {
        return Err("procedure don't have END".into());
    }

    // TO Box argu1 argu2
    let splitted: Vec<&str> = commands[i].as_str().split(' ').collect();
    if splitted.len() <= 2 {
        return Err("not enough arguments".into());
    }

    turtle.insert_funcmap(
        &splitted[1],
        i,
        last_line,
        (splitted.len() - 2) as i32,
        splitted[2..].join(" "),
    );

    Ok(last_line)
}
