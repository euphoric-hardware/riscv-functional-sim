pub fn indent(source: &str, n: usize) -> String {
    source
        .split("\n")
        .enumerate()
        .map(|(idx, line)| {
            if idx == 0 {
                line.to_owned() + "\n"
            } else {
                "    ".repeat(n) + line + "\n"
            }
        })
        .collect()
}

pub fn capitalize_first_letter(s: &str) -> String {
    if let Some(first) = s.chars().next() {
        format!("{}{}", first.to_ascii_uppercase(), &s[1..])
    } else {
        String::new()
    }
}

pub fn rename_field_enum(enum_name: &str, csr_name: &str) -> String {
    let rest = enum_name
        .strip_prefix(csr_name)
        .expect("well-formed field name");
    let camel_rest = rest
        .split('_')
        .map(capitalize_first_letter)
        .collect::<String>();
    format!("{}{}", capitalize_first_letter(csr_name), camel_rest)
}
