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
    pub start: usize,
    pub end: usize,
    pub argv: String,
}
