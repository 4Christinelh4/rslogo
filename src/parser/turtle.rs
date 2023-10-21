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
    // key: the line where the cond ends, value: the line where the cond starts
    cond_map: HashMap<usize, usize>,
    conditions: HashMap<usize, (Condition, Condition)>,
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
            cond_map: HashMap::new(),
            conditions: HashMap::new(),
        }
    }

    pub fn insert_condmap(&mut self, k: usize, v: usize) {
        self.cond_map.insert(k, v);
    }

    // TODO: change to Box???
    pub fn add_2conditions(&mut self, start_idx: usize, cond_1: Condition, cond_2: Condition) {
        self.conditions.insert(start_idx, (cond_1, cond_2));
    }

    pub fn get_conds(&self, line_idx: usize) -> Option<&(Condition, Condition)> {
        self.conditions.get(&line_idx)
    }

    pub fn search_end(&self, start: usize) -> Option<usize> {
        for (key, value) in &self.cond_map {
            if *value == start {
                return Some(*key);
            }
        }
        None
    }

    pub fn get_start_line(&self, close_line: usize) -> Option<&usize> {
        return self.cond_map.get(&close_line);
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

    pub fn get_pen_status(&self) -> bool {
        self.pen_down
    }

    pub fn check_function(&self, name: &'a str) -> Option<&Func> {
        self.func_map.get(&name)
    }

    // return false if the "end" is not in Funcmap
    pub fn has_end(&self, line_idx: usize) -> bool {
        false
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

    pub fn insert_funcmap<'b: 'a>(
        &mut self,
        f_name: &'b str,
        start_line: usize,
        end_line: usize,
        num_args: i32,
        argv_list: String,
    ) {
        let new_func: Func = Func {
            num_args: num_args,
            start: start_line,
            end: end_line,
            argv: argv_list,
        };

        // println!("turtle: new function is inserted{:?}", new_func);
        self.func_map.insert(f_name, new_func);
    }

    pub fn make_query(&self, q: &str) -> Option<f32> {
        match q {
            "XCOR" => Some(self.get_x()),
            "YCOR" => Some(self.get_y()),
            "COLOR" => Some(self.get_color() as f32),
            "HEADING" => Some(self.get_direction() as f32),
            _ => None,
        }
    }

    // for add assign only, it only allows f32
    pub fn search_assign(&self, key_name: &str) -> Option<f32> {
        match &self.var_map.get(&key_name) {
            None => None,
            Some(val) => {
                if !val.is_f32 {
                    return None;
                }

                Some(val.f32_value)
            }
        }
    }

    pub fn search_varmap(&self, in_str: &str) -> Option<(f32, String, bool)> {
        let var_map: &HashMap<&str, VarMapValue> = &self.var_map;
        match in_str.chars().nth(0) {
            Some(':') => {
                // get from var_map
                match var_map.get(&in_str[1..]) {
                    Some(val) => {
                        // println!("{}, {}: val = {:?}", file!(), line!(), val);
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
                Err(_) => Some((0.0, (&in_str[1..]).to_string(), false)),
            },
            _ => None,
        }
    }

    pub fn moving(
        &mut self,
        len: f32,
        command_dir: &str,
        img: &mut unsvg::Image,
    ) -> Option<(f32, f32)> {
        let mut dir_map: HashMap<&str, i32> = HashMap::new();

        // reset all values
        dir_map.insert(IS_FORWARD, self.heading); // -180
        dir_map.insert(IS_BACK, 180 + self.heading); // 180
        dir_map.insert(IS_RIGHT, 90 + self.heading);
        dir_map.insert(IS_LEFT, 270 + self.heading);

        //  -180 + is back = 540 = 180
        let direction_degree = dir_map.get(command_dir).expect("there must be the key");

        if self.pen_down {
            match img.draw_simple_line(
                self.get_x(),
                self.get_y(),
                *direction_degree,
                len,
                unsvg::COLORS[self.get_color() as usize],
            ) {
                Ok(end_point) => {
                    self.set_x(end_point.0);
                    self.set_y(end_point.1);
                }

                Err(_) => return None,
            }
        } else {
            let end_point =
                unsvg::get_end_coordinates(self.get_x(), self.get_y(), *direction_degree, len);
            self.set_x(end_point.0);
            self.set_y(end_point.1);
        }

        Some((self.get_x(), self.get_y()))
    }

    pub fn parse_or_search_map(&self, in_str: &str) -> Option<(f32, String, bool)> {
        match self.make_query(in_str) {
            Some(value_f32) => return Some((value_f32, String::from(""), true)),
            None => {}
        };

        self.search_varmap(&in_str)
    }
}
