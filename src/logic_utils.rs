use std::collections::VecDeque;

use regex::Regex;

use crate::str_utils::StrUtils;
use crate::table::{ColumnGetter, Item, LiteralValue, TableData, TableDataGetter};

pub trait LogicExecutor {
    fn parse_string(&self, s: String, index: u32, inc_from: &mut usize) -> LiteralValue;
    fn execute_str(&self, s: &str, index: u32, inc_from: &mut usize) -> LiteralValue;
    fn fill_data(&mut self);
    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>, inc_from: &mut usize);
    fn call_function(&self, name: &str, args: Vec<LiteralValue>, inc_from: &mut usize) -> LiteralValue;
    fn evaluate_arithmetic(&self, operator: char, args: &Vec<LiteralValue>) -> LiteralValue;
    fn resolve_literal_at(&self, s: &str, i: usize, current_row_number: u32) -> (LiteralValue, usize);
    fn increase_column_digits(text: String, prev_num: u32) -> String;
    fn evaluate_column_reference(&self, stack: &mut VecDeque<Item>);
    fn evaluate_curly_zone(&self, stack: &mut VecDeque<Item>, inc_from: &mut usize);

    fn inc_from(&self, args: Vec<LiteralValue>, inc_from: &mut usize) -> LiteralValue;
    fn split(&self, args: Vec<LiteralValue>) -> LiteralValue;
    fn spread(&self, args: Vec<LiteralValue>) -> LiteralValue;
    fn sum(&self, args: Vec<LiteralValue>) -> LiteralValue;
    fn text(&self, args: Vec<LiteralValue>) -> LiteralValue;
    fn bte(&self, args: Vec<LiteralValue>) -> LiteralValue;
    fn concat(&self, args: Vec<LiteralValue>) -> LiteralValue;
}

impl LogicExecutor for TableData {
    /// ^^
    /// =E^+sum(spread(split(D3, ",")))
    /// -> =E^+sum(spread(split(D4, ",")))
    fn increase_column_digits(text: String, prev_num: u32) -> String {
        return Regex::new(format!("[A-Z]{prev_num}+").as_str())
            .unwrap()
            .replace_all(&text, |caps: &regex::Captures| {
                caps[0][0..=0].to_string() + &(prev_num + 1).to_string()
            }).to_string();
    }

    /// @adjusted_cost<1>
    fn evaluate_column_reference(&self, stack: &mut VecDeque<Item>) {
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
        let literal = self
            .get_by_name_unmut(column_name.as_str().remove_first_symbol()) // drop @
            .get_cell_by_index(column_index);
        stack.push_back(Item::Literal(literal));
    }

    /// e.g. split(D3, ",")
    /// (E^v*A9)
    fn evaluate_curly_zone(&self, stack: &mut VecDeque<Item>, inc_from: &mut usize) {
        let mut operands: Vec<LiteralValue> = vec![];
        loop {
            let item_inner = stack.pop_back().expect("No matching pair of ')' found");
            if let Item::Literal(value) = item_inner {
                operands.push(value);
            } else if let Item::ZoneStart(value) = item_inner {
                assert_eq!(value, '(', "no opening braces");
                //         3 possible cases by now
                //   =E^v+(E^v*A9) | split(D2, ",") | (2+3)
                //       +               +            +
                if !stack.is_empty() {
                    let item_before_braces = stack.pop_back().unwrap();
                    if let Item::Token(operation) = item_before_braces {
                        let res = self.call_function(&operation, operands, inc_from);
                        stack.push_back(Item::Literal(res));
                        break;
                    }
                    // if it's not a token, than we should return it to stack
                    // so we don't break the structure
                    stack.push_back(item_before_braces);
                }
                if operands.len() == 1 {
                    stack.push_back(Item::Literal(operands[0].clone()))
                } else {
                    panic!("Multiple operands withing braces without operation")
                }
                break;
                // part of arithmetic operations, validate the last one,
                // and queue back to the stack in case there are multiple of them
                // e.g. 2+3+4+6
            } else if let Item::Operator(operator) = item_inner {
                let left_operand = stack
                    .pop_back()
                    .expect("No left expression to operator")
                    .get_literal().clone();
                operands.push(left_operand);
                let res = self.evaluate_arithmetic(operator, &operands);
                operands.clear();
                stack.push_back(Item::Literal(res));
            } else {
                panic!("Invalid operation")
            }
        }
    }

    /// if we faced the end zone symbol, e.g. '>' or ')' we should go back and eval it
    fn revaluate_from_end_zone(&self, stack: &mut VecDeque<Item>, inc_from: &mut usize) {
        let end_zone_symbol = stack
            .pop_back()
            .unwrap()
            .get_end_zone_character();

        if end_zone_symbol == '>' {  // @adjusted_cost<1>
            self.evaluate_column_reference(stack);
        } else { // split(D2, ",") ||||||    (E^v*A9)
            self.evaluate_curly_zone(stack, inc_from);
        }
    }

    /// call function by its name e.g. incFrom(1)
    fn call_function(&self, name: &str, args: Vec<LiteralValue>, inc_from: &mut usize) -> LiteralValue {
        match name {
            "incFrom" => self.inc_from(args, inc_from),
            "split" => self.split(args),
            "spread" => self.spread(args),
            "sum" => self.sum(args),
            "text" => self.text(args),
            "bte" => self.bte(args),
            "concat" => self.concat(args),
            _ => Item::conduct_str_literal_value(format!("[{}({})]", name, "wtf")),
        }
    }

    /// incFrom(1)
    fn inc_from(&self, args: Vec<LiteralValue>, inc_from: &mut usize) -> LiteralValue {
        assert_eq!(args.len(), 1, "incFrom accept 1 arg");
        let i = args[0].value_as_int.unwrap();
        *inc_from = *inc_from + i;
        return Item::conduct_int_literal_value(*inc_from);
    }

    /// sum(spread(split(D2, ",")))
    fn sum(&self, args: Vec<LiteralValue>) -> LiteralValue {
        assert_eq!(args.len(), 1, "summ accepts 1 element");
        let sum: f32 = args[0].value_as_float_array.clone().unwrap().iter().sum();
        return Item::conduct_float_literal_value(sum);
    }

    /// text(incFrom(1))
    fn text(&self, args: Vec<LiteralValue>) -> LiteralValue {
        assert_eq!(args.len(), 1, "summ accepts 1 element");
        let res = Item::literal_to_string(&args[0]);
        if res.is_none() {
            panic!("text function supports only text and int")
        }
        return Item::conduct_str_literal_value(res.unwrap());
    }

    /// concat("t_", text(incFrom(1)))
    fn concat(&self, args: Vec<LiteralValue>) -> LiteralValue {
        assert_eq!(args.len(), 2, "concant accepts 2 element");
        let a: String = args[1].value_as_string.clone().unwrap();
        let b: String = args[0].value_as_string.clone().unwrap();
        return Item::conduct_str_literal_value(format!("{}{}", a, b));
    }

    /// bte(@adjusted_cost<1>, @cost_threshold<1>)
    fn bte(&self, args: Vec<LiteralValue>) -> LiteralValue {
        assert_eq!(args.len(), 2, "summ accepts 1 element");
        let a: f32 = args[1].value_as_float.unwrap();
        let b: f32 = args[0].value_as_float.unwrap();
        let res = match a >= b {
            true => "true",
            false => "false"
        };
        return Item::conduct_str_literal_value(res.to_string());
    }

    /// spread(split(D2, ","))
    fn spread(&self, args: Vec<LiteralValue>) -> LiteralValue {
        assert_eq!(args.len(), 1, "spread accept 1 arg");
        let arr = args[0]
            .value_as_str_array
            .clone()
            .expect("expected str array for spread");
        let values: Vec<f32> = arr
            .iter()
            .map(|x| x.to_string().parse::<f32>().unwrap())
            .collect();
        return LiteralValue {
            value_as_str_array: None,
            value_as_string: None,
            value_as_int: None,
            value_as_float: None,
            value_as_float_array: Some(values),
        };
    }

    /// split(D2, ",")
    fn split(&self, args: Vec<LiteralValue>) -> LiteralValue {
        assert_eq!(args.len(), 2, "split accept 2 arg");
        // arguments are in reversed because of stack.pop
        let from_column = args[1].value_as_string.clone().unwrap();
        let separator = args[0].value_as_string.clone().unwrap();
        assert_eq!(separator.len(), 1, "separator should be 1 symbol");
        let x: char = separator.at(0);
        let array_strs = from_column
            .as_str()
            .split(x)
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let value = LiteralValue {
            value_as_string: None,
            value_as_float: None,
            value_as_int: None,
            value_as_str_array: Some(array_strs),
            value_as_float_array: None,
        };
        return value;
    }

    /// E^v+(E^v*A9)
    fn evaluate_arithmetic(&self, operator: char, args: &Vec<LiteralValue>) -> LiteralValue {
        assert_eq!(args.len(), 2, "Cannot operate with complex arguments");
        let a = args[0].value_as_float.unwrap();
        let b = args[1].value_as_float.unwrap();
        let c = match operator {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => panic!("WTf")
        };
        return Item::conduct_float_literal_value(c);
    }

    /// E^, E^v, A9
    fn resolve_literal_at(&self, s: &str, i: usize, current_row_number: u32) -> (LiteralValue, usize) {
        // A9
        if s.at(i + 1).is_ascii_digit() { // cell reference
            let index = &s.at(i + 1).to_string().parse::<u32>().expect("Expected literal number");
            let value = self.get_by_coordinate(s.at(i), index);
            return (value, 2);
            //  E^v
        } else if &s[i + 1..=i + 2] == "^v" {
            let res = self.get_last_value_of_the_column(s.at(i));
            return (res, 3);
            //  E^
        } else if &s[i + 1..=i + 1] == "^" {
            let value = self.get_by_coordinate(s.at(i), &(current_row_number - 1));
            return (value, 2);
        } else {
            panic!("Unsupported structure for value {}", &s[i + 1..=i + 1]);
        }
    }

    /// evaluate expression inside of cell
    fn execute_str(&self, s: &str, index: u32, inc_from: &mut usize) -> LiteralValue {
        let mut stack: VecDeque<Item> = VecDeque::new();
        let mut i = 0;
        while i < s.len() {
            // if resolved to literal
            if s.at(i).is_uppercase() && !s.at(i + 1).is_alphabetic() {
                let (literal, literal_length) = self.resolve_literal_at(s, i, index);
                i += literal_length;
                stack.push_back(Item::Literal(literal))
                // if token
            } else if s.at(i).is_ascii_alphabetic() {
                let token_length = s.next_word_length(i + 1);
                let token = s[i..i + token_length + 1].to_string();
                stack.push_back(Item::Token(token));
                i += token_length + 1;
                // if number
            } else if s.at(i).is_ascii_digit() {
                let digit_length = s.next_digit_length(i + 1);
                let digit = &s[i..i + digit_length + 1];
                let digit_val = digit.to_string().parse::<usize>().unwrap();
                stack.push_back(Item::conduct_int_literal(digit_val));
                i += digit_length + 1;
                // if expression starts
            } else if ['(', '<'].contains(&s.at(i)) {
                stack.push_back(Item::ZoneStart(s.at(i)));
                i += 1;
                // if expression end, here we need to evaluated all inside of it.
            } else if [')', '>'].contains(&s.at(i)) {
                stack.push_back(Item::ZoneEnd(s.at(i)));
                i += 1;
                self.revaluate_from_end_zone(&mut stack, inc_from);
                // if string literal
            } else if s.at(i) == '\"' {
                let literal_length = s.next_quote_length(i + 1);
                let val = &s[i + 1..i + literal_length + 1]; // 2 is start+end quote
                stack.push_back(Item::conduct_string_literal(val.to_string()));
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

        while !stack.is_empty() {
            if stack.len() == 1 {
                if let Item::Literal(value) = stack.pop_back().expect("Stack evaluated to 0") {
                    return value;
                } else {
                    panic!("qt");
                }
            }
            let prev_stack_length = stack.len();
            stack.push_front(Item::ZoneStart('('));
            self.evaluate_curly_zone(&mut stack, inc_from);
            if prev_stack_length == stack.len() {
                panic!("Mismatch curly braces");
            }
        }
        panic!("Invalid expression");
    }

    /// create literal from expression
    /// if expression has = at start, evaluate it
    fn parse_string(&self, s: String, index: u32, inc_from: &mut usize) -> LiteralValue {
        return if &s.as_str()[0..=0] == "=" {
            // this formula should be evaluated
            self.execute_str(s.as_str().remove_first_symbol(), index, inc_from)
        } else {
            let a = s.parse::<f32>();
            if a.is_err() {
                return Item::conduct_str_literal_value(s);
            } else {
                return Item::conduct_float_literal_value(a.unwrap());
            }
        };
    }

    /// loop through cells and fill resolved_value
    fn fill_data(&mut self) {
        let mut inc_from: usize = 0; // for incFrom(
        for column_index in 0..self.columns.len() {
            let row_keys: Vec<u32> = self.columns[column_index].get_sorted_keys();
            let mut prev_val: String = String::from("");
            for row_number in row_keys {
                let cell: &String = self.columns[column_index].string_values.get(&row_number).unwrap();
                // this formula should be resolved before the main loop, since it allows only 1 expression
                if cell == "=^^" {
                    // replace occurrences in the current row
                    // sum(spread(split(D2, ","))) -> sum(spread(split(D3, ",")))
                    let s = TableData::increase_column_digits(prev_val.clone(), row_number - 1);
                    let calculated_data = self.parse_string(
                        s,
                        row_number,
                        &mut inc_from,
                    );
                    self.columns[column_index].resolved_value.insert(row_number, calculated_data);
                } else {
                    prev_val = String::from(cell);
                    let calculated_data = self.parse_string(
                        String::from(cell),
                        row_number,
                        &mut inc_from,
                    );
                    self.columns[column_index].resolved_value.insert(row_number, calculated_data);
                }
            }
        }
    }
}


