mod table;

#[cfg(test)]
mod tests {
    use crate::table::TableData;

    #[test]
    fn it_works() {
        let a: TableData = TableData {
            columns: vec![]
        };
    }
}