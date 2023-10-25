use std::collections::VecDeque;

use crate::parser::constant::*;
use crate::parser::turtle::*;

/// when meeting an arithmetic expression, such as +, -... use this function to calculate
/// (f32: result of current calculation, usize: start of next operator, also the last of current arithmatic + 1)
pub fn calculate_bystack(
    turtle: &Turtle,
    cmd_line: &[&str],
    start_idx: usize,
) -> Option<(f32, usize)> {
    let mut stack: VecDeque<usize> = VecDeque::new();
    let mut k = start_idx;

    let mut flag = false;
    let mut prev: f32 = 0.0;
    while k < cmd_line.len() {
        // keep push back to the stack, stop when the last aritimetic operator is pushed back
        while k < cmd_line.len() && is_arithmetic_operator(cmd_line[k]) {
            stack.push_back(k);
            k += 1;
        }

        // search the expression in turtle. check if it's number or variable
        if !flag {
            match turtle.parse_or_search_map(&cmd_line[k]) {
                None => return None,
                Some(pair) => {
                    let (v, _, _is_f32) = pair;
                    prev = v;
                }
            };
            flag = true;
        } else {
            let idx_ = stack.pop_back();
            match idx_ {
                None => std::process::exit(1),
                Some(op_idx) => {
                    match turtle.parse_or_search_map(&cmd_line[k]) {
                        None => {
                            return None;
                        }
                        Some(tup) => {
                            let (another_v, _, _is_f32) = tup;
                            let another_val: f32 = another_v;

                            match calculate_on_operator(&prev, cmd_line[op_idx], &another_val) {
                                Some(calculation_res) => {
                                    prev = calculation_res;
                                }
                                None => {
                                    return None;
                                }
                            };
                        }
                    };
                }
            };
        }

        if stack.is_empty() {
            return Some((prev, k + 1));
        }

        k += 1;
    }

    Some((prev, k))
}

// WHILE AND GT XCOR "0 GT YCOR "0
// ---> return 5 in this case
pub fn parse_end_arg(cond_line: Vec<&str>) -> Option<usize> {
    for i in 3..cond_line.len() {
        match cond_line[i] {
            "EQ" | "NE" | "GT" | "LT" => {
                return Some(i);
            }
            _ => continue,
        }
    }
    None
}
