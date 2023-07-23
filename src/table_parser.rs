use crate::table::{TableData, Column, TableDataGetter};
use crate::io_utils::{str_to_vector};
use crate::logic_utils::{is_head, extract_head_name};
use std::collections::HashMap;

fn add_column(table: &mut TableData, name: &str, index: u8) {
    let letter = char::from(('A' as u8) + index); // 0 -> A, 1 -> B, ...

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

pub fn lines_to_table(data: Vec<&str>) -> TableData {
    let mut table: TableData = TableData { columns: vec![] };
    let mut current_head: Vec<&str> = vec![];
    let mut row_index = 1;

    for line in data {
        let columns = str_to_vector(line, '|');
        let current_is_head: bool = is_head(columns[0]);

        if current_is_head {
            current_head.clear();
            let mut index = 0;
            for column in columns {
                if column != "" {
                    if !is_head(&column) {
                        panic!("Invalid file structure, some columns are heads, and some not")
                    }
                    let name = extract_head_name(column);
                    current_head.push(name);
                    add_column(&mut table, name, index);
                }
                index += 1;
            }
        } else {
            let mut index = 0;
            for column in columns {
                if column != "" {
                    let name = current_head[index];
                    table
                        .get_by_name(name)
                        .values
                        .insert(row_index, String::from(column));
                }
                index += 1;
            }
        }
        row_index += 1;
    }
    return table;
}