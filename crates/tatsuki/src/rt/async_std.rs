// 🐻‍❄️🪆 tatsuki: Dead simple job scheduling library
// Copyright (c) 2024 Noel Towa <cutie@floofy.dev>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! [`Runtime`][crate::rt::Runtime] implementation using async-std's primitives.

use crate::rt::{Runtime, Sleep};
use std::pin::Pin;

/// [`Runtime`][crate::rt::Runtime] implementation using async-std's primitives.
#[derive(Clone, Default)]
pub struct AsyncStd;

impl Runtime for AsyncStd {
    fn spawn<F: std::future::Future + Send + 'static>(&self, _fut: F)
    where
        F::Output: Send + 'static,
    {
        unimplemented!()
    }

    fn sleep(&self, _duration: std::time::Duration) -> Pin<Box<dyn Sleep>> {
        unimplemented!()
    }
}
