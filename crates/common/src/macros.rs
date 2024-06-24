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

/// General macro to implement `Lazy::new` from the `once_cell` library.
#[macro_export]
macro_rules! lazy {
    ($init:expr) => {{
        ::once_cell::sync::Lazy::new(|| $init)
    }};

    ($ty:ty) => {{
        ::once_cell::sync::Lazy::<$ty>::default()
    }};
}

/// General macro to create a [regular expression][regex::Regex] that you know
/// is valid.
#[macro_export]
macro_rules! regex {
    ($e:expr) => {
        ::regex::Regex::new($e).unwrap()
    };
}

#[macro_export]
macro_rules! box_pin {
    ([$($i:ident: copyable $fragment:expr),*] $code:block) => {
        Box::pin(async move {
            $(
                let $i = $fragment;
            )*

            let __ret = $code;
            __ret
        })
    };

    ([$($i:ident: $fragment:expr),*] $code:block) => {
        Box::pin(async move {
            $(
                let $i = ::std::clone::Clone::clone(&$fragment);
            )*

            let __ret = $code;
            __ret
        })
    };

    ($code:block) => {
        Box::pin(async move $code)
    };
}

#[cfg(test)]
mod tests {
    // while this is a valid clippy lint; for this case, we don't really care
    // as we don't even have a executor who will execute it and we don't need it.
    #[allow(clippy::let_underscore_future)]
    #[test]
    fn box_pin_macro_examples() {
        let _ = crate::box_pin!({
            let _ = 0;
        });

        // ---- this will clone `x`, rather than copy it ----
        let x: String = "weow fluff".into();
        let _ = crate::box_pin!([x: x] {
            let _ = x.parse::<u8>();
        });

        // ---- this will perform a copy of `y` into the future instead of cloning it ----
        let y = "hello world";
        let _ = crate::box_pin!([y: copyable y] {
            let _ = y.parse::<u8>();
        });
    }
}
