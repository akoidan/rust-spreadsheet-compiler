use std::fs;


pub fn file_to_str(path: &str) -> String {
    return fs::read_to_string(path).expect("Cannot open file");
}


pub fn str_to_vector(data: &str, separator: char) -> Vec<&str> {
    let mut entries = data.split(separator);
    let mut result = vec![];
    for entry in entries {
        result.push(entry)
    }
    return result;
}