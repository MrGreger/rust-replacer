use std::{collections::HashMap, sync::Arc};

use regex::Regex;

use crate::{
    ReplacementsCollection::GenderReplacementCollection,
    TextAttributes::{TextAccessor, TextPart},
};

pub trait TextReplacer {
    fn replace(&self, parts: &mut Vec<TextPart>);
}

pub struct DefaultTextReplacer<'a> {
    replacement_collection: Arc<Box<dyn GenderReplacementCollection<'a>>>,
}

pub struct RegexReplacerKeepCase<'a> {
    cached_regex: HashMap<&'a str, Regex>,
}

impl<'a> RegexReplacerKeepCase<'a> {
    pub fn new() -> Self{
        RegexReplacerKeepCase { cached_regex: HashMap::new() }
    }

    pub fn replace(&mut self, string_to_replace: &str, replacement_word: &str , replacement: &str) -> String{
        Regex::new(replacement_word).unwrap().replace_all(string_to_replace, replacement).to_string()
    }
}

impl<'a> regex::Replacer for RegexReplacerKeepCase<'a>{
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let word_match = caps.get(0).unwrap().as_str();
        for c in word_match.chars() {
            
        }
    }
}

impl TextReplacer for DefaultTextReplacer<'_> {
    fn replace(&self, parts: &mut Vec<TextPart>) {
        for part in parts {}
    }
}
