pub const IS_PENDOWN: &str = "PENDOWN";
pub const IS_PENUP: &str = "PENUP";

pub const IS_SETX: &str = "SETX";
pub const IS_SETY: &str = "SETY";

pub const IS_FORWARD: &str = "FORWARD";
pub const IS_BACK: &str = "BACK";
pub const IS_RIGHT: &str = "RIGHT";
pub const IS_LEFT: &str = "LEFT";

pub const IS_TURN: &str = "TURN";
pub const IS_SETHEADING: &str = "SETHEADING";

pub const IS_MAKE: &str = "MAKE";

pub const IS_SETPENCOLOR: &str = "SETPENCOLOR";
pub const IS_ADDASSIGN: &str = "ADDASSIGN";

pub const IS_CLOSE: &str = "]";
pub const IS_WHILE: &str = "WHILE";
pub const IS_IF: &str = "IF";

pub const DEFINE_PROCEDURE: &str = "TO";
pub const PROCEDURE_END: &str = "END";

#[derive(Debug, Clone)]
pub struct VarMapValue {
    pub is_f32: bool,
    pub f32_value: f32,
    pub str_value: String,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub num_args: i32,
    pub start: usize, // the line where the function starts in logo file
    pub end: usize,
    pub argv: String, // 'arg1 'arg2 ..String
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub assigned_true: bool,
    pub cond_start: usize, // EQ/GT/LT/NE
}

pub fn get_number_float(in_str: &str) -> Result<f32, std::num::ParseFloatError> {
    match &in_str.parse::<f32>() {
        Ok(num) => Ok(*num),
        Err(parse_error) => Err(parse_error.clone()),
    }
}

pub fn is_comment(line: &str) -> bool {
    return line.len() >= 2 && line.chars().nth(0) == Some('/') && line.chars().nth(1) == Some('/');
}

pub fn is_i32(input: f32) -> bool {
    match input {
        x if x as i32 as f32 == x => true, // Check if it's an i32 or a float with no fractional part
        _ => false,
    }
}

pub fn calculate_on_operator(prev_val: &f32, op_name: &str, another_val: &f32) -> Option<f32> {
    match op_name {
        "+" => Some(prev_val + another_val),
        "-" => Some(prev_val - another_val),
        "*" => Some(prev_val * another_val),
        "/" => Some(prev_val / another_val),
        _ => None,
    }
}

pub fn is_arithmetic_operator(argu: &str) -> bool {
    match argu {
        "+" | "-" | "*" | "/" => true,
        _ => false,
    }
}
