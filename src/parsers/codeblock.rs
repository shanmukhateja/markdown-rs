use regex::Regex;

use super::ParserExt;

pub struct Codeblock {}

fn parse_multiline(line: &mut String, mut is_done: bool) -> (&mut String, bool) {
    if line.contains("```") && !is_done {
        is_done = true;

        // strip whitespace
        let whitespace_regex = Regex::new("re").unwrap();
        *line = whitespace_regex.replace(line, "")[..].to_string();

        let lang_suffix_str = line.replace("```", "");

        *line = line.replace("```", "<pre>");
        if !lang_suffix_str.trim().is_empty() {
            *line = format!(
                "{}<code class=\"language-{}\">",
                line.replace(lang_suffix_str.as_str(), ""),
                lang_suffix_str
            );
        } else {
            *line = format!("{}<code>", line);
        }
    } else if line.contains("```") {
        *line = line.replace("```", "</code></pre>");
    }

    (line, is_done)
}

fn parse_inline(line: &mut String) -> &mut String {
    // Converts all '`' to '<code>'
    if line.contains('`') {
        *line = line.replace('`', "<code>");
    }

    // Converts all '<code> ' to '</code> '
    // Ex: Run "<code>npm install<code>" => <code>npm install</code>
    if line.contains("<code> ") {
        *line = line.replace("<code> ", "</code> ");
    }

    line
}

impl ParserExt for Codeblock {
    fn parse(mut lines: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = lines
            .iter_mut()
            // strip lines with whitespace
            .filter(|x| !x.trim().is_empty())
            .map(|x| String::from(x.as_str()))
            .collect();

        let mut is_done = false;
        for (_idx, mut line) in result.iter_mut().enumerate() {
            #[allow(unused_assignments)]
            if line.contains("```") {
                (line, is_done) = parse_multiline(line, is_done);
            } else if line.contains('`') {
                line = parse_inline(line);
            }
        }

        result
    }
}
