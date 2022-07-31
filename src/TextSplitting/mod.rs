use crate::TextAttributes::{TextAccessor, TextPart};

pub trait TextSplitter {
    fn split(&self, parts: &Vec<TextPart>) -> Vec<TextPart>;
}

pub struct DefaultTextSpitter;

impl TextSplitter for DefaultTextSpitter {
    fn split(&self, parts: &Vec<TextPart>) -> Vec<TextPart> {
        let mut result: Vec<TextPart> = Vec::new();

        for part in parts {
            let mut part_cursor = 0;
            let part_chars: Vec<char> = part.get_text().chars().collect();

            while part_cursor != part_chars.len() {
                let result_part = self.process_simple_text(&part_chars, &mut part_cursor);

                if let Some(x) = result_part {
                    result.push(x);
                }

                let result_part = self.process_quoted_text(&part_chars, &mut part_cursor);

                if let Some(x) = result_part {
                    result.push(x);
                }
            }
        }

        result
    }
}

impl DefaultTextSpitter {
    fn process_simple_text(
        &self,
        source_part: &Vec<char>,
        part_cursor: &mut usize,
    ) -> Option<TextPart> {
        let mut result = String::with_capacity(source_part.len());

        loop {
            if *part_cursor >= source_part.len() {
                break;
            }
            let char = source_part.get(*part_cursor).unwrap();
            
            if *char == '"' {
                break;
            }

            result.push(*char);

            *part_cursor += 1;
        }

        if result.len() > 0 {
            return Some(TextPart::PlainText(result));
        }

        None
    }

    fn process_quoted_text(
        &self,
        source_part: &Vec<char>,
        part_cursor: &mut usize,
    ) -> Option<TextPart> {
        let mut result = String::with_capacity(source_part.len());

        let mut quotes_counter = 0;

        loop {
            if *part_cursor >= source_part.len() {
                break;
            }

            if (quotes_counter == 2) {
                break;
            }

            let char = source_part.get(*part_cursor).unwrap();

            if *char == '"' {
                quotes_counter += 1;
            }

            result.push(*char);

            *part_cursor += 1;
        }

        if result.len() > 0 {
            return Some(TextPart::QuoteText(result));
        }

        None
    }
}
