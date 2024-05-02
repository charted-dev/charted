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

use crate::caching;
use charted_common::serde::Duration;
use noelware_config::merge::Merge;
use serde::Serialize;

/// Default limit for the amount of requests for unauthorized users. Authorized users get
/// a higher ratelimit than unauthorized users.
pub const MAX_REQUESTS_FOR_UNAUTHORIZED_USERS: u16 = 50;

/// Default limit for the amount of requests for authorized users.
pub const MAX_REQUESTS_FOR_AUTHORIZED_USERS: u16 = 1000;

/// Represents the configuration for configuring server-side ratelimits which will
/// ratelimit each authorized user or IP address based off the rules that are set.
///
/// By default, it'll use a TTL-based cache strategy that you can configure if it
/// doesn't exceed over 24 hours or under 15 minutes as lower durations can lead
/// to unnecessary requests you don't want.
///
/// All endpoints are ratelimited except `/heartbeat` as it can be used to indicate
/// that the server is healthy and it shouldn't affect any other systems.
///
/// ## Example
/// ```toml file=config/charted.toml
/// [server.ratelimits]
/// # defines how many requests authorized users can make in the span
/// # of `config.server.ratelimits.ttl` until they get a 429 error.
/// max_requests_for_authorized_users = 100
///
/// # same aspect as `max_requests_for_authorized_users`, but for non-authorized
/// # users (which it'll be ratelimited by IP address)
/// max_requests_for_unauthorized_users = 10
///
/// # time-to-live duration on how long ratelimits should expire.
/// ttl = "30m"
///
/// # defines the caching strategy for ratelimits
/// [server.ratelimits.caching]
/// # uses the Redis connection to cache ratelimit objects
/// strategy = "redis"
/// ```
#[derive(Debug, Clone, Serialize, Merge)]
pub struct Config {
    /// Defines the amount of maximum requests (in range of `1..65535`) an unauthorized user
    /// can make before hitting a 429.
    #[serde(default = "__default_max_requests_for_unauthorized_users")]
    pub max_requests_for_unauthorized_users: u16,

    /// Defines the amount of maximum requests (in range of `1..65535`) an authorized user
    /// can make before hitting a 429.
    #[serde(default = "__default_max_requests_for_authorized_users")]
    pub max_requests_for_authorized_users: u16,

    /// Caching strategy for all ratelimit objects.
    #[serde(default)]
    pub caching: caching::Config,

    /// time-to-live duration on how long ratelimits should expire. they cannot go higher than 1 day
    /// and lower than 15 minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<Duration>,
}

const fn __default_max_requests_for_unauthorized_users() -> u16 {
    MAX_REQUESTS_FOR_UNAUTHORIZED_USERS
}

const fn __default_max_requests_for_authorized_users() -> u16 {
    MAX_REQUESTS_FOR_AUTHORIZED_USERS
}
