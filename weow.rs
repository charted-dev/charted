#![feature(prelude_import)]
//! # 🐻‍❄️📦 `charted_types`
//! This crate is just a generic crate that exports all newtype wrappers for the
//! API server and database entities.
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod name {
    //! Valid UTF-8 string that can be used for names that can be
    //! addressed by the API server.
    //!
    //! * A **Name** is a wrapper for <code>[`Arc`]\<str\></code> as opposed of
    //!   a [`String`] since a **Name** can be never modified and reflected
    //!   on the database.
    //!
    //! * A **Name** is also URL-encoded safe since we only use alphanumeric characters,
    //!   `-`, `_`, and `~`.
    //!
    //! * A **Name** can never overflow since we require names to have a minimum
    //!   length of 2 and a maximum length of 32.
    use std::{borrow::Cow, sync::Arc};
    const MAX_LENGTH: usize = 32;
    const MIN_LENGTH: usize = 2;
    /// Error type when name validation goes wrong.
    pub enum Error {
        #[display("name was over 32 characters")]
        ExceededLength,
        #[display(
            "invalid character '{}' received (index {} in input: \"{}\")",
            ch,
            at,
            input
        )]
        InvalidCharacter { input: Cow<'static, str>, at: usize, ch: char },
        #[display("name cannot be empty")]
        Empty,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::ExceededLength => {
                    ::core::fmt::Formatter::write_str(f, "ExceededLength")
                }
                Error::InvalidCharacter {
                    input: __self_0,
                    at: __self_1,
                    ch: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "InvalidCharacter",
                        "input",
                        __self_0,
                        "at",
                        __self_1,
                        "ch",
                        &__self_2,
                    )
                }
                Error::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
            }
        }
    }
    #[automatically_derived]
    impl derive_more::Display for Error {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match self {
                Self::ExceededLength => {
                    __derive_more_f
                        .write_fmt(format_args!("name was over 32 characters"))
                }
                Self::InvalidCharacter { input, at, ch } => {
                    __derive_more_f
                        .write_fmt(
                            format_args!(
                                "invalid character \'{0}\' received (index {1} in input: \"{2}\")",
                                ch,
                                at,
                                input,
                            ),
                        )
                }
                Self::Empty => {
                    __derive_more_f.write_fmt(format_args!("name cannot be empty"))
                }
            }
        }
    }
    impl std::error::Error for Error {}
    /// Valid UTF-8 string that can be used for names that can be
    /// addressed by the API server.
    ///
    /// * A **Name** is a wrapper for <code>[`Arc`]\<str\></code> as opposed of
    ///   a [`String`] since a **Name** can be never modified and reflected
    ///   on the database.
    ///
    /// * A **Name** is also URL-encoded safe since we only use alphanumeric characters,
    ///   `-`, `_`, and `~`.
    ///
    /// * A **Name** can never overflow since we require names to have a minimum
    ///   length of 2 and a maximum length of 32.
    pub struct Name(Arc<str>);
    #[automatically_derived]
    impl ::core::fmt::Debug for Name {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Name", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Name {
        #[inline]
        fn clone(&self) -> Name {
            Name(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Name {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Name {
        #[inline]
        fn eq(&self, other: &Name) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Name {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Arc<str>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Name {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Name,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Name {
        #[inline]
        fn cmp(&self, other: &Name) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Name {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl derive_more::Deref for Name {
        type Target = Arc<str>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Name {}
}
pub mod payloads {
    //! Types that can effictively create or patch a object's metadata. Used by
    //! the API server for the `PUT` and `PATCH` REST endpoints.
}
mod entities {}
mod newtypes {
    mod datetime {}
    mod semver {}
    mod ulid {}
    pub use datetime::*;
    pub use semver::*;
    pub use ulid::*;
}
pub use entities::*;
pub use newtypes::*;
#[doc(hidden)]
pub mod __private {
    pub use paste::paste;
}
