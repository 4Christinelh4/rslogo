use crate::parser::constant::*;
use crate::parser::helper::*;
use crate::parser::turtle::*;
use std::error::Error;

pub fn parse_func_arguments<'a, 'b: 'a>(
    turtle: &'a mut Turtle<'b>,
    f: &'b Func,
    call_line: &Vec<&str>, // BOX [arg]
    prev_turtle: &Turtle,
) -> Option<()> {
    // call_line: PENTAGRAM * :C "3

    // based on var_list in f, assign value in turtle's var_map
    let arg_list: Vec<&str> = f.argv.as_str().split(' ').collect();
    // println!("arg list = {:?}", arg_list);

    let mut expr_start: usize = 1;

    for arg_idx in 0..f.num_args {
        match parse_value(&prev_turtle, call_line, expr_start) {
            Some(res) => {
                // println!("result to insert: [{}, {:?}]", &arg_list[arg_idx as usize][1..], res);
                if res.3 {
                    // f32
                    turtle.insert_varmap(
                        &arg_list[arg_idx as usize][1..],
                        true,
                        res.0,
                        String::from(""),
                    );
                } else {
                    turtle.insert_varmap(&arg_list[arg_idx as usize][1..], false, 0.0, res.1);
                }

                expr_start = res.2;
            }
            None => return None,
        };

        // turtle.insert_varmap(&arg_list[arg_idx][1..], );
    }

    return Some(());
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
        return Err("procedure doesnt have END".into());
    }

    // TO Box argu1 argu2
    let splitted: Vec<&str> = commands[i].as_str().trim_start().split(' ').collect();
    if splitted.len() < 2 {
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
