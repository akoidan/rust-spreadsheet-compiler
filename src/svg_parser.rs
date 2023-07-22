use crate::table::*;
use crate::io_utils::*;
use crate::logic_utils::*;

pub fn lines_to_table(data: Vec<&str>) {
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
                    if !crate::logic_utils::is_head(&column) {
                        panic!("Invalid file structure, some columns are heads, and some not")
                    }
                    let name = extract_head_name(column);
                    current_head.push(name);
                    table.add_column(name, index);
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
    print!("Finished");
}
