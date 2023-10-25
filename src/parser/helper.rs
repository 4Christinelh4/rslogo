use crate::parser::compute::*;
use crate::parser::constant::*;
use crate::parser::turtle::*;

use std::collections::VecDeque;

/// parse_value for setX, setY, FORWARD..., setcolore, set_heading, trun must have f32!!!!
/// bool = true: f32, bool = false: str
/// the usize (3rd) is the index of the start of the next expression, for example, EQ <expr1> <expr2>
/// after parsing expr1, usize is the index of start of expr2
pub fn parse_value(
    turtle: &Turtle,
    params: &[&str],
    start_idx: usize,
) -> Option<(f32, String, usize, bool)> {
    if params.len() == 1 {
        return None; // nothing to parse, the first &str is the command itself
    }

    if is_arithmetic_operator(params[start_idx]) {
        // Option<(f32, usize)>
        match calculate_bystack(turtle, params, start_idx) {
            Some(ret) => Some((ret.0, String::from(""), ret.1, true)),
            None => None,
        }
    } else {
        // Option<(f32, String, bool)>
        match turtle.parse_or_search_map(&params[start_idx]) {
            Some(result) => {
                if result.2 {
                    // f32
                    Some((result.0, String::from(""), 1 + start_idx, result.2)) // if is_str -> None
                } else {
                    Some((0.0, result.1, 1 + start_idx, result.2))
                }
            }
            None => None,
        }
    }
}

/// this is the helper for MAKE command of the turtle. As the last expression of MAKE can be a string or a number,
/// for number, use f32, for string, use String
/// it inserts to turtle's varmap by calling turtle.insert_varmap, meanwhile, the type (number or string)
/// is indicated when inserting
/// MAKE :xyz * 2 6
pub fn make_cmd<'a, 'b: 'a>(turtle: &'a mut Turtle<'b>, params: &[&'b str]) -> Option<()> {
    if (params.len() > 3 && !is_arithmetic_operator(&params[2])) || params.len() < 3 {
        return None;
    }

    let k: &str = &params[1][1..];

    // params [1] -> variable
    // insert_varmap(k: &str, is_f32: bool, f32_val: f32, str_value: String)
    match params[2] {
        "XCOR" | "YCOR" | "COLOR" | "HEADING" => {
            turtle.insert_varmap(
                k,
                true,
                turtle.make_query(&params[2]).unwrap(),
                String::from(""),
            );
        }
        _ => {
            match parse_value(&turtle, &params, 2) {
                Some(res) => {
                    // println!("{}, {}: insert [{}, {:?}]", file!(), line!(), k, res);
                    if res.3 {
                        // f32
                        turtle.insert_varmap(k, true, res.0, String::from(""));
                    } else {
                        turtle.insert_varmap(k, false, 0.0, res.1);
                    }
                }

                None => {
                    return None;
                }
            };
        }
    };
    Some(())
}

// this is the helper for ADDASSIGN. It checks if the key is in the var_map, return None if not
// it also check the value to be addassign is a number by checking the boolean of the value taken from
// the var_map. If it's a string, return None.
// ADDASSIGN "DIST "5
pub fn add_assign<'a, 'b: 'a>(turtle: &'a mut Turtle<'b>, params: &[&'b str]) -> Option<()> {
    // make sure it's in turtle's var map and it's f32!!!!
    // match turtle.
    if (params.len() > 3 && !is_arithmetic_operator(&params[2])) || params.len() < 3 {
        return None;
    }

    match turtle.search_assign(&params[1][1..]) {
        Some(float_result) => {
            match parse_value(&turtle, &params, 2) {
                Some(res) => {
                    if res.3 {
                        // f32
                        turtle.insert_varmap(
                            &params[1][1..],
                            true,
                            float_result + res.0,
                            String::from(""),
                        );
                        return Some(());
                    } else {
                        return None;
                    }
                }

                None => {
                    return None;
                }
            };
        }
        None => None,
    }
}

/// Make sure a while/ if, which has multiple lines, is valid at the beginning.
/// check if it's valid (closed) by looking for the "]". It uses a stack to keep the index of a line that starts
/// with if or while to handle nested if or while. It pops from the stack once there is a
pub fn add_controlflow(idx: usize, commands: &Vec<String>, turtle: &mut Turtle) -> Option<()> {
    let mut stack: VecDeque<usize> = VecDeque::new();
    let cmd_len = commands.len();

    for k in idx..cmd_len {
        let line_ref = commands[k].as_str().trim_start();
        if line_ref.is_empty() || is_comment(line_ref) {
            continue;
        }

        let splitted: Vec<&str> = line_ref.split(' ').collect();

        match splitted[0] {
            IS_WHILE | IS_IF => {
                stack.push_back(k);
            }

            IS_CLOSE => {
                let stack_back = stack.pop_back();
                match stack_back {
                    // put the cond into condition map
                    // IF EQ XCOR "10
                    // WHILE AND GT XCOR "0 GT YCOR "0  -> cmd_condition
                    Some(start_idx) => {
                        // get the line of cond
                        let cond_line: Vec<&str> = commands[start_idx]
                            .as_str()
                            .trim_start()
                            .split(' ')
                            .collect();

                        let mut cond_1: Condition = Condition {
                            assigned_true: false,
                            cond_start: 2, // the way to evaluate lhs, rhs
                        };

                        let mut cond_2: Condition = Condition {
                            assigned_true: false,
                            cond_start: 2,
                        };

                        match cond_line[1] {
                            "AND" | "OR" => {
                                let end_cond_1 = parse_end_arg(cond_line);
                                match end_cond_1 {
                                    Some(end_index) => cond_2.cond_start = end_index,
                                    None => return None,
                                }
                            }

                            // one condition!!!
                            "EQ" | "NE" | "GT" | "LT" => {
                                cond_1.cond_start = 1;
                                cond_2.assigned_true = true;
                            }

                            _ => return None,
                        };

                        // println!(
                        //     "cond1 = {:?}, cond2 = {:?}, end = {}, start = {}",
                        //     cond_1, cond_2, k, start_idx
                        // );
                        turtle.add_2conditions(start_idx, cond_1, cond_2);
                        turtle.insert_condmap(k, start_idx);
                    }
                    None => return None, // empty stack!!!
                }

                if stack.is_empty() {
                    // error
                    Some(());
                }
            }

            _ => continue,
        }
    }

    if !stack.is_empty() {
        return None;
    }

    Some(())
}

/// evaluate a single condition's LHS and RHS
/// when LHS and RHS has different types, NE returns TRUE, GT/LT/EQ returns FALSE
pub fn evaluate_cond(turtle: &Turtle, cond: &Condition, params: &Vec<&str>) -> Option<bool> {
    if cond.assigned_true {
        return Some(true);
    }

    let lhs = parse_value(&turtle, &params, 1 + cond.cond_start);
    let correct_lhs: (f32, String, usize, bool) = if lhs.is_some() {
        lhs.unwrap()
    } else {
        return None;
    };

    let rhs_start: usize = correct_lhs.2;

    let rhs = parse_value(&turtle, &params, rhs_start);
    let correct_rhs: (f32, String, usize, bool) = if rhs.is_some() {
        rhs.unwrap()
    } else {
        return None;
    };

    match params[cond.cond_start] {
        "EQ" => {
            if correct_lhs.3 && correct_rhs.3 {
                return Some(correct_lhs.0 == correct_rhs.0);
            }
            if !correct_lhs.3 && !correct_rhs.3 {
                return Some(correct_lhs.1 == correct_rhs.1);
            }
            Some(false)
        }
        "GT" => return Some(correct_lhs.3 && correct_rhs.3 && correct_lhs.0 > correct_rhs.0),
        "LT" => return Some(correct_lhs.3 && correct_rhs.3 && correct_lhs.0 < correct_rhs.0),
        "NE" => {
            if correct_lhs.3 && correct_rhs.3 {
                return Some(correct_lhs.0 != correct_rhs.0);
            }

            if !correct_lhs.3 && !correct_rhs.3 {
                return Some(correct_lhs.1 != correct_rhs.1);
            }

            Some(true) // invalid compare (f32 & string), but it's still NOT EQUAL by definition
        }
        _ => return None,
    }
}

/// it uses the result of evaluate_cond. returns true or false based on AND/ OR rules.
pub fn check_condition(line_idx: usize, params: &Vec<&str>, turtle: &Turtle) -> Option<bool> {
    match turtle.get_conds(line_idx) {
        Some(conds) => {
            let first_cond: &Condition = &conds.0;
            let second_cond: &Condition = &conds.1; // need to find the start of secod cond before

            let connect: &str = params[1];
            // // let cond1_bool: bool;
            // let cond2_bool: bool;

            let cond1_bool: bool = match evaluate_cond(&turtle, &first_cond, &params) {
                None => return None,
                Some(res) => res,
            };

            let cond2_bool: bool = match evaluate_cond(&turtle, &second_cond, &params) {
                None => return None,
                Some(res) => res,
            };

            match connect {
                "AND" => Some(cond1_bool && cond2_bool),
                "OR" => Some(cond1_bool || cond2_bool),
                "GT" | "LT" | "NE" | "EQ" => Some(cond1_bool),
                _ => None,
            }
        }

        None => None,
    }
}
