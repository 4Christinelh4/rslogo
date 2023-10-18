use std::collections::HashMap;
use std::collections::VecDeque;

use crate::parser::constant::*;

pub fn get_number_float(in_str: &str) -> Result<f32, std::num::ParseFloatError> {
    match &in_str.parse::<f32>() {
        Ok(num) => Ok(*num),
        Err(parse_error) => Err(parse_error.clone()),
    }
}

pub fn is_arithmetic_operator(argu: &str) -> bool {
    match argu {
        "+" | "-" | "*" | "/" => true,
        _ => false,
    }
}

fn calculate_on_operator(prev_val: &f32, op_name: &str, another_val: &f32) -> Option<f32> {
    match op_name {
        "+" => Some(prev_val + another_val),
        "-" => Some(prev_val - another_val),
        "*" => Some(prev_val * another_val),
        "/" => Some(prev_val / another_val),
        _ => None,
    }
}

// bool = true if it's f32, = false if it's str
pub fn parse_or_search_map(
    var_map: &HashMap<&str, VarMapValue>,
    in_str: &str,
) -> Option<(f32, String, bool)> {
    match in_str.chars().nth(0) {
        Some(':') => {
            // get from var_map
            match var_map.get(&in_str[1..]) {
                Some(val) => {
                    println!("parse_or_search_map: {:?}", val);
                    if val.is_f32 {
                        Some((val.f32_value, String::from(""), true))
                    } else {
                        Some((0.0, val.str_value.clone(), false))
                    }
                }
                None => None,
            }
        }

        Some('"') => match get_number_float(&in_str[1..]) {
            Ok(val) => Some((val, String::from(""), true)),
            Err(_) => None,
        },
        _ => None,
    }
}

// (f32: result of current calculation, usize: start of next operator, aka end of current arithmatic + 1)
pub fn calculate_bystack(
    var_map: &HashMap<&str, VarMapValue>,
    cmd_line: &[&str],
    start_idx: usize,
) -> Option<(f32, usize)> {
    let mut stack: VecDeque<usize> = VecDeque::new();
    let mut k = start_idx;

    let mut flag = false;
    let mut prev: f32 = 0.0;

    while k < cmd_line.len() {
        while k < cmd_line.len() && is_arithmetic_operator(&cmd_line[k]) {
            stack.push_back(k);
            k += 1;
        }

        // if it's number or variable
        if !flag {
            match parse_or_search_map(var_map, &cmd_line[k]) {
                None => return None,
                Some(pair) => {
                    let (v, _, is_f32) = pair;
                    prev = v;
                }
            };
            flag = true;
        } else {
            let idx_ = stack.pop_back();
            match idx_ {
                None => std::process::exit(1),
                Some(op_idx) => {
                    match parse_or_search_map(&var_map, &cmd_line[k]) {
                        None => {
                            return None;
                        }
                        Some(tup) => {
                            let (another_v, _, is_f32) = tup;
                            let another_val: f32 = another_v;

                            match calculate_on_operator(&prev, &cmd_line[op_idx], &another_val) {
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

        if stack.len() == 0 {
            return Some((prev, k + 1 as usize));
        }

        k += 1;
    }

    Some((prev, k as usize))
}
