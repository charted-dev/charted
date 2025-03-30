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

use crate::util;
use azalia::config::{
    env::{self, TryFromEnv},
    merge::Merge,
};
use charted_core::serde::Duration;
use serde::{Deserialize, Serialize};

pub const INSECURE_SKIP_TLS_VERIFY: &str = "CHARTED_SESSIONS_LDAP_INSECURE_SKIP_TLS_VERIFY";
pub const SCHEDULE_USER_UPDATES: &str = "CHARTED_SESSIONS_LDAP_SCHEDULE_USER_UPDATES";
pub const SCHEDULE_NEW_USERS: &str = "CHARTED_SESSIONS_LDAP_SCHEDULE_NEW_USERS";
pub const CONNECT_TIMEOUT: &str = "CHARTED_SESSIONS_LDAP_CONNECT_TIMEOUT";
pub const FILTER_QUERY: &str = "CHARTED_SESSIONS_LDAP_FILTER_QUERY";
pub const STARTTLS: &str = "CHARTED_SESSIONS_LDAP_STARTTLS";
pub const BIND_DN: &str = "CHARTED_SESSIONS_LDAP_BIND_DN";
pub const SERVER: &str = "CHARTED_SESSIONS_LDAP_SERVER";

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// If `true`, then charted-server will try to establish a TLS connection with the
    /// LDAP server without certificate verification. This is not recommended for
    /// production environments.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub insecure_skip_tls_verify: bool,

    /// Schedules a fixed time job (of 10 minutes) to create new charted-server users
    /// based off all queried LDAP users.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub schedule_new_users: bool,

    /// Schedules a fixed time job (of 10 minutes) to update users from the LDAP server if
    /// any attributes change and be reflected in the database.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub schedule_user_updates: bool,

    /// Timeout on when the connection should be dropped due to not being responsive.
    #[serde(default = "__default_conn_timeout")]
    #[merge(strategy = crate::util::merge_duration)]
    pub connect_timeout: Duration,

    /// Query used to authenticate users as. If empty, then `<username>=%u` will be used
    /// as the default bind DN.
    ///
    /// This is represented as a templated string, charted-server will replace the
    /// following variables with what the bind DN should be used as:
    ///
    /// * `<username>`: username, which will be replaced by the username to query as.
    #[serde(default = "__default_filter_query")]
    pub filter_query: String,

    /// Query used to bind users from the LDAP server into charted-server user objects.
    ///
    /// ## Examples
    /// * OpenLDAP/LDAP: `uid=%u,dc=domain,dc=com`
    /// * Active Directory: `%u@domain`
    #[serde(default)]
    pub bind_dn: String,

    /// Allows to connect to the LDAP server with [`STARTTLS`](https://www.openldap.org/doc/admin24/tls.html) enabled.
    #[serde(default)]
    #[merge(strategy = azalia::config::merge::strategy::bool::only_if_falsy)]
    pub starttls: bool,

    /// Configures the attribute mappings of a LDAP user.
    #[serde(default)]
    pub attributes: Attributes,

    /// LDAP server to connect to.
    #[serde(default = "__default_ldap_server")]
    pub server: String,
}

impl TryFromEnv for Config {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        Ok(Config {
            insecure_skip_tls_verify: util::bool_env(INSECURE_SKIP_TLS_VERIFY)?,
            schedule_user_updates: util::bool_env(SCHEDULE_USER_UPDATES)?,
            schedule_new_users: util::bool_env(SCHEDULE_NEW_USERS)?,
            connect_timeout: env::try_parse_or_else(CONNECT_TIMEOUT, __default_conn_timeout())?,
            filter_query: env::try_parse_or_else(FILTER_QUERY, __default_filter_query())?,
            attributes: Attributes::try_from_env()?,
            starttls: util::bool_env(STARTTLS)?,
            bind_dn: env::try_parse_or_else(BIND_DN, String::from("uid=%u,dc=domain,dc=com"))?,
            server: env::try_parse_or_else(SERVER, __default_ldap_server())?,
        })
    }
}

/// List of attributes that charted-server will map to the LDAP server.
#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attributes {
    /// Maps a charted-server username to a LDAP username.
    ///
    /// * OpenLDAP/LDAP: `uid`
    /// * Active Directory: `sAMAccountName`
    #[serde(default = "__default_ldap_username_attribute")]
    pub username: String,

    /// Maps a charted-server `user.display_name` to some attribute. By default, this will
    /// be `displayName`
    #[serde(default = "__default_ldap_display_name_attribute")]
    pub display_name: String,

    /// Maps a charted-server `user.email` to some attribute. By default, this will
    /// be `mail`
    #[serde(default = "__default_ldap_email_attribute")]
    pub email: String,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            display_name: __default_ldap_display_name_attribute(),
            username: __default_ldap_username_attribute(),
            email: __default_ldap_email_attribute(),
        }
    }
}

pub const ATTRIBUTE_DISPLAY_NAME: &str = "CHARTED_SESSIONS_LDAP_ATTR_DISPLAY_NAME";
pub const ATTRIBUTE_USERNAME: &str = "CHARTED_SESSIONS_LDAP_ATTR_USERNAME";
pub const ATTRIBUTE_EMAIL: &str = "CHARTED_SESSIONS_LDAP_ATTR_EMAIL";

impl TryFromEnv for Attributes {
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        Ok(Attributes {
            display_name: env::try_parse_or_else(ATTRIBUTE_DISPLAY_NAME, __default_ldap_display_name_attribute())?,
            username: env::try_parse_or_else(ATTRIBUTE_USERNAME, __default_ldap_username_attribute())?,
            email: env::try_parse_or_else(ATTRIBUTE_EMAIL, __default_ldap_email_attribute())?,
        })
    }
}

fn __default_ldap_username_attribute() -> String {
    String::from("uid")
}

fn __default_ldap_display_name_attribute() -> String {
    String::from("displayName")
}

fn __default_ldap_email_attribute() -> String {
    String::from("mail")
}

fn __default_filter_query() -> String {
    String::from("<username>=%u")
}

fn __default_ldap_server() -> String {
    String::from("ldap://localhost:389")
}

fn __default_duration() -> Duration {
    Duration::from_secs(5)
}

const fn __default_conn_timeout() -> Duration {
    Duration::from_secs(1)
}
