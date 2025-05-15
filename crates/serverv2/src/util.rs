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

use std::{
    fmt::{self, Write},
    iter,
};
use url::Url;

/// Options object for [`build_link_header`]
#[derive(Debug)]
pub struct BuildLinkHeaderOpts {
    /// Amount of entries avaliable.
    pub entries: usize,

    /// Current page that we are on.
    pub current: usize,

    /// How many elements per page are avaliable.
    pub per_page: usize,

    /// Maximum amount of pages returned from the database.
    pub max_pages: u64,

    /// Resource URL to extend to include additional query parameters.
    pub resource: Url,
}

/// Builds a `Link` header for paginated results, which is based off how GitHub
/// [does pagination with the `Link` header].
///
/// [does pagination with the `Link` header]: https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api?apiVersion=2022-11-28#using-link-headers
///
/// **NOTE**: The `per_page` query parameter will be avaliable if `per_page` is not 10.
///
/// ## Example
/// ```
/// # use charted_serverv2::util::{self, BuildLinkHeaderOpts};
/// # use url::Url;
/// #
/// // build a buffer since `build_link_header` uses the `write!` macro.
/// let mut buf = String::new();
/// util::build_link_header(&mut buf, BuildLinkHeaderOpts {
///     // resource URI to point to when creating the URI references encapsulated in `<>`.
///     resource: Url::parse("https://charts.noelware.org/api/users/@me/apikeys").unwrap(),
///
///     // amount of entries
///     entries: 1,
///
///     // how many elements per page are avaliable
///     per_page: 5,
///
///     // current page that we are on
///     current: 1,
///
///     // maximum amount of pages avaliable
///     max_pages: 0,
/// });
///
/// // The writer can produce a empty buffer if:
/// //     - entries is zero
/// //     - entries < per_page (this is our case based off the params we set)
/// assert!(buf.is_empty());
///
/// // let's try again with different params
/// util::build_link_header(&mut buf, BuildLinkHeaderOpts {
///     // resource URI to point to when creating the URI references encapsulated in `<>`.
///     resource: Url::parse("https://charts.noelware.org/api/users/@me/apikeys").unwrap(),
///
///     // amount of entries
///     entries: 10,
///
///     // how many elements per page are avaliable
///     per_page: 10,
///
///     // current page that we are on
///     current: 5,
///
///     // maximum amount of pages avaliable
///     max_pages: 25,
/// });
///
/// assert_eq!(
///     buf,
///     "<https://charts.noelware.org/api/users/@me/apikeys?page=6>; rel=\"next\", <https://charts.noelware.org/api/users/@me/apikeys?page=25>; rel=\"last\", <https://charts.noelware.org/api/users/@me/apikeys?page=4>; rel=\"prev\", <https://charts.noelware.org/api/users/@me/apikeys?page=1>; rel=\"first\""
/// );
/// ```
pub fn build_link_header<W: Write>(
    writer: &mut W,
    BuildLinkHeaderOpts {
        current,
        entries,
        max_pages,
        per_page,
        resource,
    }: BuildLinkHeaderOpts,
) -> fmt::Result {
    // If there is no entries, then don't being to write anything.
    if entries == 0 {
        return Ok(());
    }

    // If there is less entries than per page, don't do anything as going through
    // more pages than there is avaliable is probably going to break things or
    // flat out return nothing.
    if entries < per_page {
        return Ok(());
    }

    {
        let next = current + 1;
        let mut param = format!("page={next}");

        if per_page != 10 {
            param.extend(iter::once(format!("&per_page={per_page}")));
        }

        let mut resource = resource.clone();
        resource.set_query(Some(&param));

        write!(writer, "<{resource}>; rel=\"next\", ")?;
    }

    {
        let mut resource = resource.clone();
        let mut param = format!("page={max_pages}");

        if per_page != 10 {
            param.extend(iter::once(format!("&per_page={per_page}")));
        }

        resource.set_query(Some(&param));

        write!(writer, "<{resource}>; rel=\"last\"")?;
    }

    if current > 1 {
        {
            let prev = current - 1;
            let mut param = format!("page={prev}");

            if per_page != 10 {
                param.extend(iter::once(format!("&per_page={per_page}")));
            }

            let mut resource = resource.clone();
            resource.set_query(Some(&param));

            write!(writer, ", <{resource}>; rel=\"prev\"")?;
        }

        let mut param = String::from("page=1");

        if per_page != 10 {
            param.extend(iter::once(format!("&per_page={per_page}")));
        }

        let mut resource = resource.clone();
        resource.set_query(Some(&param));

        write!(writer, ", <{resource}>; rel=\"first\"")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;
    use url::Url;

    #[test]
    fn valid_entries() {
        let mut buf = String::new();
        super::build_link_header(&mut buf, super::BuildLinkHeaderOpts {
            resource: Url::parse("https://api.github.com/repositories/469212491/issues").unwrap(),
            current: 1,
            entries: 10,
            per_page: 10,
            max_pages: 1000,
        })
        .unwrap();

        assert_eq!(
            buf,
            "<https://api.github.com/repositories/469212491/issues?page=2>; rel=\"next\", <https://api.github.com/repositories/469212491/issues?page=1000>; rel=\"last\""
        );

        buf.clear();

        super::build_link_header(&mut buf, super::BuildLinkHeaderOpts {
            resource: Url::parse("https://api.github.com/repositories/469212491/issues").unwrap(),
            current: 4,
            entries: 25,
            per_page: 25,
            max_pages: 1000,
        })
        .unwrap();

        assert_eq!(
            buf,
            "<https://api.github.com/repositories/469212491/issues?page=5&per_page=25>; rel=\"next\", <https://api.github.com/repositories/469212491/issues?page=1000&per_page=25>; rel=\"last\", <https://api.github.com/repositories/469212491/issues?page=3&per_page=25>; rel=\"prev\", <https://api.github.com/repositories/469212491/issues?page=1&per_page=25>; rel=\"first\""
        );
    }

    #[test]
    fn result_is_header_value_compliant() {
        let mut buf = String::new();
        super::build_link_header(&mut buf, super::BuildLinkHeaderOpts {
            resource: Url::parse("https://api.github.com/repositories/469212491/issues").unwrap(),
            current: 1,
            entries: 10,
            per_page: 10,
            max_pages: 1000,
        })
        .unwrap();

        HeaderValue::from_bytes(buf.as_bytes()).unwrap();

        buf.clear();

        super::build_link_header(&mut buf, super::BuildLinkHeaderOpts {
            resource: Url::parse("https://api.github.com/repositories/469212491/issues").unwrap(),
            current: 4,
            entries: 25,
            per_page: 25,
            max_pages: 1000,
        })
        .unwrap();

        HeaderValue::from_bytes(buf.as_bytes()).unwrap();
    }
}
