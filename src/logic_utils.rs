use std::collections::VecDeque;

use crate::table::*;

fn remove_first_symbol<'a>(s: &'a str) -> &'a str {
    return &s[1..s.len()];
}

pub fn extract_head_name(column:  &str) -> &str {
    return remove_first_symbol(column);
}

pub fn is_head(col: &str) -> bool {
    return col.starts_with("!");
}

pub trait LogicExecutor { 
    fn parse_string(&self, s: String, index: u32, name: String) -> String;
    fn get_command(s: &str) -> Command;
    fn execute_str(&self, s: String, index: u32, name: String) -> String;
    fn fill_data(&mut self);
}

struct Command {
    operands: Vec<String>,
    operator: String,
}

impl LogicExecutor for TableData {
    fn get_command(s: &str) -> Command {
        let operands = vec![];
        let c = Command {
            operands,
            operator: String::from("asd"),
        };
        return c;
    }

    fn execute_str(&self, s: String, index: u32, name: String) -> String {
        return String::from(s);
    }
    fn parse_string(&self, s: String, index: u32, name: String) -> String {
        let mut chars = s.chars();
        let first_char = chars.nth(0).unwrap();
        if first_char == '=' {
            return self.execute_str(String::from(remove_first_symbol(&s)), index, name);
        } else {
            return String::from(s);
        }
    }


    fn fill_data(&mut self) {
        for i in 0..self.columns.len() {
            let mut newKeys: Vec<u32> = self.columns[i].values.keys().map(|x| x.clone()).collect::<Vec<u32>>();
            newKeys.sort();

            for key in newKeys { 
                let cell: &String = self.columns[i].values.get(&key).unwrap();
                let calculated_data: String = self.parse_string(String::from(cell), key, String::from(&self.columns[i].name));
                self.columns[i].values.insert(12, calculated_data);
            }
            
        }

    }
}







