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
