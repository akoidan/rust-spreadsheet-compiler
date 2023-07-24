use std::collections::HashMap;

pub struct Column {
    pub name: String,
    pub letter: char,
    pub string_values: HashMap<u32, String>,
    pub resolved_value: HashMap<u32, LiteralValue>,
}

pub struct TableData {
    pub columns: Vec<Column>,
}

#[derive(Clone)]
pub struct LiteralValue {
    pub value_as_string: Option<String>,
    pub value_as_float: Option<f32>,
    pub value_as_int: Option<usize>,
    pub value_as_str_array: Option<Vec<String>>,
    pub value_as_float_array: Option<Vec<f32>>,
}

pub enum Item {
    Literal(LiteralValue),
    Token(String),
    Operator(char),
    ZoneStart(char),
    ZoneEnd(char),
}


pub trait TableDataGetter {
    fn add_column(&mut self, name: &str, index: usize);
    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column;
    fn get_by_name_unmut<'a>(&'a self, name: &str) -> &'a Column;
    fn get_last_value_of_the_column(&self, letter: char) -> LiteralValue;
    fn get_by_coordinate(&self, letter: char, row_number: &u32) -> LiteralValue;
    fn get_col_width(&self, col_index: u8) -> usize;
    fn get_row_count(&self) -> usize;
    fn get_col_count(&self) -> usize;
    fn as_string(&self) -> String;
    fn get_data_as_str(&self, row_index: &u32, col_index: u8) -> Option<String>;
}

pub trait ColumnGetter {
    fn get_sorted_keys(&self) -> Vec<u32>;
    fn get_cell_by_index(&self, index: usize) -> LiteralValue;
    fn get_last_cell_index(&self) -> u32;
    fn get_first_cell_index(&self) -> u32;
}

impl Item {
    pub fn get_literal_as_number(&self) -> usize {
        if let Item::Literal(value) = self {
            return value.value_as_int.expect("Expected literal number");
        } else {
            panic!("Current item should be a literal");
        }
    }

    pub fn conduct_string_literal(val: String) -> Item {
        return Item::Literal(Item::conduct_str_literal_value(val));
    }

    pub fn literal_to_string(val: &LiteralValue) -> Option<String> {
        if val.value_as_string.is_some() {
            return Some(val.value_as_string.clone().unwrap().clone().to_string());
        } else if val.value_as_float.is_some() {
            return Some(val.value_as_float.unwrap().clone().to_string());
        } else if val.value_as_int.is_some() {
            return Some(val.value_as_int.unwrap().clone().to_string());
        } else {
            return None;
        }
    }

    pub fn conduct_int_literal_value(val: usize) -> LiteralValue {
        return LiteralValue {
            value_as_string: None,
            value_as_str_array: None,
            value_as_float: None,
            value_as_int: Some(val),
            value_as_float_array: None,
        };
    }

    pub fn conduct_float_literal_value(val: f32) -> LiteralValue {
        return LiteralValue {
            value_as_string: None,
            value_as_str_array: None,
            value_as_float: Some(val),
            value_as_int: None,
            value_as_float_array: None,
        };
    }

    pub fn conduct_str_literal_value(val: String) -> LiteralValue {
        return LiteralValue {
            value_as_string: Some(val.to_string()),
            value_as_str_array: None,
            value_as_float: None,
            value_as_int: None,
            value_as_float_array: None,
        };
    }

    pub fn conduct_int_literal(val: usize) -> Item {
        return Item::Literal(Item::conduct_int_literal_value(val));
    }

    pub fn get_end_zone_character(&self) -> char {
        if let Item::ZoneEnd(s) = self {
            *s
        } else {
            panic!("Current item should be a literal");
        }
    }

    pub fn get_literal(&self) -> &LiteralValue {
        if let Item::Literal(value) = self {
            return value;
        } else {
            panic!("Current item should be a literal");
        }
    }

    pub fn get_token(&self) -> String {
        if let Item::Token(s) = self {
            return String::from(s);
        } else {
            panic!("Current item should be a token");
        }
    }

    pub fn expect_start_of(&self, char_value: char) {
        if let Item::ZoneStart(s) = self {
            assert_eq!(s, &char_value, "Column reference should preceed <");
        } else {
            panic!("Current item should be literal");
        }
    }
}


impl ColumnGetter for Column {
    fn get_sorted_keys(&self) -> Vec<u32> {
        let mut new_keys: Vec<u32> = self.string_values
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<u32>>();
        new_keys.sort();
        return new_keys;
    }

    fn get_last_cell_index(&self) -> u32 {
        return *self.string_values.keys().max().unwrap();
    }

    fn get_first_cell_index(&self) -> u32 {
        return *self.string_values.keys().min().unwrap();
    }

    fn get_cell_by_index(&self, index: usize) -> LiteralValue {
        // @adjusted_cost<1> this references to first element which indexing start at 0,
        // this is why -1
        let key = self.get_sorted_keys()[index - 1];
        return self
            .resolved_value.get(&key)
            .expect(&format!("key by index {} is has not been calculated yet or doesnt exist", index))
            .clone();
    }
}

impl TableDataGetter for TableData {
    /// Adds column to table by specific Name (e.g. !adjusted_cost)
    /// to specific col_index that transforms to a letter (e.g. A)
    fn add_column(&mut self, name: &str, index: usize) {
        let letter = char::from(('A' as u8) + (index as u8)); // 0 -> A, 1 -> B, ...
        assert!(!self.columns.iter().any(|x| x.name == name), "Column {} already exists", name);
        self.columns.push(Column {
            letter,
            name: String::from(name),
            string_values: HashMap::new(),
            resolved_value: HashMap::new(),
        });
    }

    /// Returns a mutable Column by name e.g. adjusted_cost
    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &mut self.columns[index.expect(&format!("column {name} doesnt exist"))];
    }
    //// Returns a Column by name e.g. adjusted_cost
    fn get_by_name_unmut<'a>(&'a self, name: &str) -> &'a Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &self.columns[index.expect(&format!("column {name} doesnt exist"))];
    }


    ///     Returns the last (closer to bottom) cell by specified letter
    ///    e.g. if 2 named columns present on letter A, then it would return the lower one
    fn get_last_value_of_the_column(&self, letter: char) -> LiteralValue {
        let mut colum_index: u32 = 0;
        let mut res: Option<LiteralValue> = None;
        for column in &self.columns {
            if column.letter != letter {
                continue;
            }
            let new_index = column.get_last_cell_index();
            if new_index > colum_index {
                colum_index = new_index;
                res = Some(column.resolved_value.get(&new_index).unwrap().clone());
            }
        }
        return res.expect("No such letter");
    }

    /// Returns LiteralValue (calculated already) by coordinates, e.g. D2 or A9
    fn get_by_coordinate(&self, letter: char, row_number: &u32) -> LiteralValue {
        for column in &self.columns {
            if column.letter != letter {
                continue;
            }
            let row = column.resolved_value.get(row_number);
            if row.is_some() {
                return column
                    .resolved_value
                    .get(row_number)
                    .unwrap()
                    .clone();
            }
        }
        panic!("Referenced to non-existed column {}{}", letter, row_number);
    }

    /// Gets string data from original file by coordinates
    /// Columns have non-original structure, making table have more column that original file,
    /// since some name duplicate
    fn get_data_as_str(&self, row_index: &u32, col_index: u8) -> Option<String> {
        for col in &self.columns {
            if col.letter as u8 - 'A' as u8 == col_index {
                let value = col.resolved_value.get(&row_index);
                if let Some(value) = value {
                    if let Some(value_as_string) = value.value_as_string.as_ref() {
                        return Some(value_as_string.to_string());
                    } else if let Some(value_as_float) = value.value_as_float.as_ref() {
                        return Some(value_as_float.to_string());
                    } else {
                        return Some("".to_string());
                    }
                } else if col.get_first_cell_index() == (row_index + 1) {
                    return Some(format!("!{}", col.name));
                }
            }
        }
        return None;
    }

    fn get_col_width(&self, col_index: u8) -> usize {
        let mut max_width = 1;
        for col in &self.columns {
            let index = (col.letter as u8) - 'A' as u8;
            if col.name.len() > max_width {
                max_width = col.name.len() + 1;
            }
            if index == col_index {
                for (_, value) in &col.resolved_value {
                    let str = Item::literal_to_string(&value);
                    if str.is_some() && str.as_ref().unwrap().len() > max_width {
                        max_width = str.as_ref().unwrap().len();
                    }
                }
            }
        }
        return max_width;
    }

    /// Get original file amount of rows
    fn get_row_count(&self) -> usize {
        let mut current_biggest_index = 0;
        for col in &self.columns {
            let cur_index = col.get_last_cell_index();
            if cur_index > current_biggest_index {
                current_biggest_index = cur_index;
            }
        }
        return current_biggest_index as usize;
    }

    /// Get original file amount of columns
    fn get_col_count(&self) -> usize {
        let mut found_first_a = false;
        let mut col_count = 0;
        for col in &self.columns {
            col_count += 1;
            if col.letter == 'A' && !found_first_a {
                found_first_a = true
            } else if col.letter == 'A' && found_first_a {
                break;
            }
        }
        return col_count;
    }

    /// Prints resolved table in original format
    fn as_string(&self) -> String {
        let mut s: String = String::from("");
        let row_count = self.get_row_count();
        let col_count = self.get_col_count();

        for row_index in 1..=row_count {
            for col_index in 0..col_count {
                let width = self.get_col_width(col_index as u8);
                let res = self.get_data_as_str(&(row_index as u32), col_index as u8);
                if res.is_some() {
                    s.push_str(&format!("{:<width$}", res.unwrap().as_str(), width = width + 1));
                    s.push_str("|")
                }
            }
            s.push_str("\n")
        }
        return s;
    }
}
