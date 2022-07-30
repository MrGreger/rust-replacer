use std::{collections::HashMap, cell::RefCell};

pub enum Gender {
    Male,
    Female,
}

pub struct GenderReplacement<'a> {
    male: &'a str,
    female: &'a str,
}

pub trait GenderReplacementCollection<'a> {
    fn get_replacement(&self, text_to_replace: &str, gender: Gender) -> Option<&str>;
    fn add_replacent(&mut self, target_text: &'a str, replacement: GenderReplacement<'a>);
}

pub struct DefaltGenderReplacementCollection<'a> {
    replacements: RefCell<HashMap<&'a str, GenderReplacement<'a>>>,
}

impl<'a> GenderReplacementCollection<'a> for DefaltGenderReplacementCollection<'a> {
    fn get_replacement(&self, text_to_replace: &str, gender: Gender) -> Option<&str> {
        let map = self.replacements.borrow();
        let replacement = map.get(text_to_replace);
        if let Some(x) = replacement {
            match gender {
                Gender::Male => Some(x.male),
                Gender::Female => Some(x.female),
            }
        } else {
            None
        }
    }

    fn add_replacent(&mut self, target_text: &'a str, replacement: GenderReplacement<'a>) {
        self.replacements.borrow_mut().insert(target_text, replacement);
    }
}
