use self::{bold::InlineBold, codeblock::Codeblock, header::Header, plaintext::PlainText};

pub mod bold;
pub mod codeblock;
pub mod header;
pub mod plaintext;

pub trait ParserExt {
    fn new_parser(lines: Vec<String>) -> Vec<String>;
}

#[allow(dead_code)]
pub fn run_parser(mut lines: Vec<String>) -> Vec<String> {
    let result: Vec<&mut String> = lines.iter_mut().filter(|x| !x.is_empty()).collect();
    let mut result: Vec<String> = result.iter().map(|f| f.to_string()).collect();

    result = Header::new_parser(result);

    result = InlineBold::new_parser(result);

    result = Codeblock::new_parser(result);

    result = PlainText::new_parser(result);

    result
}
