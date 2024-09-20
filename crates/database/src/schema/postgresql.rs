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

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    api_keys (id) {
        #[max_length = 140]
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        expires_in -> Nullable<Timestamptz>,
        scopes -> Int8,
        owner -> Text,
        token -> Text,
        #[max_length = 32]
        name -> Varchar,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    organization_members (id) {
        public_visibility -> Bool,
        #[max_length = 32]
        display_name -> Nullable<Varchar>,
        organization -> Text,
        permissions -> Int8,
        updated_at -> Timestamptz,
        joined_at -> Timestamptz,
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
        #[max_length = 32]
        display_name -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        icon_hash -> Nullable<Text>,
        private -> Bool,
        owner -> Text,
        #[max_length = 32]
        name -> Varchar,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    repositories (id) {
        #[max_length = 64]
        description -> Nullable<Varchar>,
        deprecated -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        icon_hash -> Nullable<Text>,
        private -> Bool,
        creator -> Nullable<Text>,
        owner -> Text,
        #[max_length = 32]
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> ChartType,
        id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::schema::sql_types::*;

    repository_members (id) {
        public_visibility -> Bool,
        #[max_length = 32]
        display_name -> Nullable<Varchar>,
        permissions -> Int8,
        repository -> Text,
        updated_at -> Timestamptz,
        joined_at -> Timestamptz,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        noelware_account_id -> Nullable<Int8>,
        google_account_id -> Nullable<Text>,
        github_account_id -> Nullable<Text>,
        apple_account_id -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        #[max_length = 240]
        description -> Nullable<Varchar>,
        avatar_hash -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 64]
        username -> Varchar,
        password -> Nullable<Text>,
        email -> Text,
        admin -> Bool,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        id -> Text,
    }
}

diesel::joinable!(api_keys -> users(owner));
diesel::joinable!(organization_members -> organizations(organization));
diesel::joinable!(organization_members -> users(account));
diesel::joinable!(organizations -> users(owner));
diesel::joinable!(repository_members -> repositories(repository));
diesel::joinable!(repository_members -> users(account));
diesel::joinable!(repository_releases -> repositories(repository));
diesel::joinable!(sessions -> users(owner));
diesel::joinable!(user_connections -> users(account));

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
