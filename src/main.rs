use std::sync::Arc;

use regex::{Captures, Regex};
use ReplacementsCollection::{
    DefaltGenderReplacementCollection, GenderReplacement, GenderReplacementCollection, Gender,
};
use TextAttributes::{TextPart, TextAccessor};
use TextReplacing::{DefaultTextReplacer, TextReplacer};
use TextSplitting::{DefaultTextSpitter, TextSplitter};

mod ReplacementsCollection;
mod TextAttributes;
mod TextReplacing;
mod TextSplitting;

fn main() {
    let sentence = "лол 123 \" \" dsfsd лол лол Лол лОл лоЛ \"";

    let mut replacements_collection = DefaltGenderReplacementCollection::new();
    replacements_collection.add_replacent("лол", GenderReplacement::new("лолий", "лолушка"));

    let mut replacements_collection: Arc<Box<dyn GenderReplacementCollection>> =
        Arc::new(Box::new(replacements_collection));

    let text_splitter = DefaultTextSpitter;
    let text_replacer = DefaultTextReplacer::new(Arc::clone(&replacements_collection));

    let mut text_parts = text_splitter.split(&vec![TextPart::PlainText(sentence.to_string())]);

    for part in &text_parts {
        print!("{}", part.get_text());
    }

    println!("");

    text_replacer.replace(&mut text_parts, &Gender::Male);
    
    for part in text_parts {
        print!("{}", part.get_text());
    }
}
