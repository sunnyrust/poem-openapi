use std::ops::Deref;

use derive_more::Display;

use crate::{registry::MetaValidators, types::Type, validation::Validator};

#[derive(Display)]
#[display(fmt = "minItems({})", len)]
pub struct MinItems {
    len: usize,
}

impl MinItems {
    #[inline]
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl<T: Deref<Target = [E]>, E: Type> Validator<T> for MinItems {
    #[inline]
    fn check(&self, value: &T) -> bool {
        value.deref().len() >= self.len
    }

    fn update_meta(&self, meta: &mut MetaValidators) {
        meta.min_items = Some(self.len);
    }
}
