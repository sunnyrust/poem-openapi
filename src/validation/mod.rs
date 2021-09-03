use std::fmt::Display;

mod max_items;
mod max_length;
mod maximum;
mod min_items;
mod min_length;
mod minimum;
mod multiple_of;
mod pattern;

pub use max_items::MaxItems;
pub use max_length::MaxLength;
pub use maximum::Maximum;
pub use min_items::MinItems;
pub use min_length::MinLength;
pub use minimum::Minimum;
pub use multiple_of::MultipleOf;
pub use pattern::Pattern;

use crate::registry::MetaValidators;

pub trait Validator<T>: Display {
    fn check(&self, value: &T) -> bool;

    fn update_meta(&self, meta: &mut MetaValidators);
}
