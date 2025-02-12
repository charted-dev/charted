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

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    api_keys (id) {
        description -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        expires_in -> Nullable<TimestamptzSqlite>,
        scopes -> BigInt,
        owner -> Text,
        token -> Text,
        name -> Text,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    organization_members (id) {
        public_visibility -> Bool,
        display_name -> Nullable<Text>,
        organization -> Text,
        permissions -> BigInt,
        updated_at -> TimestamptzSqlite,
        joined_at -> TimestamptzSqlite,
        account -> Text,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    organizations (id) {
        verified_publisher -> Bool,
        twitter_handle -> Nullable<Text>,
        gravatar_email -> Nullable<Text>,
        display_name -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        icon_hash -> Nullable<Text>,
        private -> Bool,
        owner -> Text,
        name -> Text,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    repositories (id) {
        description -> Nullable<Text>,
        deprecated -> Bool,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        icon_hash -> Nullable<Text>,
        private -> Bool,
        owner -> Text,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        id -> Text,
        creator -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    repository_members (id) {
        public_visibility -> Bool,
        display_name -> Nullable<Text>,
        permissions -> BigInt,
        repository -> Text,
        updated_at -> TimestamptzSqlite,
        joined_at -> TimestamptzSqlite,
        account -> Text,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    repository_releases (id) {
        repository -> Text,
        update_text -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        tag -> Text,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    sessions (id) {
        refresh_token -> Text,
        access_token -> Text,
        owner -> Text,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    user_connections (id) {
        noelware_account_id -> Nullable<BigInt>,
        google_account_id -> Nullable<Text>,
        github_account_id -> Nullable<Text>,
        apple_account_id -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        account -> Text,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    users (id) {
        verified_publisher -> Bool,
        gravatar_email -> Nullable<Text>,
        description -> Nullable<Text>,
        avatar_hash -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        username -> Text,
        password -> Nullable<Text>,
        email -> Text,
        admin -> Bool,
        name -> Nullable<Text>,
        id -> Text,
        prefers_gravatar -> Bool,
    }
}

diesel::joinable!(api_keys -> users (owner));
diesel::joinable!(organization_members -> organizations (organization));
diesel::joinable!(organization_members -> users (account));
diesel::joinable!(organizations -> users (owner));
diesel::joinable!(repository_members -> repositories (repository));
diesel::joinable!(repository_members -> users (account));
diesel::joinable!(repository_releases -> repositories (repository));
diesel::joinable!(sessions -> users (owner));
diesel::joinable!(user_connections -> users (account));

diesel::allow_tables_to_appear_in_same_query!(
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
