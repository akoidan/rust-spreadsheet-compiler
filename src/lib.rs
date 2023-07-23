mod table;
mod logic_utils;
mod table_factory;
mod str_utils;

#[cfg(test)]
mod tests {
    use crate::table::TableData;
    use crate::logic_utils::LogicExecutor;
    use crate::table_factory::lines_to_table;
    use std::fs::read_to_string;

    fn construct_table() -> TableData {
        let data = read_to_string("/home/andrew/it/my-projects/rust/assets/transactions.csv")
            .expect("Cannot open file");
        return lines_to_table(&data);
    }
    #[test]
    fn parses_single_forumla() {
        let a = construct_table();
        let value = a.parse_string(String::from("=text(bte(@adjusted_cost<1>, @cost_threshold<1>))"), 16, String::from("cost_too_high"));
        println!("{}", value);
    }

    #[test]
    fn parses_single_forumla_2() {
        let a = construct_table();
        let value = a.parse_string(String::from("=concat(\"t_\", text(incFrom(1)))"), 10, String::from("transaction_id"));
        println!("{}", value);
    }

    #[test]
    fn parses_single_forumla_3() {
        let a = construct_table();
        let value = a.parse_string(String::from("=E^v+(E^v*A9)"), 7, String::from("adjusted_cost"));
        println!("{}", value);
    }

}
