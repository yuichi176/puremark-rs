pub fn parse_markdown(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.trim().is_empty()) // 空行をスキップ
        .map(|line| {
            if let Some((level, content)) = parse_heading(line) {
                format!("<h{level}>{}</h{level}>", content.trim())
            } else {
                format!("<p>{}</p>", line)
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/*
 * Parses a line of text to determine if it is a heading.
 * Returns the heading level and content if it is a valid heading.
 */
fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let trimmed = line.trim_start();
    let hashes = trimmed.chars().take_while(|&c| c == '#').count();

    if (1..=6).contains(&hashes) && trimmed.chars().nth(hashes) == Some(' ') {
        Some((hashes, &trimmed[hashes + 1..]))
    } else {
        None
    }
}
