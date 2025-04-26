use puremark_rs::parse_markdown;

fn main() {
    let input = "\
# Heading

> This is a quote.
> It has multiple lines.
>
> Another line.

# Fruits

- Apple
- Banana
- Orange

# Steps

1. Wake up
2. Brush teeth
3. Eat breakfast

Normal paragraph here.";

    let output = parse_markdown(input);
    println!("{}", output);
}
