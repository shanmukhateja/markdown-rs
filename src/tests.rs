use crate::parsers::run_parser;

#[test]
fn test_header3() {
    let input = "### Heading 3".to_string();

    let expected = "<h3>Heading 3</h3>";

    assert_eq!(run_parser(vec![input]), vec![expected]);
}

#[test]
fn test_header2() {
    let input = "## Heading 2".to_string();

    let expected = "<h2>Heading 2</h2>";

    assert_eq!(run_parser(vec![input]), vec![expected]);
}

#[test]
fn test_header1() {
    let input = "# Heading 1".to_string();

    let expected = "<h1>Heading 1</h1>";

    assert_eq!(run_parser(vec![input]), vec![expected]);
}

#[test]
fn test_header1_with_plaintext() {
    let input_str = "# Heading 1\nI am here!".to_string();

    let expected = vec!["<h1>Heading 1</h1>", "<p>I am here!</p>"];

    let input: Vec<String> = input_str.lines().map(|f| f.to_string()).collect();

    assert_eq!(run_parser(input), expected);
}

#[test]
fn test_inline_bold() {
    let input_str =
    "It is **mandatory** to wear school uniform or you will **not be allowed** into class.

    --
    
    Management";

    let expected = vec!["<p>It is <strong>mandatory</strong> to wear school uniform or you will <strong>not be allowed</strong> into class.</p>","<p>--</p>","<p>Management</p>"];

    let input: Vec<String> = input_str.lines()
    .map(|f| f.to_string())
    .collect();

    assert_eq!(run_parser(input), expected);
}

#[test]
fn plaintext_no_markdown_test() {
    let input_str = "This is a test.";

    let input: Vec<String> = input_str.lines().map(|f| f.to_string()).collect();

    let expected = vec!["<p>This is a test.</p>"];

    assert_eq!(run_parser(input), expected);
}

#[test]
fn codeblock_test() {
    let input_str = 
    "```

    cd /tmp
    ls *.json
    
    ```";

    let input: Vec<String> = input_str.lines().map(|f| f.to_string()).collect();

    let expected = "<pre><code>
    cd /tmp
    ls *.json
    </code></pre>".split_terminator('\n')
    .collect::<Vec<&str>>();

    assert_eq!(run_parser(input), expected);
}

#[test]
fn codeblock_with_lang_test() {
    let input_str = "```sh
    cargo run
    ls -l
    ```";

    let input: Vec<String> = input_str.lines().map(|f| f.to_string()).collect();

    let expected = vec![
        "<pre><code class=\"language-sh\">",
        "    cargo run",
        "    ls -l",
        "    </code></pre>",
    ];

    assert_eq!(run_parser(input), expected);
}

#[test]
fn inline_codeblock_test() {
    let input_str = "Run `npm install` to install deps.";

    let input: Vec<String> = input_str.lines().map(|f| f.to_string()).collect();

    let expected = vec![
        "<p>Run <code>npm install</code> to install deps.</p>",
    ];

    assert_eq!(run_parser(input), expected);
}

#[test]
fn inline_codeblock_test2() {
    let input_str = "Run `mkdir foo` before running `npm install` or it will throw.";

    let input: Vec<String> = input_str.lines().map(|f| f.to_string()).collect();

    let expected = vec![
        "<p>Run <code>mkdir foo</code> before running <code>npm install</code> or it will throw.</p>",
    ];

    assert_eq!(run_parser(input), expected);
}
