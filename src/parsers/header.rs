use crate::prelude::ToHtml;

use super::ParserExt;

#[allow(unused)]
pub struct Header {
    pub level: usize,
    pub text: String,
}

impl Header {
    fn new(level: usize, text: String) -> Self {
        Header { level, text }
    }

    fn generate_header_text(&self) -> String {
        // Generate '#' chars acc. to header level
        let mut generate_header_by_level = "#".repeat(self.level);
        generate_header_by_level.push(' ');

        generate_header_by_level
    }
}

impl ParserExt for Header {
    fn parse(lines: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = lines;

        for line in result.iter_mut() {
            let mut header = Header::new(0, line.clone());
            if line.contains("### ") {
                header.level = 3;
            } else if line.contains("## ") {
                header.level = 2;
            } else if line.starts_with("# ") {
                header.level = 1;
            }

            // update text
            if header.level != 0 {
                *line = header.to_html();
            }
        }

        result
    }
}

impl ToHtml for Header {
    fn to_html(&mut self) -> String {
        let generate_header_by_level = self.generate_header_text();

        // remove header markdown given by user
        let strip_markdown = self.text.replace(generate_header_by_level.as_str(), "");

        let text_with_header_markdown = format!("{}{}", generate_header_by_level, strip_markdown);

        self.text = self
            .text
            .clone()
            .replace(text_with_header_markdown.as_str(), "");

        // return HTML
        format!("<h{}>{}</h{}>", self.level, strip_markdown, self.level)
    }
}
