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

use noelware_config::{env, merge::Merge, FromEnv};
use serde::{Deserialize, Serialize};

use crate::TRUTHY_REGEX;

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
            filter_query: __default_filter_query(),
            attributes: Attributes::default(),
            starttls: false,
            bind_dn: String::from("uid=%u,dc=domain,dc=com"),
            host: __default_ldap_server(),
        }
    }
}

impl FromEnv for Config {
    type Output = Config;

    fn from_env() -> Self::Output {
        Config {
            insecure_skip_tls_verify: env!("CHARTED_SESSION_LDAP_INSECURE_SKIP_TLS_VERIFY", {
                or_else: false;
                mapper: |val| TRUTHY_REGEX.is_match(&val);
            }),

            schedule_user_updates: env!("CHARTED_SESSION_LDAP_SCHEDULE_USER_UPDATES", {
                or_else: false;
                mapper: |val| TRUTHY_REGEX.is_match(&val);
            }),

            schedule_new_users: env!("CHARTED_SESSION_LDAP_NEW_USERS", {
                or_else: true;
                mapper: |val| TRUTHY_REGEX.is_match(&val);
            }),

            filter_query: env!("CHARTED_SESSION_LDAP_FILTER_QUERY", or_else: __default_filter_query()),
            attributes: Attributes::from_env(),
            starttls: env!("CHARTED_SESSION_LDAP_STARTTLS", {
                or_else: false;
                mapper: |val| TRUTHY_REGEX.is_match(&val);
            }),

            bind_dn: env!("CHARTED_SESSION_LDAP_FILTER_QUERY", or_else: String::from("uid=%u,dc=domain,dc=com")),
            host: env!("CHARTED_SESSION_LDAP_SERVER", or_else: __default_ldap_server()),
        }
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

impl FromEnv for Attributes {
    type Output = Attributes;

    fn from_env() -> Self::Output {
        Attributes {
            display_name: env!("CHARTED_SESSION_LDAP_ATTR_DISPLAY_NAME", or_else: __default_ldap_display_name_attribute()),
            username: env!("CHARTED_SESSION_LDAP_ATTR_USERNAME", or_else: __default_ldap_username_attribute()),
            email: env!("CHARTED_SESSION_LDAP_ATTR_EMAIL", or_else: __default_ldap_email_attribute()),
        }
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
    String::from("http://localhost:389")
}
