use std::collections::VecDeque;

use crate::str_utils::{StrUtils};
use crate::table::{TableDataGetter, TableData, ColumnGetter};
use regex::{Regex};

pub enum Item {
    Literal(String),
    Token(String),
    Operator(char),
    ZoneStart(char),
    ZoneEnd(char),
}

impl Item {
    fn get_literal_as_number(&self) -> usize {
        if let Item::Literal(s) = self {
            s.as_str().parse::<usize>().expect("Expected literal number")
        } else {
            panic!("Current item should be a literal");
        }
    }

    fn get_literal_as_text(&self) -> &str {
        if let Item::Literal(s) = self {
            return s.as_str();
        } else {
            panic!("Current item should be a literal");
        }
    }

    fn get_token(&self) -> String {
        if let Item::Token(s) = self {
            return String::from(s);
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
    // fn get_command(s: &str) -> Command;
    fn execute_str(&self, s: &str, index: u32, name: String) -> String;
    fn fill_data(&mut self);
    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>);
    fn calc_function(&self, name: &str, args: &[String]) -> String;
    fn resolve_literal_at(&self, s: &str, i: usize) -> (String, usize);
    fn revaluate_from_literal(&self, stack: &mut VecDeque<Item>);
    fn get_matching_start_zone(item: Item) -> char;
    fn increase_column_digits(text: String, prev_num: u32) -> String;
    fn evaluate_column_reference(&self, stack: &mut VecDeque<Item>) -> Item;
}

impl LogicExecutor for TableData {
    // fn get_command(s: &str) -> Command {
    //     let operands = vec![];
    //     let c = Command {
    //         operands,
    //         operator: String::from("asd"),
    //     };
    //     return c;
    // }

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

    fn evaluate_column_reference(&self, stack: &mut VecDeque<Item>) -> Item {
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
        let res = String::from(
            self
                .get_by_name_unmut(column_name.as_str().remove_first_symbol()) // drop @
                .get_cell_by_index(column_index));
        return Item::Literal(res);
    }

    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>) {
        let item = stack.pop_back().unwrap();
        let mut operands: Vec<String> = Vec::new();
        match item {
            Item::Literal(val) => {
                operands.push(val);
            }
            Item::ZoneEnd(val) if val == '>' => {
                let column_res = self.evaluate_column_reference(stack);
                stack.push_back(column_res);
                return;
            }
            Item::ZoneEnd(val)  if val == '>' => {
                let start_zone_value = TableData::get_matching_start_zone(item);
                loop {
                    let item_inner = stack.pop_back().expect("WTF");
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

    fn resolve_literal_at(&self, s: &str, i: usize) -> (String, usize) {
        if s.at(i + 1).is_ascii_digit() { // cell reference
            let index = &s.at(i + 1).to_string().parse::<usize>().expect("Expected literal number");
            let value = self.get_by_letter_unmut(s.at(i)).get_cell_by_index(*index);
            return (value, 2);
        } else if &s[i + 1..=i + 2] == "^v" {
            return (String::from("asd"), 3);
        } else if &s[i + 1..=i + 1] == "^" {
            return (String::from("asd"), 2);
        } else {
            panic!("Unsupported structure for value {}", &s[i + 1..=i + 1]);
        }
    }

    fn execute_str(&self, s: &str, index: u32, name: String) -> String {
        let mut stack: VecDeque<Item> = VecDeque::new();
        let mut i = 0;
        while i < s.len() || stack.len() > 1 {
            // if resolved to literal
            if s.at(i).is_uppercase() && !s.at(i + 1).is_alphabetic() {
                let (literal, literal_length) = self.resolve_literal_at(s, i);
                i += literal_length;
                stack.push_back(Item::Literal(literal))
            // if token
            } else if s.at(i).is_ascii_alphabetic() {
                let token_length = s.next_word_length(i+1);
                let token = s[i..i + token_length + 1].to_string();
                stack.push_back(Item::Token(token));
                i += token_length + 1;
            // if number
            } else if s.at(i).is_ascii_digit() {
                let digit_length = s.next_digit_length(i+1);
                let digit = &s[i..i + digit_length + 1];
                stack.push_back(Item::Literal(digit.to_string()));
                i += digit_length + 1;
            // if expression starts
            } else if ['(', '<'].contains(&s.at(i)) {
                stack.push_back(Item::ZoneStart(s.at(i)));
                i += 1;
            // if expression end, here we need to evaluated all inside of it.
            } else if [')', '>'].contains(&s.at(i)) {
                stack.push_back(Item::ZoneEnd(s.at(i)));
                i += 1;
                self.revaluate_from_end_zone(&mut stack);
            // if string literal
            } else if s.at(i) == '\"' {
                let literal_length =  s.next_quote_length(i+1);
                let val = &s[i..i + literal_length + 2]; // 2 is start+end quote
                stack.push_back(Item::Literal(val.to_string()));
                i += literal_length + 2; // "asd" = 3 + 2
            // if column reference
            } else if s.at(i) == '@' {
                let token_length = s.next_word_length_underscore(i + 1);
                stack.push_back(Item::Token(s[i..i + token_length + 1].to_string()));
                i += token_length + 1; // @ + word itself
            // ignore separators
            } else if [' ', ','].contains(&s.at(i)) {
                i += 1;
            // arithmetic operations
            } else if ['+', '*', '-', '/'].contains(&s.at(i)) {
                stack.push_back(Item::Operator(s.at(i)));
                i += 1;
            } else {
                panic!("Unknown symbol {} at position {}", &s[i..i + 1], i);
            }
        }

        return stack
            .pop_back()
            .expect("Stack evaluated to 0")
            .get_literal_as_text()
            .to_string();
    }

    fn parse_string(&self, s: String, index: u32, name: String) -> String {
        return if &s.as_str()[0..=0] == "=" {
            // this formula should be evaluated
            self.execute_str(s.as_str().remove_first_symbol(), index, name)
        } else {
            s
        };
    }

    fn fill_data(&mut self) {
        for column_index in 0..self.columns.len() {
            let row_keys: Vec<u32> = self.columns[column_index].get_sorted_keys();
            let mut prev_val: String = String::from("");
            for row_number in row_keys {
                let cell: &String = self.columns[column_index].values.get(&row_number).unwrap();
                // this formula should be resolved before the main loop, since it allows only 1 expression
                if cell == "=^^" {
                    // replace occurrences in the current row
                    // sum(spread(split(D2, ","))) -> sum(spread(split(D3, ",")))
                    let s = TableData::increase_column_digits(prev_val.clone(), row_number - 1);
                    let calculated_data = self.parse_string(
                        s,
                        row_number,
                        String::from(&self.columns[column_index].name),
                    );
                    self.columns[column_index].values.insert(row_number, calculated_data);
                } else {
                    prev_val = String::from(cell);
                    let calculated_data = self.parse_string(
                        String::from(cell),
                        row_number,
                        String::from(&self.columns[column_index].name),
                    );
                    self.columns[column_index].values.insert(row_number, calculated_data);
                }
            }
        }
    }
}
