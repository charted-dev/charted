// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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

/// This macro helps consume the result of a [`Result<T, E>`][std::result::Result] object
/// and exists the process if anything had happened if [Err] was called.
#[macro_export]
macro_rules! try_get_value {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => {
                ::log::error!("Unable to run command line runner:\n{}", e);
                ::std::process::exit(1);
            }
        }
    };
}
