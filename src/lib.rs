mod parsers;
mod prelude;

use parsers::header::Header;
use prelude::ToHtml;

#[cfg(test)]
mod tests;

#[allow(unused)]
fn parse_markdown(s: &String) -> String {
    let text = s.to_owned();

    if s.starts_with("# ") {
        Header::new(1, text).to_html()
    } else if s.starts_with("## ") {
        Header::new(2, text).to_html()
    } else {
        // Plain text
        s.to_owned()
    }
}
