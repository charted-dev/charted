// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

/// Asserts that 1..N of types implements [`axum::response::IntoResponse`].
///
/// ## Example
/// ```
/// use charted_core::assert_into_response;
///
/// // will compile
/// assert_into_response!(String);
///
/// struct DoesntImplIntoResponse;
/// #
/// // will fail if it doesn't implement `IntoResponse`
/// // assert_into_response!(DoesntImplIntoResponse);
/// ```
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
#[macro_export]
macro_rules! assert_into_response {
    ($($T:ty),+) => {
        const _: () = {
            const fn __asserts_into_response<T: ::axum::response::IntoResponse>() {}
            $(__asserts_into_response::<$T>();)*
        };
    };
}

/// Assertion macro that expects the response is a OK response.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
#[macro_export]
macro_rules! assert_response_ok {
    ($res:expr, $message:literal $(,)?) => {
        ::core::assert!(($res).status().is_success(), $message);
    };

    ($res:expr, $message:literal, $($arg:tt),*) => {
        ::core::assert!(($res).status().is_success(), $message, $($arg),*);
    };

    ($res:expr) => {
        ::core::assert!(($res).status().is_success());
    };
}

/// Assertion macro that expects the response is a error response that
/// the user that made this request.
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
#[macro_export]
macro_rules! assert_response_is_client_error {
    ($res:expr, $message:literal $(,)?) => {
        ::core::assert!(($res).status().is_client_error(), $message);
    };

    ($res:expr, $message:literal, $($arg:tt),*) => {
        ::core::assert!(($res).status().is_client_error(), $message, $($arg),*);
    };

    ($res:expr) => {
        ::core::assert!(($res).status().is_client_error());
    };
}

/// Assertion macro that expects the response to return a specific status code
#[cfg(feature = "axum")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "axum")))]
#[macro_export]
macro_rules! assert_status_code {
    ($res:expr, $code:ident) => {
        ::core::assert_eq!(($res).status(), ::axum::http::StatusCode::$code);
    };

    ($res:expr, $code:ident, $message:literal $(,)?) => {
        ::core::assert_eq!(($res).status(), ::axum::http::StatusCode::$code, $message);
    };

    ($res:expr, $code:ident, $message:literal, $($arg:tt),*) => {
        ::core::assert_eq!(($res).status(), ::axum::http::StatusCode::$code, $message, $($arg),*);
    };

    ($res:expr, $code:expr) => {
        ::core::assert_eq!(($res).status(), $code);
    };

    ($res:expr, $code:expr, $message:literal $(,)?) => {
        ::core::assert_eq!(($res).status(), $code, $message);
    };

    ($res:expr, $code:expr, $message:literal, $($arg:tt),*) => {
        ::core::assert_eq!(($res).status(), $code, $message, $($arg),*);
    };
}

/// Asserts that 1..N of types implement [`Send`].
///
/// ## Example
/// ```
/// use charted_core::assert_send;
///
/// assert_send!(String);
/// // compile fail:
/// // assert_send!(std::rc::Rc<()>);
/// ```
#[macro_export]
macro_rules! assert_send {
    ($($T:ty),+) => {
        const _: () = {
            const fn __asserts_send<T: Send>() {}
            $(__asserts_send::<$T>();)*
        };
    };
}

/// Asserts that 1..N of types implement [`Sync`].
///
/// ## Example
/// ```
/// use charted_core::assert_sync;
///
/// assert_sync!(&String);
/// // compile fail:
/// // assert_sync!(std::rc::Rc<()>);
/// ```
#[macro_export]
macro_rules! assert_sync {
    ($($T:ty),+) => {
        const _: () = {
            const fn __asserts_sync<T: Sync>() {}
            $(__asserts_sync::<$T>();)*
        };
    };
}

/// Asserts that 1..N of types implement [`Send`] + [`Sync`].
///
/// ## Example
/// ```
/// use charted_core::assert_send_and_sync;
///
/// assert_send_and_sync!(String);
/// // compile fail:
/// // assert_send_and_sync!(std::rc::Rc<()>);
/// ```
#[macro_export]
macro_rules! assert_send_and_sync {
    ($($T:ty),+) => {
        const _: () = {
            const fn __asserts_send_and_sync<T: Send + Sync>() {}
            $(__asserts_send_and_sync::<$T>();)*
        };
    };
}
