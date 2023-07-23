pub trait StrUtils {
    fn at(self, index: usize) -> char;
    fn split_to_vect(&self, separator: char) -> Vec<&str>;
    fn remove_first_symbol(&self) -> &str;
    fn next_word_length_underscore(&self, start_index: usize) -> usize;
    fn next_word_length(&self, start_index: usize) -> usize;
    fn next_digit_length(&self, start_index: usize) -> usize;
    fn next_quote_length(&self, start_index: usize) -> usize;
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

    fn next_word_length_underscore(&self, start_index: usize) -> usize {
        return self[start_index..]
            .find(|c: char| c != '_' && !c.is_ascii_alphabetic())
            .expect("word doesn't end");
    }

    fn next_word_length(&self, start_index: usize) -> usize {
        return self[start_index..]
            .find(|c: char| !c.is_ascii_alphabetic())
            .expect("word doesn't end");;
    }
    fn next_digit_length(&self, start_index: usize) -> usize {
        return self[start_index..]
            .find(|c: char| !c.is_ascii_digit())
            .expect("digit doesn't end");;
    }

    fn next_quote_length(&self, start_index: usize) -> usize {
        return self[start_index..]
            .find('\"')
            .expect("doesn't have a terminating quite");;
    }
}
