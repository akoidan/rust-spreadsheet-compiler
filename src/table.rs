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
    fn get_by_name_unmut<'a>(&'a self, name: &str) -> &'a Column;
    fn get_by_letter_unmut<'a>(&'a self, letter: &str) -> &'a Column;
}

impl TableDataGetter for TableData {
    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &mut self.columns[index.expect(&format!("column {name} doesnt exist"))];
    }
    fn get_by_letter<'a>(&'a mut self, letter: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.letter == letter);
        return &mut self.columns[index.expect(&format!("column by letter {letter} doesnt exist"))];
    }

    fn get_by_name_unmut<'a>(&'a self, name: &str) -> &'a Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &self.columns[index.expect( &format!("column {name} doesnt exist"))];
    }
    fn get_by_letter_unmut<'a>(&'a self, letter: &str) -> &'a Column {
        let index = self.columns.iter().position(|x| x.letter == letter);
        return &self.columns[index.expect(&format!("column by letter {letter} doesnt exist"))];
    }
}
