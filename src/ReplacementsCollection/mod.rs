use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub enum Gender {
    Male,
    Female,
}

pub struct GenderReplacement {
    male: String,
    female: String,
}

impl GenderReplacement {
    pub fn new(male: &str, female: &str) -> Self {
        Self {
            male: male.to_lowercase(),
            female: female.to_lowercase(),
        }
    }

    pub fn get_replacement(&self, gender: &Gender) -> &str {
        match gender {
            Gender::Male => &self.male,
            Gender::Female => &self.female,
        }
    }
}

pub trait GenderReplacementCollection {
    fn get_replacement(&self, text_to_replace: &str) -> Option<Rc<GenderReplacement>>;
    fn add_replacent(&mut self, target_text: &str, replacement: GenderReplacement);
}

pub struct DefaltGenderReplacementCollection {
    replacements: RefCell<HashMap<String, Rc<GenderReplacement>>>,
}

impl DefaltGenderReplacementCollection {
    pub fn new() -> Self {
        Self {
            replacements: RefCell::new(HashMap::new()),
        }
    }
}

impl GenderReplacementCollection for DefaltGenderReplacementCollection {
    fn get_replacement(&self, text_to_replace: &str) -> Option<Rc<GenderReplacement>> {
        let map = self.replacements.borrow();
        let replacement = map.get(text_to_replace);
        if let Some(x) = replacement {
            Some(Rc::clone(x))
        } else {
            None
        }
    }

    fn add_replacent(&mut self, target_text: &str, replacement: GenderReplacement) {
        let key = target_text.to_owned().to_lowercase();

        self.replacements
            .borrow_mut()
            .insert(key, Rc::new(replacement));
    }
}
