use derive_more::Display;
use num_traits::AsPrimitive;

use crate::{registry::MetaValidators, validation::Validator};

#[derive(Display)]
#[display(fmt = "multipleOf({})", n)]
pub struct MultipleOf {
    n: f64,
}

impl MultipleOf {
    #[inline]
    pub fn new(n: f64) -> Self {
        Self { n }
    }
}

impl<T: AsPrimitive<f64>> Validator<T> for MultipleOf {
    #[inline]
    fn check(&self, value: &T) -> bool {
        value.as_() % self.n as f64 == 0.0
    }

    fn update_meta(&self, meta: &mut MetaValidators) {
        meta.multiple_of = Some(self.n);
    }
}
