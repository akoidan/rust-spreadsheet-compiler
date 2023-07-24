use crate::str_utils::StrUtils;
use crate::table::{TableData, TableDataGetter};

/// check if current cell indicates a head, e.g. !date
fn is_head(col: &str) -> bool {
    return col.starts_with("!");
}

/// creates TableData from csv file content.
/// evaluation is not permitted here
pub fn lines_to_table(csv_str: &str) -> TableData {
    let rows: Vec<&str> = csv_str.split_to_vect('\n');
    let mut table: TableData = TableData { columns: vec![] };
    let mut current_head: Vec<String> = vec![];

    for row_index in 0..rows.len() {
        let columns = rows[row_index].split_to_vect('|');
        let current_is_head: bool = is_head(columns[0].trim());
        if current_is_head {
            current_head.clear();
            for column_index in 0..columns.len() {
                let column = columns[column_index].trim();
                if column == "" {
                    continue;
                }
                assert!(is_head(column), "Invalid head at index {}", row_index);
                current_head.push(String::from(column.remove_first_symbol()));
                table.add_column(column.remove_first_symbol(), column_index);
            }
        } else {
            for column_index in 0..columns.len() {
                let column = columns[column_index].trim();
                if column == "" {
                    continue;
                }
                let x = table.get_by_name(&current_head[column_index]);
                x.string_values.insert(row_index as u32 + 1, String::from(column));
            }
        }
    }
    return table;
}
