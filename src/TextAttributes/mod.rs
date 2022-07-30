pub trait TextAccessor {
    fn get_text(&self) -> &str;
    fn set_text(&mut self, replacement_text: String);
}

pub enum TextPart {
    QuoteText(String),
    PlainText(String),
}

impl TextAccessor for TextPart {
    fn get_text(&self) -> &str {
        match self {
            TextPart::QuoteText(x) => x,
            TextPart::PlainText(x) => x,
        }
    }

    fn set_text(&mut self, replacement_text: String) {
        match self {
            TextPart::QuoteText(x) => {
                *x = replacement_text;
            }
            TextPart::PlainText(x) => {
                *x = replacement_text;
            },
        }
    }
}
