// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use hex::encode;
use md5::{digest::FixedOutput, Digest, Md5};

/// Computes a MD5-encoded hash from a u8 array.
///
/// ## Warning
/// This method shouldn't be used in any circumstances, this is only here since
/// Gravatar uses MD5 as the hashing algorithm to fetch a user avatar from
/// an email.
///
/// Please use the [`sha256`][sha256] function to do safe and secure hashing
/// of anything.
///
/// ## Example
/// ```no_run
/// # use charted_common::crypto::md5;
/// #
/// let hash = md5("Hello, world!");
/// assert_eq!(hash.as_str(), "6cd3556deb0da54bca060b4c39479839");
/// ```
///
/// [sha256]: crate::crypto::sha256
pub fn md5<I: AsRef<[u8]>>(input: I) -> String {
    let mut hasher: Md5 = Md5::new();
    Digest::update(&mut hasher, input.as_ref());

    let result = hasher.finalize_fixed();
    encode(result.as_slice())
}

#[cfg(test)]
mod tests {
    #[test]
    fn md5() {
        let hash = super::md5("Hello, world!");
        assert_eq!(hash.as_str(), "6cd3556deb0da54bca060b4c39479839");
    }
}
