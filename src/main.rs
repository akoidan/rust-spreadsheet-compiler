mod svg_parser;
mod io_utils;
mod logic_utils;
mod table;

use io_utils::*;
use svg_parser::*;

fn main() {
       let data = file_to_str("/home/andrew/it/my-projects/rust/assets/transactions.csv");
       let lines = str_to_vector(&data, '\n');
       let table = lines_to_table(lines);

}
