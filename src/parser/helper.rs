use crate::parser::compute::*;
use crate::parser::turtle::*;

pub fn is_comment(line: &str) -> bool {
    return line.len() >= 2 && line.chars().nth(0) == Some('/') && line.chars().nth(1) == Some('/');
}

pub fn is_i32(input: f32) -> bool {
    match input {
        x if x as i32 as f32 == x => true, // Check if it's an i32 or a float with no fractional part
        _ => false,
    }
}

// parse_value for setX, setY, FORWARD..., setcolore, set_heading, trun must have f32!!!!
// bool = true: f32, bool = false: str
pub fn parse_value(
    turtle: &Turtle,
    params: &[&str],
    start_idx: usize,
) -> Option<(f32, String, usize, bool)> {
    if params.len() > 1 && !is_arithmetic_operator(&params[start_idx]) {
        return None;
    }

    if is_arithmetic_operator(&params[0]) {
        // Option<(f32, usize)>
        match calculate_bystack(turtle.get_varmap(), params, start_idx) {
            Some(ret) => {
                println!("**** ret = {:?} ****", ret);
                Some((ret.0, String::from(""), ret.1, true))
            }
            None => None,
        }

        // (ret.0, "", ret.1, true)
    } else {
        // Option<(f32, String, bool)>
        match parse_or_search_map(turtle.get_varmap(), &params[0]) {
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

// MAKE :xyz * 2 6
// MAKE "X XCOR
// MAKE "Y YCOR
pub fn make_cmd<'a, 'b: 'a>(turtle: &'a mut Turtle<'b>, params: &[&'b str]) -> Option<bool> {
    let k: &str = &params[0][1..];

    // params [1] -> variable
    // insert_varmap(k: &str, is_f32: bool, f32_val: f32, str_value: String)
    match params[1] {
        "XCOR" | "YCOR" | "COLOR" | "HEADING" => {
            turtle.insert_varmap(k, true, turtle.make_query(&params[1]), String::from(""));
        }
        _ => {
            match parse_value(&turtle, &params, 1) {
                Some(res) => {
                    //  what to insert!!!
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
    Some(true)
}

// // check if it's valid (closed) at the beginning
// pub fn add_controlflow_to_map(idx: i32, commands: & Vec<String>
//     , end_start_map: &mut HashMap<i32, i32>
//     , cond_map_valid: &mut HashMap<i32, Condition> ) {

//     let mut stack: VecDeque<i32> = VecDeque::new();
//     let cmd_len: i32 =  commands.len() as i32;

//     for k in idx..cmd_len {
//         let line_ref = commands[k as usize].as_str();
//         if line_ref.len() == 0 || is_comment(line_ref) {
//             continue;
//         }

//         let splitted: Vec<&str> = line_ref.split(' ').collect();

//         match splitted[0] {
//             IS_WHILE | IS_IF => {
//                 stack.push_back(k);
//             },

//             IS_CLOSE => {
//                 // println!("IS_CLOSE {}", k);
//                 if stack.len() == 0 {
//                     // error
//                     std::process::exit(1);
//                 }

//                 let start_idx = stack.pop_back();
//                 // let start_idx_: i32 = 0;

//                 // println!("{:?}", start_idx);

//                 match start_idx {
//                     // put the cond into condition map
//                     // EQ XCOR "10
//                     Some(start_idx_) => {

//                         // cmd_condition needs to be check calculate_by_stack
//                         // IF EQ * "6 "2 "12
//                         // WHILE AND GT XCOR "0 GT YCOR "0  -> cmd_condition
//                         let cmd_condition: Vec<&str>  = commands[start_idx_ as usize].as_str().split(' ').collect();

//                         cond_map_valid.insert(start_idx_, Condition{
//                             operator_name: String::from(cmd_condition[0]),
//                             cond_name: String::from(cmd_condition[1]),
//                             lhs: String::from(cmd_condition[2]),
//                             rhs: String::from(cmd_condition[3]),
//                         } );

//                         // println!("{}, {}", k, start_idx_);
//                         end_start_map.insert(k, start_idx_);
//                     },

//                     None => std::process::exit(1)
//                 }

//             },

//             _ =>  { continue; }
//         }
//     }

//     if stack.len() > 0 {
//         // error -> no close -> process exit(1)
//         println!("Error: stack len > 0!");
//         std::process::exit(1);
//     }
// }
