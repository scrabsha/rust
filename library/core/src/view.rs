//! Helpers module for exporting the `view_types` macro.

use crate::{fmt, marker::PhantomData};

/// Creates a view type.
/// ```
/// #![feature(view_types, view_type_macro)]
//
/// struct Foo {
///     bar: usize,
///     baz: u32,
/// }
///
/// type FooBar = std::view::view_type!(Foo.{ bar });
/// ```
#[macro_export]
#[rustc_builtin_macro(view_type)]
#[unstable(feature = "view_type_macro", issue = "155938")]
macro_rules! view_type {
    ($($arg:tt)*) => {
        /* compiler built-in */
    };
}

/// Field Set Representing Type
#[unstable(feature = "field_set_representing_type_raw", issue = "none")]
#[lang = "field_set_representing_type"]
#[fundamental]
pub struct FieldSetRepresentingType<T: ?Sized, const ID: u32> {
    // FIXME(scrabsha): i largely copied this from `FieldRepresentingType`, which really cares about
    // being invariant over its `T`. I don't know if we need to keep this property?
    _phantom: PhantomData<fn(T) -> T>,
}

impl<T: ?Sized, const ID: u32> fmt::Debug for FieldSetRepresentingType<T, ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = const { crate::any::type_name::<T>() };

        write!(f, "view_type!({type_name}.{{ #{ID} }})")
    }
}
