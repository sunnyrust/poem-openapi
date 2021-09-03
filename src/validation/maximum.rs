use derive_more::Display;
use num_traits::AsPrimitive;

use crate::{registry::MetaValidators, validation::Validator};

#[derive(Display)]
#[display(fmt = "maximum({}, exclusive: {})", n, exclusive)]
pub struct Maximum {
    n: f64,
    exclusive: bool,
}

impl Maximum {
    #[inline]
    pub fn new(n: f64, exclusive: bool) -> Self {
        Self { n, exclusive }
    }
}

impl<T: AsPrimitive<f64>> Validator<T> for Maximum {
    #[inline]
    fn check(&self, value: &T) -> bool {
        if self.exclusive {
            value.as_() < self.n
        } else {
            value.as_() <= self.n
        }
    }

    fn update_meta(&self, meta: &mut MetaValidators) {
        meta.maximum = Some(self.n);
        if self.exclusive {
            meta.exclusive_maximum = Some(true);
        }
    }
}
