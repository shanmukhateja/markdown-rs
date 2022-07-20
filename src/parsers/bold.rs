use crate::prelude::ToHtml;

pub struct InlineBold {
    text: String
}

impl ToHtml for InlineBold {
    fn to_html(&self) -> String {
        
        let strip_markdown = self.text.replace("**", "");

        format!("<b>{}</b>", strip_markdown)
    }
}