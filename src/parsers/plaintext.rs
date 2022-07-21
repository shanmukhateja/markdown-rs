use super::ParserExt;

pub struct PlainText {
    text: String,
}

impl PlainText {
    fn new(text: String) -> Self {
        PlainText { text }
    }

    #[allow(unused_assignments)]
    pub fn wrap_in_p_tag(&mut self, _is_prev_wrapped: bool) -> String {
        let final_text = format!("<p>{}</p>", self.text.clone());

        final_text
    }
}

impl ParserExt for PlainText {
    fn new_parser(mut lines: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = lines
            .iter_mut()
            // strip lines with whitespace
            .filter(|x| !x.trim().is_empty())
            .map(|x| String::from(x.as_str()))
            .collect();

        // Strip codeblock entries and early return if plaintext data found
        let mut code_block_triggered = false;
        
        #[allow(clippy::needless_collect)]
        let result2: Vec<String> = result
            .clone()    
            .iter()
            .filter(|x| {

                if !code_block_triggered {
                    code_block_triggered = x.starts_with("<pre>");
                    false
                } else if code_block_triggered && x.ends_with("</pre>") {
                    code_block_triggered = false;
                    false
                } else if code_block_triggered{
                    false
                } else {
                    code_block_triggered && x.ends_with("</pre>")
                }

            })
            .cloned()
            .collect();

            // println!("final result: {:?}", result2);

            if result2.is_empty() && result.iter().any(|x| x.contains("<pre>")) {
                return result;
            }

        let is_prev_wrapped = true;
        for line in result.iter_mut() {
            *line = line.trim().to_string();

            if !line.is_empty() && !line.starts_with('<') {
                let mut plaintext = PlainText::new(line.to_string());
                *line = plaintext.wrap_in_p_tag(is_prev_wrapped);
            }
        }

        result
    }
}
