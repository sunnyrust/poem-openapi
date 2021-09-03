use derive_more::Display;
use num_traits::AsPrimitive;

use crate::{registry::MetaValidators, validation::Validator};

#[derive(Display)]
#[display(fmt = "minimum({}, exclusive: {})", n, exclusive)]
pub struct Minimum {
    n: f64,
    exclusive: bool,
}

impl Minimum {
    #[inline]
    pub fn new(n: f64, exclusive: bool) -> Self {
        Self { n, exclusive }
    }
}

impl<T: AsPrimitive<f64>> Validator<T> for Minimum {
    #[inline]
    fn check(&self, value: &T) -> bool {
        if self.exclusive {
            value.as_() > self.n
        } else {
            value.as_() >= self.n
        }
    }

    fn update_meta(&self, meta: &mut MetaValidators) {
        meta.minimum = Some(self.n);
        if self.exclusive {
            meta.exclusive_minimum = Some(true);
        }
    }
}
