mod logic_utils;
mod str_utils;
mod table;
mod table_factory;

use logic_utils::LogicExecutor;
use std::fs::read_to_string;
use table_factory::lines_to_table;
use table::TableDataGetter;

use std::env;

extern crate regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Please pass path to the csv file");
    let data = read_to_string(&args[1])
        .expect("Cannot open file");
    let mut table = lines_to_table(&data);
    table.fill_data();
    println!("{}", table.as_string());
}
