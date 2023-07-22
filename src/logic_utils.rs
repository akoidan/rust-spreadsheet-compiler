use std::collections::VecDeque;

use crate::table::*;

pub trait LogicExecutor { 
    fn parse_string(&mut self, s: &str, index: u32, name: &str) -> &str;
}

impl LogicExecutor for TableData {
    fn parse_string<'a>(&'a mut self, s: &'a str, index: u32, name: &'a str) -> &'a str {
        let mut chars = s.chars();
        let first_char = chars.nth(0).unwrap();
        if first_char == '=' {
            return self.parse_string(remove_first_symbol(s), index, name);
        } else if (first_char >= 'A') && (first_char <= 'Z') {
            if (chars.nth(1).unwrap() ==  '^') && (chars.nth(2).unwrap() ==  'v') {
                return "asd";
            }
            return "asdas"
        } else {
            return s;
        }
    }
}

fn remove_first_symbol(s: &str) -> &str {
    return &s[1..s.len()];
}

pub fn extract_head_name(column:  &str) -> &str {
    return remove_first_symbol(column);
}


pub fn is_head(col: &str) -> bool {
    return col.starts_with("!");
}






