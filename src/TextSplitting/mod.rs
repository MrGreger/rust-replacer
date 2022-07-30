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
            let part_len = part.get_text().chars().count();

            while part_cursor != part_len {
                let result_part = self.process_simple_text(&part, &mut part_cursor, &part_len);

                if let Some(x) = result_part {
                    result.push(x);
                }

                let result_part = self.process_quoted_text(&part, &mut part_cursor, &part_len);

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
        source_part: &TextPart,
        part_cursor: &mut usize,
        part_len: &usize,
    ) -> Option<TextPart> {
        let mut source_text_iter = source_part.get_text().chars();
        let mut result = String::with_capacity(*part_len);

        loop {
            if *part_cursor >= *part_len {
                break;
            }

            let char = source_text_iter.nth(*part_cursor).unwrap();

            if char == '"' {
                break;
            }

            result.push(char);

            *part_cursor += 1;
        }

        if result.len() > 0 {
            return Some(TextPart::PlainText(result));
        }

        None
    }

    fn process_quoted_text(
        &self,
        source_part: &TextPart,
        part_cursor: &mut usize,
        part_len: &usize,
    ) -> Option<TextPart> {
        let mut source_text_iter = source_part.get_text().chars();
        let mut result = String::with_capacity(*part_len);

        let mut quotes_counter = 0;

        loop {
            if *part_cursor >= *part_len {
                break;
            }

            if (quotes_counter == 2) {
                break;
            }

            let char = source_text_iter.nth(*part_cursor).unwrap();

            if char == '"' {
                quotes_counter += 1;
            }

            result.push(char);

            *part_cursor += 1;
        }

        if result.len() > 0 {
            return Some(TextPart::QuoteText(result));
        }

        None
    }
}
