mod svg_parser;
mod io_utils;
mod logic_utils;

fn main() {
       let data = io_utils::file_to_str("/home/andrew/it/my-projects/rust/assets/transactions.csv");
       let lines = io_utils::str_to_vector(&data, '\n');
       let table = svg_parser::lines_to_table(lines);

}
