use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use regex::{Captures, Regex};

use crate::{
    ReplacementsCollection::{Gender, GenderReplacementCollection},
    TextAttributes::{TextAccessor, TextPart},
};

pub trait TextReplacer {
    fn replace(&self, parts: &mut Vec<TextPart>, gender: &Gender);
}

pub struct DefaultTextReplacer {
    replacement_collection: Arc<Box<dyn GenderReplacementCollection>>,
    replacers_cache: RegexReplacerCache,
}

impl DefaultTextReplacer {
    pub fn new(replacement_collection: Arc<Box<dyn GenderReplacementCollection>>) -> Self {
        Self {
            replacement_collection,
            replacers_cache: RegexReplacerCache::new(),
        }
    }
}

struct RegexReplacerCache {
    cached_regex: RefCell<HashMap<String, Rc<RegexReplacerKeepCase>>>,
}

struct RegexReplacerKeepCase {
    regex_template: Regex,
}

impl RegexReplacerKeepCase {
    fn replace(&self, source: &str, replacement: &str) -> String {
        self.regex_template
            .replace_all(source, |caps: &Captures| {
                let mut result = String::with_capacity(replacement.len());

                for word_match_option in caps.iter() {
                    if let Some(word_match) = word_match_option {
                        let mut replacement_position = 0;
                        let mut match_chars_iter = word_match.as_str().chars();

                        for replacement_char in replacement.chars() {
                            if let Some(c) = match_chars_iter.nth(replacement_position) {
                                if char::is_uppercase(c) {
                                    for uppercased_char in char::to_uppercase(replacement_char) {
                                        result.push(uppercased_char);
                                    }
                                } else {
                                    result.push(replacement_char);
                                }

                                replacement_position += 1;
                            } else {
                                result.push(replacement_char);
                            }
                        }
                    }
                }

                result
            })
            .to_string()
    }
}

impl RegexReplacerCache {
    fn new() -> Self {
        RegexReplacerCache {
            cached_regex: RefCell::new(HashMap::new()),
        }
    }

    fn get_replacer(&self, replace_template: &str) -> Rc<RegexReplacerKeepCase> {
        if let Some(x) = self.cached_regex.borrow_mut().get(replace_template) {
            return Rc::clone(x);
        }

        let key = replace_template.to_owned();

        let replacer = Rc::new(RegexReplacerKeepCase {
            regex_template: Regex::new(replace_template)
                .expect("Failed to create regex replacement"),
        });

        let result = Rc::clone(&replacer);

        self.cached_regex.borrow_mut().insert(key, replacer);

        result
    }
}

impl TextReplacer for DefaultTextReplacer {
    fn replace(&self, parts: &mut Vec<TextPart>, gender: &Gender) {
        for part in parts {
            if let TextPart::PlainText(_) = part {
                let text = part.get_text().to_owned();

                for word in text.split(' ') {
                    if let Some(replacement_variants) =
                        self.replacement_collection.get_replacement(word)
                    {
                        let replacer = self.replacers_cache.get_replacer(&format!(r"(?i){}", word));

                        part.set_text(RegexReplacerKeepCase::replace(
                            replacer.as_ref(),
                            &text,
                            replacement_variants.get_replacement(gender),
                        ));
                    }
                }
            }
        }
    }
}
