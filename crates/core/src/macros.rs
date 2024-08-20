// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Creates a new "newtype" wrapper that implements the following traits:
///
/// * [`core::ops::Deref`]
/// * [`core::convert::From`]
/// * [`std::fmt::Display`]
///
/// To add more external traits, you can add `#[derive]` at the top of the statement:
/// ```no_run
/// charted_core::create_newtype_wrapper!(
///     /// doc comment is also accepted!
///     #[derive(Debug)]
///     pub S for String;
/// );
///
/// println!("{:?}", S::from(String::from("hello world")));
/// ```
#[macro_export]
macro_rules! create_newtype_wrapper {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident for $ty:ty;
    ) => {
        $(#[$meta])*
        $vis struct $name($ty);

        impl ::core::convert::From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }

        impl ::core::convert::From<$name> for $ty {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = $ty;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::fmt::Display for $name
        where
            $ty: ::std::fmt::Display,
        {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}
