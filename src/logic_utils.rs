use std::collections::VecDeque;
use std::fmt::Pointer;

use crate::str_utils::{StrUtils};
use crate::table::*;
use regex::{Captures, Regex};

struct Command {
    operands: Vec<String>,
    operator: String,
}

enum Item {
    Literal(String),
    Token(String),
    Operator(char),
    ZoneStart(char),
    ZoneEnd(char),
}

impl Item {
    fn get_literal_as_number(&self) -> u32 {
        if let Item::Literal(s) = self {
            s.as_str().parse::<u32>().expect("Expected literal number")
        } else {
            panic!("Current item should be a literal");
        }
    }

    fn get_token(&self) -> String {
        if let Item::Token(s) = self {
            return String::from(s)
        } else {
            panic!("Current item should be a token");
        }
    }

    fn expect_start_of(&self, char_value: char) {
        if let Item::ZoneStart(s) = self {
            assert_eq!(s, &char_value, "Column reference should preceed <");
        } else {
            panic!("Current item should be literal");
        }
    }
}

pub trait LogicExecutor {
    fn parse_string(&self, s: String, index: u32, name: String) -> String;
    fn get_command(s: &str) -> Command;
    fn execute_str(&self, s: &str, index: u32, name: String) -> String;
    fn fill_data(&mut self);
    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>);
    fn calc_function(&self, name: &str, args: &[String]) -> String;
    fn revaluate_from_literal(&self, stack: &mut VecDeque<Item>);
    fn get_matching_start_zone(item: Item) -> char;
    fn increase_column_digits(text: String, prev_num: u32) -> String;
    fn evaluate_column_reference(&self, item: Item, stack: &mut VecDeque<Item>) -> Item;
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

    fn increase_column_digits(text: String, prev_num: u32) -> String {
        return Regex::new(format!("[A-Z]{prev_num}+").as_str())
            .unwrap()
            .replace_all(&text, |caps: &regex::Captures| {
                caps[0][0..=0].to_string() + &(prev_num + 1).to_string()
            }).to_string();
    }

    fn get_matching_start_zone(item: Item) -> char {
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

    fn revaluate_from_literal(&self, stack: &mut VecDeque<Item>) {}

    fn evaluate_column_reference(&self, item: Item, stack: &mut VecDeque<Item>) -> Item {
        let column_with_index = stack.pop_back().expect("column reference should predicate index");
        let column_index = column_with_index.get_literal_as_number();
        stack
            .pop_back()
            .expect("Column ref should precede index")
            .expect_start_of('<');
       let column_name = stack
           .pop_back()
           .expect("Column ref should precede name")
           .get_token();
        return Item::Literal(
            String::from(
                self.get_by_name(&column_name)
                    .values
                    .get(&column_index)
                    .expect("wtf")
            )
        )
    }

    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>) {
        let item = stack.pop_back().unwrap();
        let mut operands = Vec::new();
        match item {
            Item::Literal(val) => {
                println!("WTF");
            }
            Item::ZoneEnd(val) if val == '>' => {
                stack.push_back(self.evaluate_column_reference(item, stack));
            }
            Item::ZoneEnd(val) => {
                let start_zone_value = TableData::get_matching_start_zone(item);
                loop {
                    assert!(!stack.is_empty(), "WTF");
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
                                                    let res = self
                                                        .calc_function(&op.to_string(), &operands);
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
    }

    fn calc_function(&self, name: &str, args: &[String]) -> String {
        format!("[{}({})]", name, args.join(","))
    }

    fn execute_str(&self, s: &str, index: u32, name: String) -> String {
        let mut stack: VecDeque<Item> = VecDeque::new();
        let mut i = 0;

        while i < s.len() {
            match &s[i..=i] {
                c if c.at(0).is_uppercase() && !s.at(i + 1).is_alphabetic() => {
                    if s.at(i + 1).is_ascii_digit() {
                        stack.push_back(Item::Literal(String::from("asd")));
                        i += 2;
                    } else if &s[i + 1..=i + 2] == "^v" {
                        stack.push_back(Item::Literal(String::from("asd")));
                        i += 3;
                    } else if &s[i + 1..=i + 1] == "^" {
                        i += 2;
                        stack.push_back(Item::Literal(String::from("asd")));
                    } else {
                        panic!("Unsupported structure for value {}", &s[i + 1..=i + 1]);
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
                    let match_len = s.next_word_length_underscore(i+1).expect("Wtf");
                    let value = &s[i..i + match_len + 1];
                    stack.push_back(Item::Token(value.to_string()));
                    i += match_len + 1;
                }
                " " | "," => {
                    i += 1;
                }
                op if ['+', '*', '-', '/'].contains(&op.chars().next().unwrap()) => {
                    stack.push_back(Item::Operator(op.chars().next().unwrap()));
                    i += 1;
                }
                _ => panic!("Unknown symbol {} at position {}", &s[i..i + 1], i),
            }
        }
        assert_eq!(stack.len(), 1, "Unknown structure");

        match stack.pop_back().unwrap() {
            Item::Literal(val) => {
                return val;
            }
            _ => panic!("WTF"),
        }
    }

    fn parse_string(&self, s: String, index: u32, name: String) -> String {
        return if &s.as_str()[0..=0] == "=" {
            self.execute_str(s.as_str().remove_first_symbol(), index, name)
        } else {
            s
        }
    }

    fn fill_data(&mut self) {
        for i in 0..self.columns.len() {
            let mut new_keys: Vec<u32> = self.columns[i]
                .values
                .keys()
                .map(|x| x.clone())
                .collect::<Vec<u32>>();
            new_keys.sort();

            let mut prev_val: String = "".to_string();
            for key in new_keys {
                let cell: &String = self.columns[i].values.get(&key).unwrap();

                if cell == "=^^" {
                    let replaced_prev_values_str =
                        TableData::increase_column_digits(prev_val.clone(), key - 1);
                    let calculated_data = self.parse_string(
                        replaced_prev_values_str,
                        key,
                        String::from(&self.columns[i].name),
                    );
                    self.columns[i].values.insert(12, calculated_data);
                } else {
                    prev_val = String::from(cell);
                    let calculated_data = self.parse_string(
                        String::from(cell),
                        key,
                        String::from(&self.columns[i].name),
                    );
                    self.columns[i].values.insert(12, calculated_data);
                }
            }
        }
    }
}
