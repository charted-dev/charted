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

use std::any::Any;

/// Generic trait to implement `as_any` to help aid using [`Any`]
/// easier.
pub trait AsAny: private::Sealed + Any {
    /// Transforms a reference of `self` into a reference of `dyn Any`.
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Trait to support downcasting, which you can transform
pub trait Cast: private::Sealed + AsAny {
    fn cast<T: AsAny>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}

impl<T: ?Sized + AsAny> Cast for T {}

mod private {
    use super::AsAny;

    pub trait Sealed {}

    impl<T: ?Sized + AsAny> Sealed for T {}
}
