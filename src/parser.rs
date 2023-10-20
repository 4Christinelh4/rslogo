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
) -> Result<(f32, f32, i32, i32), Box<dyn Error>> {
    // dir = 0: up
    // 360 - dir
    let mut i = start_execute;
    while i < finish_execute {
        // println!("{}", commands[i]);
        let line_ref = commands[i].as_str().trim_start();
        if line_ref.is_empty() || constant::is_comment(line_ref) {
            i += 1;
            continue;
        }

        let splitted: Vec<&str> = line_ref.split(' ').collect();

        match splitted[0] {
            constant::DEFINE_PROCEDURE => {
                // need to loop through all commands to find the end to make sure the procedure has the end
                match function::define_procedure(turtle, commands, i) {
                    Ok(last_line) => {
                        // move i to 1 + last_line
                        i = 1 + last_line;
                        // println!("define procedure, last_line = {}", last_line);
                        continue;
                    }
                    Err(_) => std::process::exit(1),
                }
            }

            // setpencolor [start of expression] + only 1 value
            constant::IS_SETPENCOLOR | constant::IS_TURN | constant::IS_SETHEADING => {
                if splitted.len() > 2 && !constant::is_arithmetic_operator(splitted[1])
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
                if !constant::is_i32(val) {
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
                if (splitted.len() > 2 && !constant::is_arithmetic_operator(&splitted[1]))
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
                // println!("i = {}", i);

                match turtle.search_end(i) {
                    None => match helper::add_controlflow(i, commands, turtle) {
                        None => std::process::exit(1),
                        Some(_) => {}
                    },
                    Some(_) => {}
                };

                // println!("i = {} turtle_search end = {:?}", i, turtle.search_end(i));

                match helper::check_condition(i, &splitted, &turtle) {
                    Some(res) => {
                        if res {
                            i += 1;
                            continue;
                        } else {
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
                // we already make sure that the loop is valid
                let idx = turtle.get_start_line(i).unwrap();
                let cond_line = &commands[*idx].as_str().trim_start();
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
                        // create a new turtle and put all args in the var_map!
                        // the turtle is like a stack pointer here

                        let mut new_turtle = turtle::Turtle::new(
                            turtle.get_x(),
                            turtle.get_y(),
                            turtle.get_color(),
                            turtle.get_direction(),
                        );

                        match function::parse_func_arguments(
                            &mut new_turtle,
                            &f,
                            &splitted,
                            &turtle,
                        ) {
                            None => std::process::exit(1),
                            Some(_) => {}
                        };

                        // let mut input = String::new();
                        // io::stdin().read_line(&mut input)
                        //     .expect("Failed to read line");

                        if turtle.get_pen_status() {
                            new_turtle.set_pendown();
                        }

                        // println!("call the function: {:?}", f);
                        match turtle_move(commands, &mut new_turtle, img, 1 + f.start, f.end) {
                            Ok(result) => {
                                // println!("{:?}", result);

                                turtle.set_x(result.0);
                                turtle.set_y(result.1);
                                turtle.set_color(result.2);
                                turtle.set_heading(result.3);
                            }
                            Err(_) => std::process::exit(1),
                        }
                    }

                    None => std::process::exit(1),
                }
            }
        };

        i += 1;
    }

    Ok((
        turtle.get_x(),
        turtle.get_y(),
        turtle.get_color(),
        turtle.get_direction(),
    ))
}
