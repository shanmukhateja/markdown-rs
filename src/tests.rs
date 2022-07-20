use crate::parse_markdown;

#[test]
fn no_markdown_gives_text() {
    let input = "foo".into();
    assert_eq!(parse_markdown(&input), input);
}

#[test]
fn header1_markdown_gives_html() {
    let input = "# foo".into();
    assert_eq!(parse_markdown(&input), "<h1>foo</h1>");
}

#[test]
fn header2_markdown_gives_html() {
    let input = "## foobar".into();
    assert_eq!(parse_markdown(&input), "<h2>foobar</h2>");
}

#[test]
fn inline_bold_markdown_gives_html() {
    let input = "**This is in bold**".into();
    assert_eq!(parse_markdown(&input), "<b>This is in bold</b>");
}
