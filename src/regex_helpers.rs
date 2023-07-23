use regex::{Captures, Regex};


pub fn increase_column_digits(text: String, prev_num: u32) -> String {
    return Regex::new(format!("[A-Z]{prev_num}+").as_str())
        .unwrap()
        .replace_all(&text, |caps: &regex::Captures| {
            caps[0][0..=0].to_string() + &(prev_num + 1).to_string()
        }).to_string();
}
