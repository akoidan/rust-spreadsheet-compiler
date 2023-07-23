pub trait StrUtils {
    fn at(self, index: usize) -> char;
    fn split_to_vect(&self, separator: char) -> Vec<&str>;
    fn remove_first_symbol(&self) -> &str;
    fn next_word_length_underscore(&self, start_index: usize) -> Option<usize>;
}

impl StrUtils for &str {
    fn at(self, index: usize) -> char {
        return self.chars().nth(index).unwrap();
    }
    fn split_to_vect(&self, separator: char) -> Vec<&str> {
        return self.split(separator).collect();
    }
    fn remove_first_symbol(&self) -> &str {
        return &self[1..self.len()];
    }

    fn next_word_length_underscore(&self, start_index: usize) -> Option<usize> {
        return self[start_index..].find(|c: char| c != '_' && !c.is_ascii_alphabetic());
    }
}
