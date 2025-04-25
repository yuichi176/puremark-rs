pub fn parse_markdown(input: &str) -> String {
    let mut lines = input.lines().peekable();
    let mut result = Vec::new();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some((level, content)) = parse_heading(trimmed) {
            result.push(format!("<h{level}>{}</h{level}>", content.trim()));
        } else if trimmed.starts_with('>') {
            result.push(parse_blockquote(trimmed, &mut lines));
        } else if is_list_item(trimmed) {
            result.push(parse_list(trimmed, &mut lines));
        } else if is_ordered_list_item(trimmed) {
            result.push(parse_ordered_list(trimmed, &mut lines));
        } else {
            result.push(format!("<p>{}</p>", trimmed));
        }
    }

    result.join("\n")
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

/*
 * Parses a blockquote from the given line and the following lines.
 * Returns the blockquote as a formatted string.
 */
fn parse_blockquote<'a, I>(first_line: &'a str, lines: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = &'a str>,
{
    let mut blockquote_lines = Vec::new();
    let mut current_line = Some(first_line);

    while let Some(line) = current_line {
        if line.trim_start().starts_with('>') {
            let content = line.trim_start().trim_start_matches('>').trim_start();
            blockquote_lines.push(content.to_string());

            current_line = lines.peek().map(|s| *s);

            if current_line.map_or(false, |s| s.trim_start().starts_with('>')) {
                lines.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let blockquote_content = blockquote_lines.join("\n");
    format!("<blockquote>\n{}</blockquote>", blockquote_content)
}

fn is_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || trimmed.starts_with("+ ")
}

/*
 * Parses a list from the given line and the following lines.
 * Returns the list as a formatted string.
 */
fn parse_list<'a, I>(first_line: &'a str, lines: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = &'a str>,
{
    let mut items = Vec::new();
    let mut current_line = Some(first_line);

    while let Some(line) = current_line {
        let trimmed = line.trim_start();

        if is_list_item(trimmed) {
            let content = trimmed[2..].trim();
            items.push(format!("<li>{}</li>", content));

            current_line = lines.peek().copied();

            if current_line.map_or(false, |s| is_list_item(s.trim_start())) {
                lines.next(); // consume
            } else {
                break;
            }
        } else {
            break;
        }
    }

    format!("<ul>\n{}\n</ul>", items.join("\n"))
}

fn is_ordered_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    let mut chars = trimmed.chars();

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            continue;
        } else if c == '.' {
            return chars.next() == Some(' ');
        } else {
            break;
        }
    }
    false
}

/*
 * Parses an ordered list from the given line and the following lines.
 * Returns the ordered list as a formatted string.
 */
fn parse_ordered_list<'a, I>(first_line: &'a str, lines: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = &'a str>,
{
    let mut items = Vec::new();
    let mut current_line = Some(first_line);

    while let Some(line) = current_line {
        let trimmed = line.trim_start();

        if is_ordered_list_item(trimmed) {
            let content = trimmed
                .splitn(2, ". ")
                .nth(1)
                .unwrap_or("")
                .trim();
            items.push(format!("<li>{}</li>", content));

            current_line = lines.peek().copied();

            if current_line.map_or(false, |s| is_ordered_list_item(s.trim_start())) {
                lines.next(); // consume
            } else {
                break;
            }
        } else {
            break;
        }
    }

    format!("<ol>\n{}\n</ol>", items.join("\n"))
}
