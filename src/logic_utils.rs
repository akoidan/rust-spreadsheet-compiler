use std::collections::VecDeque;
use std::fmt::Pointer;

use crate::table::*;
use crate::regex_helpers::*;
use crate::io_utils::*;

fn remove_first_symbol<'a>(s: &'a str) -> &'a str {
    return &s[1..s.len()];
}

pub fn extract_head_name(column: &str) -> &str {
    return remove_first_symbol(column);
}

pub fn is_head(col: &str) -> bool {
    return col.starts_with("!");
}

enum Item {
    Literal(String),
    Token(String),
    Operator(char),
    ZoneStart(char),
    ZoneEnd(char),
}

pub trait LogicExecutor {
    fn parse_string(&self, s: String, index: u32, name: String) -> String;
    fn get_command(s: &str) -> Command;
    fn execute_str(&self, s: &str, index: u32, name: String) -> String;
    fn fill_data(&mut self);
    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>);
    fn calc_function(&self, name: &str, args: &[String]) -> String;
    fn revaluate_from_literal(&self,stack: &mut VecDeque<Item>);
    fn get_matching_start_zone(&self, item: Item) -> char;
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

    fn get_matching_start_zone(&self, item: Item) -> char {
        let start_zone_value = match item {
            Item::ZoneEnd(c) => match c {
                '>' => '<',
                ')' => '(',
                _ => panic!("Invalid zone_end char"),
            },
            _ => panic!("Expected zone_end"),
        };
        return start_zone_value;
    }

    fn revaluate_from_literal(&self, stack: &mut VecDeque<Item>) {

    }

    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>) {
        let item = stack.pop_back().unwrap();
        let mut operands = Vec::new();
        match item {
            Item::Literal(val) => {
                // ...
            }
            Item::ZoneEnd(val) => {
                let start_zone_value = self.get_matching_start_zone(item);
                loop {
                    if stack.is_empty() {
                        panic!("WTF");
                    }

                    let item_inner = stack.pop_back().unwrap();
                    match item_inner {
                        Item::Literal(val) => operands.push(val),
                        Item::ZoneStart(c) if c == start_zone_value => {
                            if let Some(name_item) = stack.pop_back() {
                                match name_item {
                                    Item::Token(name) => {
                                        let res = self.calc_function(&name, &operands);
                                        stack.push_back(Item::Literal(res));
                                    }
                                    Item::Operator(op) => {
                                        if let Some(name_2_item) = stack.pop_back() {
                                            match name_2_item {
                                                Item::Literal(val) => {
                                                    operands.push(val);
                                                    let res =
                                                        self.calc_function(&op.to_string(), &operands);
                                                    stack.push_back(Item::Literal(res));
                                                    operands.clear();
                                                }
                                                _ => panic!("WTF"),
                                            }
                                        }
                                    }
                                    _ => panic!("WTF"),
                                }
                            }
                            break;
                        }
                        Item::Operator(op) => {
                            if let Some(name_item) = stack.pop_back() {
                                match name_item {
                                    Item::Literal(val) => {
                                        operands.push(val);
                                        let res = self.calc_function(&op.to_string(), &operands);
                                        stack.push_back(Item::Literal(res));
                                        operands.clear();
                                    }
                                    _ => panic!("WTF"),
                                }
                            }
                        }
                        _ => panic!("WTF"),
                    }
                }

            }
            _ => panic!("Unsupported type of expression"),
        }

        loop {

        }
    }

    fn calc_function(&self, name: &str, args: &[String]) -> String {
        format!("[{}({})]", name, args.join(","))
    }

    fn execute_str(&self, s: &str, index: u32, name: String) -> String {
        let mut stack: VecDeque<Item> = VecDeque::new();
        let mut i = 0;

        while i < s.len() {
            match &s[i..=i] {
                c if c.at(0).is_uppercase() && !s.at(i+1).is_alphabetic() => {
                        if s.at(i+1).is_ascii_digit() {
                            stack.push_back(Item::Literal(String::from("asd")));
                            i += 2;
                        } else if &s[i+1..=i+2] == "^v" {
                            stack.push_back(Item::Literal(String::from("asd")));
                            i += 3;
                        } else if &s[i+1..=i+1] == "^" {
                            i += 2;
                            stack.push_back(Item::Literal(String::from("asd")));
                        } else {
                            panic!("Unsupported structure for value {}", &s[i+1..=i+1]);
                        }
                }
                c if c.chars().all(|c| c.is_ascii_alphabetic()) => {
                    if let Some(match_len) = s[i + 1..].find(|c: char| !c.is_ascii_alphabetic()) {
                        let value = &s[i..i + match_len + 1];
                        stack.push_back(Item::Token(value.to_string()));
                        i += match_len + 1;
                    } else {
                        panic!("WTF");
                    }
                }
                c if c.chars().all(|c| c.is_ascii_digit()) => {
                    if let Some(match_len) = s[i + 1..].find(|c: char| !c.is_ascii_digit()) {
                        let value = &s[i..i + match_len + 1];
                        stack.push_back(Item::Literal(value.to_string()));
                        i += match_len + 1;
                    } else {
                        panic!("WTF");
                    }
                }
                "(" | "<" => {
                    let c = &s[i..=i].chars().next().unwrap();
                    stack.push_back(Item::ZoneStart(*c));
                    i += 1;
                }
                ")" | ">" => {
                    let c = &s[i..=i].chars().next().unwrap();
                    stack.push_back(Item::ZoneEnd(*c));
                    i += 1;
                    self.revaluate_from_end_zone(&mut stack);
                }
                "\"" => {
                    if let Some(value) = s[i + 1..].find('\"') {
                        let val = &s[i..i + value + 2];
                        stack.push_back(Item::Literal(val.to_string()));
                        i += value + 2;
                    } else {
                        panic!("WTF");
                    }
                }
                "@" => {
                    if let Some(match_len) =
                        s[i + 1..].find(|c: char| c == '_' || c.is_ascii_alphabetic())
                    {
                        let value = &s[i..i + match_len + 2];
                        stack.push_back(Item::Token(value.to_string()));
                        i += match_len + 2;
                    } else {
                        panic!("WTF {i}");
                    }
                }
                " " | "," => {
                    i += 1;
                }
                op if ['+', '*', '-', '/'].contains(&op.chars().next().unwrap()) => {
                    stack.push_back(Item::Operator(op.chars().next().unwrap()));
                    i += 1;
                }
                _ => panic!("Unkown symbol {} at position {}", &s[i..i + 1], i),
            }
        }

        if stack.len() > 1 {
            panic!("Lacking end statement");
        }

        match stack.pop_back().unwrap() {
            Item::Literal(val) => {
                return val;
            }
            _ => panic!("WTF"),
        }
    }

    fn parse_string(&self, s: String, index: u32, name: String) -> String {
        let mut chars = s.chars();
        let first_char = chars.nth(0).unwrap();
        if first_char == '=' {
            return self.execute_str(remove_first_symbol(&s), index, name);
        } else {
            return String::from(s);
        }
    }

    fn fill_data(&mut self) {
        for i in 0..self.columns.len() {
            let mut newKeys: Vec<u32> = self.columns[i]
                .values
                .keys()
                .map(|x| x.clone())
                .collect::<Vec<u32>>();
            newKeys.sort();

            let mut prev_val: String = "".to_string();
            for key in newKeys {
                let cell: &String = self.columns[i].values.get(&key).unwrap();

                if cell == "=^^" {
                    let replaced_prev_values_str = increase_column_digits(prev_val.clone(), key - 1);
                    let calculated_data = self.parse_string(
                        replaced_prev_values_str,
                        key,
                        String::from(&self.columns[i].name)
                    );
                    self.columns[i].values.insert(12, calculated_data);
                } else {
                    prev_val = String::from(cell);
                    let calculated_data = self.parse_string(
                        String::from(cell),
                        key,
                        String::from(&self.columns[i].name)
                    );
                    self.columns[i].values.insert(12, calculated_data);
                }
            }
        }
    }
}
