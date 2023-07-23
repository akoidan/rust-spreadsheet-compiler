use std::fs::read_to_string;

pub fn file_to_str(path: &str) -> String {
    return read_to_string(path).expect("Cannot open file");
}

pub fn str_to_vector(data: &str, separator: char) -> Vec<&str> {
    return data.split(separator).collect();
}

pub fn substr(string: String, start: u32, end: u32) -> String {
    let mut iter = string.char_indices();
    let (start, _) = iter.nth(start as usize).unwrap();
    let (end, _) = iter.nth(end as usize).unwrap();
    let slice = &string[start..end];
    return String::from(slice);
}
pub fn remove_first_symbol<'a>(s: &'a str) -> &'a str {
    return &s[1..s.len()];
}