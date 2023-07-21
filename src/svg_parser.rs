use std::collections::HashMap;

pub struct Column {
    name: String,
    letter: String,
    values: HashMap<u32, String>,
}

pub struct TableData {
    columns: Vec<Column>,
}

pub trait TableDataGetter {
    fn add_column(&mut self, name: &str, index: u8);
    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column;
    fn get_by_letter<'a>(&'a mut self, letter: &str) -> &'a mut Column;
}

impl TableDataGetter for TableData {
    fn add_column(&mut self, name: &str, index: u8) {


        // char::fr
        let letter = char::from(('A' as u8) + index); // 0 -> A, 1 -> B, ...

        if self.columns.iter().any(|x| x.name == name) {
            panic!("Column {name} already exists");
        }
        let c = Column {
            letter: String::from(letter.to_string()),
            name: String::from(name),
            values: HashMap::new(),
        };
        self.columns.push(c);
    }

    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &mut self.columns[index.expect("column doesnt exist")];
    }
    fn get_by_letter<'a>(&'a mut self, letter: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.letter == letter);
        return &mut self.columns[index.expect("column doesnt exist")];
    }
}

pub fn lines_to_table(data: Vec<&str>) {
    let mut table: TableData = TableData { columns: vec![] };

    let mut current_head: Vec<&str> = vec![];

    let mut row_index = 1;

    for line in data {
        let columns = crate::io_utils::str_to_vector(line, '|');
        let current_is_head: bool = crate::logic_utils::is_head(columns[0]);

        if current_is_head {
            current_head.clear();
            let mut index = 0;
            for column in columns {
                if column == "" {
                    continue;
                }
                if !crate::logic_utils::is_head(&column) {
                    panic!("Invalid file structure, some columns are heads, and some not")
                }
                let name = crate::logic_utils::extract_head_name(column);
                current_head.push(name);
                table.add_column(name, index);
                index += 1;
            }
        } else {
            let mut index = 0;
            for column in columns {
                if column == "" {
                    continue;
                }
                let name = current_head[index];
                table
                    .get_by_name(name)
                    .values
                    .insert(row_index, String::from(column));
                index += 1;
            }
        }
        row_index += 1;
    }
    print!("Finished");
}
