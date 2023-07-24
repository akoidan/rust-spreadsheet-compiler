extern crate regex;

use std::env;
use std::fs::read_to_string;

use logic_utils::LogicExecutor;
use table::TableDataGetter;
use table_factory::lines_to_table;

mod logic_utils;
mod str_utils;
mod table;
mod table_factory;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Please pass path to the csv file");
    let data = read_to_string(&args[1])
        .expect("Cannot open file");
    let mut table = lines_to_table(&data);
    table.fill_data();
    println!("{}", table.as_string());
}
