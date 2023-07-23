use std::collections::HashMap;

pub struct Column {
    pub name: String,
    pub letter: String,
    pub values: HashMap<u32, String>,
}

pub struct TableData {
    pub columns: Vec<Column>,
}

pub trait TableDataGetter {
    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column;
    fn get_by_letter<'a>(&'a mut self, letter: &str) -> &'a mut Column;
}

impl TableDataGetter for TableData {
    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &mut self.columns[index.expect("column doesnt exist")];
    }
    fn get_by_letter<'a>(&'a mut self, letter: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.letter == letter);
        return &mut self.columns[index.expect("column doesnt exist")];
    }
}