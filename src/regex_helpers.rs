use regex::{Captures, Regex};

fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

/*
 Replaces 
 =sum(spread(split(D2, ",")))
 =sum(spread(split(D3, ",")))
 */
pub fn increase_column_digits(text: String, prev_num: u32) -> String {
    let regex_data = format!("[A-Z]{prev_num}+");
    let re = Regex::new(&regex_data).unwrap();
    let replacement = |caps: &Captures| -> Result<String, &'static str> {
        Ok(caps[0][0..=0].to_string() + &(prev_num+1).to_string())
    };
    let response = replace_all(&re, &text, &replacement);
    return response.unwrap();
}
