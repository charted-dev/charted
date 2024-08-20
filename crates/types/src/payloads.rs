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

//! The `payloads` module contains submodules that provide the types
//! for modifying entities.

pub mod apikey;
pub mod member;
pub mod organization;
pub mod repository;
pub mod user;

macro_rules! create_modifying_payload {
    (
        $name:ident;

        $(#[$create:meta])*
        create {
            $(
                $(#[$create_field_meta:meta])*
                $create_vis:vis $create_field:ident: $create_ty:ty,
            )*
        }

        $(#[$patch:meta])*
        patch {
            $(
                $(#[$patch_field_meta:meta])*
                $patch_vis:vis $patch_field:ident: $patch_ty:ty,
            )*
        }
    ) => {
        paste::paste! {
            $(#[$create])*
            pub struct [<Create $name Payload>] {
                $(
                    $(#[$create_field_meta])*
                    $create_vis $create_field: $create_ty,
                )*
            }

            $(#[$patch])*
            pub struct [<Patch $name Payload>] {
                $(
                    $(#[$patch_field_meta])*
                    $patch_vis $patch_field: $patch_ty,
                )*
            }
        }
    };
}

use create_modifying_payload;
