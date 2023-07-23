mod table_parser;
mod io_utils;
mod logic_utils;
mod table;
mod regex_helpers;

use io_utils::{file_to_str, str_to_vector};
use logic_utils::{LogicExecutor};
use table_parser::{lines_to_table};

extern crate regex;
extern crate substring;


fn main() {
       let data = file_to_str("/home/andrew/it/my-projects/rust/assets/transactions.csv");
       let lines = str_to_vector(&data, '\n');
       let mut table = lines_to_table(lines);
       table.fill_data();

}
