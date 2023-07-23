use crate::str_utils::StrUtils;
use crate::table::{TableData, TableDataGetter};

fn is_head(col: &str) -> bool {
    return col.starts_with("!");
}

pub fn lines_to_table(csv_str: &str) -> TableData {
    let data: Vec<&str> = csv_str.split_to_vect('\n');
    let mut table: TableData = TableData { columns: vec![] };
    let mut current_head: Vec<String> = vec![];

    for row_index in 0..data.len() {
        let columns = data[row_index].split_to_vect('|');
        let current_is_head: bool = is_head(columns[0]);
        if current_is_head {
            current_head.clear();
            for column_index in 0..columns.len() {
                if columns[column_index] == "" {
                    continue;
                }
                assert!(is_head(columns[column_index]), "Invalid head at index {}", row_index);
                current_head.push(String::from(columns[column_index].remove_first_symbol()));
                table.add_column(columns[column_index].remove_first_symbol(), column_index);
            }
        } else {
            for column_index in 0..columns.len() {
                if columns[column_index] == "" {
                    continue;
                }
                let x = table.get_by_name(&current_head[column_index]);
                x.values.insert(row_index as u32 +1, String::from(columns[column_index]));
            }
        }
    }
    return table;
}
