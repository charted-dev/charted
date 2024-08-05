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

use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

pub mod types {
    #[derive(diesel::SqlType)]
    #[diesel(postgres_type(name = "chart_type"))]
    pub struct ChartType;
}

table! {
    users {
        verified_publisher -> Bool,
        gravatar_email -> Nullable<Text>,

        #[max_length = 240]
        description -> Nullable<VarChar>,
        avatar_hash -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,

        #[max_length = 64]
        username -> Nullable<VarChar>,
        password -> Text,
        email -> Text,
        admin -> Bool,

        #[max_length = 64]
        name -> VarChar,
        id -> BigInt,
    }
}

table! {
    user_connections {
        // supported oidc providers are set here -- used to query
        // information from the provider when provided via callback.
        github_account_id -> Nullable<Text>,
        google_account_id -> Nullable<Text>,
        apple_account_id -> Nullable<Text>,

        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> BigInt,
        id -> BigInt,
    }
}

table! {
    use crate::schema::types;
    use diesel::sql_types::*;

    repositories {
        #[max_length = 140]
        description -> Nullable<Varchar>,
        deprecated -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        icon_hash -> Nullable<Text>,
        private -> Bool,
        creator -> Nullable<BigInt>,
        owner -> BigInt,
        ty -> types::ChartType,

        #[max_length = 32]
        name -> Varchar,
        id -> Int8,
    }
}

table! {
    repository_members (id) {
        public_visibility -> Bool,

        #[max_length = 32]
        display_name -> Nullable<Varchar>,
        permissions -> Int8,
        repository -> Int8,
        updated_at -> Timestamptz,
        joined_at -> Timestamptz,
        account -> Int8,
        id -> Int8,
    }
}

diesel::table! {
    repository_releases (id) {
        repository -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        tag -> Text,
        id -> Int8,
    }
}

diesel::table! {
    organizations (id) {
        verified_publisher -> Bool,
        twitter_handle -> Nullable<Text>,
        gravatar_email -> Nullable<Text>,

        #[max_length = 32]
        display_name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        icon_hash -> Nullable<Text>,
        private -> Bool,
        owner -> Int8,

        #[max_length = 32]
        name -> Varchar,
        id -> Int8,
    }
}

table! {
    organization_members (id) {
        public_visibility -> Bool,

        #[max_length = 32]
        display_name -> Nullable<Varchar>,
        organization -> Int8,
        permissions -> Int8,
        updated_at -> Timestamptz,
        joined_at -> Timestamptz,
        account -> Int8,
        id -> Int8,
    }
}

diesel::table! {
    sessions (id) {
        refresh_token -> Text,
        access_token -> Text,
        expires_in -> Timestamptz,
        user -> Int8,
        id -> Uuid,
    }
}

diesel::table! {
    api_keys (id) {
        #[max_length = 140]
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        expires_in -> Nullable<Timestamptz>,
        scopes -> Int8,
        owner -> Int8,
        token -> Text,

        #[max_length = 32]
        name -> Varchar,
        id -> Int8,
    }
}

joinable!(user_connections -> users(user_id));
joinable!(api_keys -> users(owner));
joinable!(sessions -> users(user));

joinable!(repository_releases -> repositories(repository));
joinable!(repository_members -> repositories(repository));

joinable!(organization_members -> organizations(organization));

allow_tables_to_appear_in_same_query!(
    api_keys,
    organization_members,
    organizations,
    repositories,
    repository_members,
    repository_releases,
    sessions,
    user_connections,
    users,
);
