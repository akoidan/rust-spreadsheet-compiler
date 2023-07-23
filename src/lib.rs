mod table;
mod logic_utils;
mod str_utils;

#[cfg(test)]
mod tests {
    use crate::table::TableData;
    use crate::logic_utils::LogicExecutor;

    #[test]
    fn parses_single_forumla() {
        let a: TableData = TableData { columns: vec![] };
        a.parse_string(String::from("=text(bte(@adjusted_cost<1>, @cost_threshold<1>)"), 1, String::from("e"));
    }
}
