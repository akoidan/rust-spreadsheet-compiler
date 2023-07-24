mod table;
mod logic_utils;
mod table_factory;
mod str_utils;
extern crate regex;

#[cfg(test)]
mod tests {
    use crate::logic_utils::LogicExecutor;
    use crate::table_factory::lines_to_table;


    use std::fs::read_to_string;
    use std::path::Path;

    fn fill_table_and_compare_to(from: &str, to: &str) {
        let data = read_to_string(Path::new(from))
            .expect("Cannot open file");
        let data_res = read_to_string(Path::new(to))
            .expect("Cannot open file");
        let mut table = lines_to_table(&data);
        table.fill_data();
        let rendered = table.as_string();
        assert_eq!(data_res, rendered);
    }

    #[test]
    fn test_without_evaluating() {
        fill_table_and_compare_to("./assets/simple_asset.csv", "./assets/simple_asset.res.csv");
    }

    #[test]
    fn test_wqith_simple_evaluation() {
        fill_table_and_compare_to("./assets/transactions_simple.csv", "./assets/transactions_simple.res.csv");
    }

    #[test]
    fn test_given_example() {
        fill_table_and_compare_to("./assets/transactions.csv", "./assets/transactions.res.csv");
    }

}
