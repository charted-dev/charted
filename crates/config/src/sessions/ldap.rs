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

use azalia::TRUTHY_REGEX;
use charted_common::serde::Duration;
use noelware_config::{env, merge::Merge, TryFromEnv};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, ops::Deref};

#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
pub struct Config {
    /// If `true`, then charted-server will try to establish a TLS connection with the LDAP
    /// server without certificate verification. This is not recommended for production environments.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub insecure_skip_tls_verify: bool,

    /// Schedules a fixed time job (of 10 minutes) to create new charted-server users based off
    /// all queried LDAP users.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub schedule_new_users: bool,

    /// Schedules a fixed time job (of 10 minutes) to update users from the LDAP server if any
    /// attributes change and be reflected in the database.
    #[serde(default)]
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub schedule_user_updates: bool,

    /// Timeout on when the connection should be dropped due to not being responsive.
    #[serde(default = "__default_duration")]
    #[merge(strategy = __duration_strategy)]
    pub conn_timeout: Duration,

    /// Query used to authenticate users as. If empty, then `<username>=%u` will be used as the default
    /// bind DN.
    ///
    /// This is represented as a templated string, charted-server will replace the following variables
    /// with what the bind DN should be used as:
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
    #[merge(strategy = noelware_config::merge::strategy::bool::only_if_falsy)]
    pub starttls: bool,

    /// Configures the attribute mappings of a LDAP user.
    #[serde(default)]
    pub attributes: Attributes,

    /// LDAP server to connect to.
    #[serde(default = "__default_ldap_server")]
    pub host: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            insecure_skip_tls_verify: false,
            schedule_user_updates: false,
            schedule_new_users: true,
            conn_timeout: __default_duration(),
            filter_query: __default_filter_query(),
            attributes: Attributes::default(),
            starttls: false,
            bind_dn: String::from("uid=%u,dc=domain,dc=com"),
            host: __default_ldap_server(),
        }
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Config {
            insecure_skip_tls_verify: env!("CHARTED_SESSION_LDAP_INSECURE_SKIP_TLS_VERIFY", |val| TRUTHY_REGEX.is_match(&val); or false),
            schedule_user_updates: env!("CHARTED_SESSION_LDAP_SCHEDULE_USER_UPDATES", |val| TRUTHY_REGEX.is_match(&val); or false),
            schedule_new_users: env!("CHARTED_SESSION_LDAP_SCHEDULE_NEW_USERS", |val| TRUTHY_REGEX.is_match(&val); or true),
            conn_timeout: charted_common::env(
                "CHARTED_SESSION_LDAP_CONNECTION_TIMEOUT",
                __default_duration(),
                |err| Cow::Owned(err.to_string()),
            )?,

            filter_query: charted_common::env_string("CHARTED_SESSION_LDAP_FILTER_QUERY", __default_filter_query())?,
            attributes: Attributes::try_from_env()?,
            starttls: env!("CHARTED_SESSION_LDAP_STARTTLS", |val| TRUTHY_REGEX.is_match(&val); or false),
            bind_dn: charted_common::env_string(
                "CHARTED_SESSION_LDAP_BIND_DN",
                String::from("uid=%u,dc=domain,dc=com"),
            )?,

            host: charted_common::env_string("CHARTED_SESSION_LDAP_SERVER", __default_ldap_server())?,
        })
    }
}

/// List of attributes that charted-server will map to the LDAP server.
#[derive(Debug, Clone, Merge, Serialize, Deserialize)]
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

impl TryFromEnv for Attributes {
    type Output = Attributes;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(Attributes {
            display_name: charted_common::env_string(
                "CHARTED_SESSION_LDAP_ATTR_DISPLAY_NAME",
                __default_ldap_display_name_attribute(),
            )?,

            username: charted_common::env_string(
                "CHARTED_SESSION_LDAP_ATTR_USERNAME",
                __default_ldap_username_attribute(),
            )?,

            email: charted_common::env_string("CHARTED_SESSION_LDAP_ATTR_EMAIL", __default_ldap_email_attribute())?,
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
    Duration::from(std::time::Duration::from_secs(5))
}

fn __duration_strategy(first: &mut Duration, other: Duration) {
    // don't do anything if it is both
    if first.is_zero() && other.is_zero() {
        return;
    }

    // if `other` is zero, but `first` is non-zero, don't merge
    if !first.is_zero() && other.is_zero() {
        return;
    }

    // if it goes over one minute, don't update it
    if other.deref() > &std::time::Duration::from_secs(60) {
        return;
    }

    // if it is under 1 second, don't do anything
    if other.deref() < &std::time::Duration::from_secs(1) {
        return;
    }

    // otherwise, do update it
    *first = other;
}
