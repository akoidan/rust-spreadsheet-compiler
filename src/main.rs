mod logic_utils;
mod regex_helpers;
mod str_utils;
mod table;
mod table_factory;

use logic_utils::LogicExecutor;
use std::fs::read_to_string;
use str_utils::StrUtils;
use table_factory::lines_to_table;

extern crate regex;
extern crate substring;

fn main() {
    let data = read_to_string("/home/andrew/it/my-projects/rust/assets/transactions.csv")
        .expect("Cannot open file");
    let mut table = lines_to_table(&data);
    table.fill_data();
}
