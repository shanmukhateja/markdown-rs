use crate::prelude::ToHtml;


#[allow(unused)]
pub struct Header {
    level: usize,
    text: String,
}

impl Header {
    pub fn new(level: usize, text: String) -> Self {
        Header { level, text }
    }
}

impl ToHtml for Header {
    fn to_html(&self) -> String {
        // Generate '#' chars acc. to header level
        let mut generate_header_by_level = "#".repeat(self.level);
        generate_header_by_level.push(' ');

        // remove header markdown given by user
        let strip_markdown = self.text.replace(generate_header_by_level.as_str(), "");

        // return HTML
        format!(
            "<h{}>{}</h{}>",
            self.level,
            strip_markdown,
            self.level
        )
    }
}