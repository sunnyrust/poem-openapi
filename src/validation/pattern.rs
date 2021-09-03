use std::cell::RefCell;

use derive_more::Display;
use regex::Regex;

use crate::{registry::MetaValidators, validation::Validator};

#[derive(Display)]
#[display(fmt = "pattern(\"{}\")", pattern)]
pub struct Pattern {
    pattern: &'static str,
    re: RefCell<Option<Regex>>,
}

impl Pattern {
    #[inline]
    pub fn new(pattern: &'static str) -> Self {
        Self {
            pattern,
            re: RefCell::new(None),
        }
    }
}

impl<T: AsRef<str>> Validator<T> for Pattern {
    #[inline]
    fn check(&self, value: &T) -> bool {
        let mut re = self.re.borrow_mut();
        if re.is_none() {
            *re = Some(Regex::new(self.pattern).unwrap());
        }
        re.as_ref().unwrap().is_match(value.as_ref())
    }

    fn update_meta(&self, meta: &mut MetaValidators) {
        meta.pattern = Some(self.pattern.to_string());
    }
}
