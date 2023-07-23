use std::collections::VecDeque;

use crate::table::*;
// use regex::Regex;

use substring::Substring;

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

    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>) {
        let item = stack.pop_back().unwrap();
        let mut operands = Vec::new();
        let start_zone_value = match item {
            Item::ZoneEnd(c) => match c {
                '>' => '<',
                ')' => '(',
                _ => panic!("Invalid zone end value"),
            },
            _ => panic!("Invalid zone end item"),
        };

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

    fn calc_function(&self, name: &str, args: &[String]) -> String {
        format!("[{}({})]", name, args.join(","))
    }

    fn execute_str(&self, s: &str, index: u32, name: String) -> String {
        let mut stack: VecDeque<Item> = VecDeque::new();
        let mut i = 0;

        while i < s.len() {
            match &s[i..=i] {
                c if c.chars().all(|c| c.is_ascii_uppercase()) => {
                    if let Some(match_len) = s[i + 1..].find(|c:char| !c.is_ascii_alphabetic()) {
                        let value = &s[i..i + match_len + 1];
                        if value.chars().all(|c| c.is_ascii_digit()) {
                            stack.push_back(Item::Literal(value.to_string()));
                            i += match_len + 1;
                        } else if value == "^v" {
                            stack.push_back(Item::Literal(value.to_string()));
                            i += 3;
                        } else if value == "^" {
                            i += 2;
                            stack.push_back(Item::Literal(
                                "copies the evaluated result of the cell above in the same column"
                                    .to_string(),
                            ));
                        } else {
                            panic!("Unsupported structure");
                        }
                    } else {
                        panic!("WTF");
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
                        s[i + 1..].find(|c:char| c == '_' || c.is_ascii_alphabetic())
                    {
                        let value = &s[i..i + match_len + 2];
                        stack.push_back(Item::Token(value.to_string()));
                        i += match_len + 2;
                    } else {
                        panic!("WTF");
                    }
                }
                " " | "," => {
                    i += 1;
                }
                op if ['+', '*', '-', '/'].contains(&op.chars().next().unwrap()) => {
                    stack.push_back(Item::Operator(op.chars().next().unwrap()));
                    i += 1;
                }
                _ => panic!("WTF"),
            }
        }

        if stack.len() > 1 {
            panic!("Lacking end statement");
        }
        String::from("sadf")
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

            for key in newKeys {
                let cell: &String = self.columns[i].values.get(&key).unwrap();
                let calculated_data: String =
                    self.parse_string(String::from(cell), key, String::from(&self.columns[i].name));
                self.columns[i].values.insert(12, calculated_data);
            }
        }
    }
}
