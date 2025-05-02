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
//
//! Allows collecting backtraces and serializing them into JSON.

use serde_json::Value;

/// A list of functions to skip since they don't really matter.
#[cfg(all(debug_assertions, feature = "collect-backtrace-frames"))]
const FUNCTIONS_TO_SKIP: &[&str] = &[
    // "<core::pin::Pin",
    // "<futures_util::future::",
    // "<alloc::boxed::Box",
    // "core::ops::function::Fn",
    // "std::sys::pal",
    // "<unknown>",
    // "start_thread",
    // "__GI",
    // "std::thread::Builder::spawn_",
    // "__rust_try",
    // "std::panicking::try::do_call",
    // "<core::panic::unwind_safe::",
    // "tokio::runtime",
    // "<tokio::runtime",
    // "tokio::loom",
    // "std::thread::local",
    // "<axum::serve",
    // "<core::future",
    // "<hyper_util",
    // "hyper_util",
    // "<hyper",
    // "hyper",
    // "<axum::routing",
    // "<tower",
    // "<tower_http",
    // "<F as futures_core::future::TryFuture>",
    // "<sentry_core::futures",
    // "<sentry_tower::http",
    // "<axum::middleware::from_fn",
    // "axum::middleware::from_fn",
    // "<tracing::instrument::Instrumented",
    // "<axum::handler::future",
    // "<F as axum::handler::Handler",
    // "std::panic",
];

// Ensures that `slice::Iter` is an exact-sized iterator since we want to
// build a `Vec` of JSON values and want to allocate what is avaliable.
#[cfg(all(debug_assertions, feature = "collect-backtrace-frames"))]
const _: () = {
    use core::slice;

    const fn __is_exact_sized_iterator<T: ExactSizeIterator>() {}
    __is_exact_sized_iterator::<slice::Iter<'_, ()>>();
};

#[cfg(all(debug_assertions, feature = "collect-backtrace-frames"))]
#[inline(never)]
#[cold] // system failures should theorically never happen
pub fn collect() -> Value {
    use backtrace::Backtrace;
    use serde_json::json;

    let backtrace = Backtrace::new();
    let mut stack = match sentry_backtrace::backtrace_to_stacktrace(&backtrace) {
        Some(bt) => bt,
        None => return Value::Null,
    };

    stack.frames.reverse();

    let iter = stack.frames.iter().skip(1).filter(|f| {
        // If we can't get the function name, just don't use the frame
        let Some(ref func) = f.function else {
            return false;
        };

        !FUNCTIONS_TO_SKIP.contains(&func.as_str())
    });

    let (len, _) = iter.size_hint();
    let mut data = Vec::with_capacity(len);

    for frame in iter {
        // Safety: the filter in `iter` returns `false` if this is `None`.
        let func = unsafe { frame.function.as_deref().unwrap_unchecked() };

        data.push(json!({
            "function": func,
            "file": frame.abs_path,
            "line": frame.lineno,
        }));
    }

    Value::Array(data)
}

#[cfg(not(all(debug_assertions, feature = "collect-backtrace-frames")))]
#[allow(dead_code)] // it'll be optimized out as a `serde_json::Value`
#[inline(never)]
#[cold]
pub const fn collect() -> Value {
    Value::Null
}
