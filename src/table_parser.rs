use crate::table::{TableData, Column, TableDataGetter};
use crate::io_utils::{str_to_vector, remove_first_symbol};

use std::collections::HashMap;

fn add_column(table: &mut TableData, name: &str, index: i8) {
    let letter = char::from(('A' as u8) + (index as u8)); // 0 -> A, 1 -> B, ...

    if table.columns.iter().any(|x| x.name == name) {
        panic!("Column {name} already exists");
    }
    let c = Column {
        letter: String::from(letter.to_string()),
        name: String::from(name),
        values: HashMap::new(),
    };
    table.columns.push(c);
}

fn extract_head_name(column: &str) -> &str {
    return remove_first_symbol(column);
}

fn is_head(col: &str) -> bool {
    return col.starts_with("!");
}

pub fn lines_to_table(data: Vec<&str>) -> TableData {
    let mut table: TableData = TableData { columns: vec![] };
    let mut current_head: Vec<&str> = vec![];
    let mut row_index = 1;

    for line in data {
        let columns = str_to_vector(line, '|');
        let current_is_head: bool = is_head(columns[0]);
        let mut index: i8 = -1;
        if current_is_head {
            current_head.clear();
            for column in columns {
                index += 1;
                if column == "" {
                    continue
                }
                assert!(!is_head(&column), "Invalid head at index {row_index}");
                current_head.push(extract_head_name(column));
                add_column(&mut table, extract_head_name(column), index);
            }
        } else {
            for column in columns {
                index += 1;
                if column == "" {
                    continue
                }
                table
                    .get_by_name(current_head[index])
                    .values
                    .insert(row_index, String::from(column));
            }
        }
        row_index += 1;
    }
    return table;
}