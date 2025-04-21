use markdown_parser_rust::parse_markdown;

fn main() {
    let input = "\
# Heading 1

This is a paragraph.

## Heading 2

Another paragraph.";

    let output = parse_markdown(input);
    println!("{}", output);
}
