pub fn extract_head_name(column:  &str) -> &str {
    return &column[1..column.len()];
}

pub fn is_head(col: &str) -> bool {
    return col.starts_with("!");
}
