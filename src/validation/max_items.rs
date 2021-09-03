use std::ops::Deref;

use derive_more::Display;

use crate::{registry::MetaValidators, types::Type, validation::Validator};

#[derive(Display)]
#[display(fmt = "maxItems({})", len)]
pub struct MaxItems {
    len: usize,
}

impl MaxItems {
    #[inline]
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl<T: Deref<Target = [E]>, E: Type> Validator<T> for MaxItems {
    #[inline]
    fn check(&self, value: &T) -> bool {
        value.deref().len() <= self.len
    }

    fn update_meta(&self, meta: &mut MetaValidators) {
        meta.max_items = Some(self.len);
    }
}
