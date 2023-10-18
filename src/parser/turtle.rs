use crate::parser::constant::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Turtle<'b> {
    x: f32,
    y: f32,
    color_idx: i32,
    heading: i32,
    pen_down: bool,
    var_map: HashMap<&'b str, VarMapValue>,
    func_map: HashMap<&'b str, Func>,
}

// Implement methods for the class
impl<'a> Turtle<'a> {
    // Constructor method (associated function)
    pub fn new(x: f32, y: f32, color_idx: i32, heading: i32) -> Self {
        Turtle {
            x: x,
            y: y,
            color_idx: color_idx,
            heading: heading,
            pen_down: false,
            var_map: HashMap::new(),
            func_map: HashMap::new(),
        }
    }

    pub fn set_pendown(&mut self) {
        self.pen_down = true;
    }

    pub fn set_penup(&mut self) {
        self.pen_down = false;
    }

    pub fn set_x(&mut self, new_x: f32) {
        self.x = new_x;
    }

    pub fn set_y(&mut self, new_y: f32) {
        self.y = new_y;
    }

    pub fn set_color(&mut self, new_idx: i32) {
        self.color_idx = new_idx;
    }

    pub fn set_heading(&mut self, new_heading: i32) {
        self.heading = new_heading;
    }

    pub fn turn(&mut self, new_heading: i32) {
        self.heading += new_heading;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_direction(&self) -> i32 {
        self.heading
    }

    pub fn get_color(&self) -> i32 {
        self.color_idx
    }

    pub fn check_function(&self, name: &'a str) -> Option<&Func> {
        return self.func_map.get(&name);
    }

    pub fn insert_varmap<'b: 'a>(
        &mut self,
        k: &'b str,
        is_f32: bool,
        f32_val: f32,
        str_value: String,
    ) {
        self.var_map.insert(
            k,
            VarMapValue {
                is_f32: is_f32,
                f32_value: f32_val,
                str_value: str_value,
            },
        );
    }

    pub fn make_query(&self, q: &str) -> f32 {
        match q {
            "XCOR" => self.get_x(),
            "YCOR" => self.get_y(),
            "COLOR" => self.get_color() as f32,
            "HEADING" => self.get_direction() as f32,
            _ => 0.0,
        }
    }

    pub fn get_varmap(&self) -> &HashMap<&str, VarMapValue> {
        &self.var_map
    }

    pub fn moving(&mut self, len: f32, command_dir: &str, img: &mut unsvg::Image) -> 
        Option<(f32, f32)> { 
        
        let mut dir_map: HashMap<&str, i32> = HashMap::new();

        // reset all values
        dir_map.insert(IS_FORWARD, self.heading); // -180
        dir_map.insert(IS_BACK, 180 + self.heading);  // 180
        dir_map.insert(IS_RIGHT, 90 + self.heading);
        dir_map.insert(IS_LEFT, 270 + self.heading);
        
        //  -180 + is back = 540 = 180
        let direction_degree = dir_map.get(command_dir).expect("there must be the key");

        if self.pen_down {
            match img.draw_simple_line(self.get_x(), self.get_y(), *direction_degree, len, unsvg::COLORS[self.get_color() as usize]) {
                Ok(end_point) => {
                    self.set_x(end_point.0);
                    self.set_y(end_point.1);
                },

                Err(_) => return None,
            }

        } else {
            let end_point = unsvg::get_end_coordinates(self.get_x(), self.get_y(), *direction_degree, len);
            self.set_x(end_point.0);
            self.set_y(end_point.1);
        }

        Some((self.get_x(), self.get_y()))
    }
}
