use std::collections::HashMap;
use std::error::Error;

mod compute;
mod constant;
mod function;
mod helper;
pub mod turtle;

pub fn turtle_move<'a, 'b: 'a>(
    commands: &'b Vec<String>,
    turtle: &mut turtle::Turtle<'a>,
    img: &mut unsvg::Image,
    start_execute: usize,
    finish_execute: usize,
    function_var_map: &HashMap<&str, f32>,
) -> Result<(f32, f32), Box<dyn Error>> {
    // dir = 0: up
    // 360 - dir
    let mut i = start_execute;
    while i < finish_execute {
        // println!("{}", commands[i]);
        let line_ref = commands[i].as_str().trim_start();
        if line_ref.is_empty() || helper::is_comment(line_ref) {
            i += 1;
            continue;
        }

        let splitted: Vec<&str> = line_ref.split(' ').collect();

        match splitted[0] {
            constant::DEFINE_PROCEDURE => {
                // need to loop through all commands to find the end to make sure the procedure has the end
                // match function::define_procedure(&mut turtle, commands, i) {
                //     Ok(last_line) => {
                //         // move i to 1 + last_line
                //         i = 1 + last_line;
                //         continue;
                //     }
                //     Err(_) => std::process::exit(1),
                // }
            }

            // setpencolor [start of expression] + only 1 value
            constant::IS_SETPENCOLOR | constant::IS_TURN | constant::IS_SETHEADING => {
                if splitted.len() > 2 && !compute::is_arithmetic_operator(splitted[1])
                    || splitted.len() == 1
                {
                    std::process::exit(1);
                }

                let val: f32;
                match helper::parse_value(&turtle, &splitted, 1) {
                    Some(v) => {
                        // Option<(f32, String, usize, bool)>
                        if v.3 {
                            val = v.0;
                        } else {
                            std::process::exit(1);
                        }
                    }
                    None => std::process::exit(1),
                };

                // check if it's interger
                if !helper::is_i32(val) {
                    std::process::exit(1);
                }

                if constant::IS_SETPENCOLOR == splitted[0] {
                    turtle.set_color(val as i32);
                } else if constant::IS_TURN == splitted[0] {
                    turtle.turn(val as i32);
                } else {
                    turtle.set_heading(val as i32);
                }
            }

            // unsvg::COLORS[idx as usize];
            constant::IS_PENDOWN | constant::IS_PENUP => {
                if splitted.len() != 1 {
                    std::process::exit(1);
                }

                if constant::IS_PENDOWN == splitted[0] {
                    turtle.set_pendown();
                } else {
                    turtle.set_penup();
                }
            }

            constant::IS_SETX
            | constant::IS_SETY
            | constant::IS_FORWARD
            | constant::IS_BACK
            | constant::IS_RIGHT
            | constant::IS_LEFT => {
                if (splitted.len() > 2 && !compute::is_arithmetic_operator(&splitted[1]))
                    || splitted.len() == 1
                {
                    std::process::exit(1);
                }

                let val: f32;
                match helper::parse_value(turtle, &splitted, 1) {
                    Some(v) => {
                        if v.3 {
                            val = v.0;
                        } else {
                            std::process::exit(1);
                        }
                    }
                    None => std::process::exit(1),
                };

                if constant::IS_SETX == splitted[0] {
                    turtle.set_x(val);
                } else if constant::IS_SETY == splitted[0] {
                    turtle.set_y(val);
                } else {
                    let _ = turtle.moving(val, &splitted[0], img);
                }
            }

            constant::IS_MAKE => match helper::make_cmd(turtle, &splitted) {
                None => std::process::exit(1),
                Some(_) => {}
            },

            // only allows f32!!!
            constant::IS_ADDASSIGN => match helper::add_assign(turtle, &splitted) {
                None => std::process::exit(1),
                Some(_) => {}
            },

            constant::IS_WHILE | constant::IS_IF => {
                println!("i = {}", i);

                match turtle.search_end(i) {
                    None => match helper::add_controlflow(i, commands, turtle) {
                        None => std::process::exit(1),
                        Some(_) => {}
                    },
                    Some(_) => {}
                };

                println!("i = {} turtle_search end = {:?}", i, turtle.search_end(i));

                match helper::check_condition(i, &splitted, &turtle) {
                    Some(res) => {
                        if res {
                            i += 1;
                            continue;
                        } else {
                            println!("i = {}, to false", i);
                            let end_condition = turtle.search_end(i).unwrap();
                            i = end_condition + 1;
                            continue;
                        }
                    }

                    None => std::process::exit(1),
                }
            }

            constant::IS_CLOSE => {
                // get the start_line from the index, if it's while : check the condition again
                let idx = turtle.get_start_line(i).unwrap();
                println!("idx of start = {}", idx); // 6
                let cond_line = &commands[*idx].as_str().trim_start();
                println!("cond_line = {}", cond_line);
                if &cond_line[..2] == "IF" {
                    i += 1;
                    continue;
                }

                let cond_line_to_vec: Vec<&str> = cond_line.split(' ').collect();

                // check the condition again
                match helper::check_condition(*idx, &cond_line_to_vec, &turtle) {
                    Some(result) => {
                        if result {
                            i = 1 + idx;
                            continue;
                        }
                    }
                    None => std::process::exit(1),
                }
                // condition wrong: go to the end of match
            }

            constant::PROCEDURE_END => {
                // check if the end is in turtle's map
                if !turtle.has_end(i) {
                    std::process::exit(1);
                }
            }

            _ => {
                match turtle.check_function(&splitted[0]) {
                    Some(f) => {
                        println!("function = {:?} exists", f);
                        let mut f_map: HashMap<&str, f32> = HashMap::new();
                        // map all args in func_map

                        // create a new turtle!!!
                        // if !function::parse_func_arguments(turtle, &mut f_map, &f, &splitted) {
                        //     std::process::exit(1);
                        // }

                        // match turtle_move(commands, turtle, img, f.start, f.end, &f_map) {
                        //     Ok(result) => println!("{:?}", result),
                        //     Err(_) => std::process::exit(1),
                        // }
                    }

                    None => std::process::exit(1),
                }
            }
        };

        println!(
            "x = {}, y = {}, dir = {} degree",
            turtle.get_x(),
            turtle.get_y(),
            turtle.get_direction()
        );
        i += 1;
    }

    Ok((turtle.get_x(), turtle.get_y()))
}
