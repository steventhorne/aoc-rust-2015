pub fn strip_newline(s: &str) -> &str {
    if let Some(ss) = s.strip_suffix('\n') {
        ss
    } else {
        s
    }
}
