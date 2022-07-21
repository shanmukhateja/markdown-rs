use crate::prelude::ToHtml;

use super::ParserExt;

pub struct InlineBold {
    text: String,
}

impl ParserExt for InlineBold {

    fn new_parser(lines: Vec<String>) -> Vec<String> {
        let mut result = lines;

        for line in result.iter_mut() {
            if line.contains("**") {
                let mut token = InlineBold::new(line.clone());
                *line = token.to_html();
            }
        }

        result
    }
}

impl InlineBold {

    fn new(text: String) -> InlineBold {
        InlineBold { text }
    }

    fn parse_inline_bold(&mut self) {
        while self.text.contains("**") {
            let start_index = self.text.find("**").unwrap();

            let strslice_for_last_index: String =
                self.text.get((start_index + 2)..).unwrap().into();
            let last_index = strslice_for_last_index.find("**").unwrap();

            let text_in_bold: String = self
                .text
                .get(start_index + 2..(start_index + 2 + last_index))
                .unwrap()
                .into();

            self.text = self.text.replace(
                format!("**{}**", text_in_bold).as_str(),
                format!("<strong>{}</strong>", text_in_bold).as_str(),
            );
        }
    }
}

impl ToHtml for InlineBold {
    fn to_html(&mut self) -> String {
        let mut clone = InlineBold {
            text: self.text.clone(),
        };

        clone.parse_inline_bold();

        clone.text
    }
}
