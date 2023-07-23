use std::collections::HashMap;

pub struct Column {
    pub name: String,
    pub letter: char,
    pub values: HashMap<u32, String>,
}

pub struct TableData {
    pub columns: Vec<Column>,
}

pub trait ColumnGetter {
    fn get_sorted_keys(&self) -> Vec<u32>;
    fn get_cell_by_index(&self, index: usize) -> String;
}

impl ColumnGetter for Column {
    fn get_sorted_keys(&self) -> Vec<u32> {
        let mut new_keys: Vec<u32> = self.values
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<u32>>();
        new_keys.sort();
        return new_keys;
    }

    fn get_cell_by_index(&self, index: usize) -> String {
        // @adjusted_cost<1> this references to first element which indexing start at 0,
        // this is why -1
        let key = self.get_sorted_keys()[index - 1];
        return self.values.get(&key).expect(&format!("key by index {} not found", index)).to_string();
    }
}

pub trait TableDataGetter {
    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column;
    fn get_by_name_unmut<'a>(&'a self, name: &str) -> &'a Column;
    fn add_column(&mut self, name: &str, index: usize);
    fn get_by_letter_unmut<'a>(&'a self, letter: char) -> &'a Column;
}

impl TableDataGetter for TableData {
    fn add_column(&mut self, name: &str, index: usize) {
        let letter = char::from(('A' as u8) + (index as u8)); // 0 -> A, 1 -> B, ...
        assert!(!self.columns.iter().any(|x| x.name == name), "Column {} already exists", name);
        self.columns.push(Column {
            letter,
            name: String::from(name),
            values: HashMap::new(),
        });
    }

    fn get_by_name<'a>(&'a mut self, name: &str) -> &'a mut Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &mut self.columns[index.expect(&format!("column {name} doesnt exist"))];
    }

    fn get_by_name_unmut<'a>(&'a self, name: &str) -> &'a Column {
        let index = self.columns.iter().position(|x| x.name == name);
        return &self.columns[index.expect(&format!("column {name} doesnt exist"))];
    }
    fn get_by_letter_unmut<'a>(&'a self, letter: char) -> &'a Column {
        let index = self.columns.iter().position(|x| x.letter == letter);
        return &self.columns[index.expect(&format!("column by letter {letter} doesnt exist"))];
    }
}
