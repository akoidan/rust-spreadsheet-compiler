use std::collections::HashMap;

pub struct Column {
    name: String,
    letter: String,
    pub values: HashMap<u32, String>,
}

pub struct TableData {
    pub columns: Vec<Column>,
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