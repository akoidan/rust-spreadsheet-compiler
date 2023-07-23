pub trait StrUtils {
    fn at(self, index: usize) -> char;
    fn split_to_vect(&self, separator: char) -> Vec<&str>;
    fn remove_first_symbol(&self) -> &str;
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
}
