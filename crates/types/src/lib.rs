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

//! # 🐻‍❄️📦 `charted_types`
//! This crate is just a generic crate that exports all newtype wrappers for the
//! API server and database entities.

#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]

pub mod name;
pub mod payloads;

mod entities;
mod newtypes;

pub use entities::*;
pub use newtypes::*;

// Not public API, used by macros in this crate.
#[doc(hidden)]
pub mod __private {
    pub use paste::paste;
}

#[allow(unused_imports)]
mod helm {
    #[cfg(feature = "__internal_db")]
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::{fmt::Write, str::FromStr};

    /// Representation of a Helm chart.
    #[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, derive_more::Display)]
    #[serde(rename_all = "lowercase")]
    #[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
    #[cfg_attr(feature = "__internal_db", derive(EnumIter, DeriveActiveEnum))]
    #[cfg_attr(
        feature = "__internal_db",
        sea_orm(
            rs_type = "String",
            db_type = "String(StringLen::None)",
            rename_all = "lowercase",
            enum_name = "chart_type"
        )
    )]
    pub enum ChartType {
        /// The default chart type and represents a standard Helm chart.
        ///
        /// **Note**: Application charts can also act like library charts! Set `type` to
        /// `"library"`.
        #[default]
        #[display("application")]
        Application,

        /// The chart provide utilities or functions for building application Helm charts.
        ///
        /// Library charts cannot expose Helm templates since it cannot create Kubernetes
        /// objects from `helm install`
        #[display("library")]
        Library,
    }

    impl FromStr for ChartType {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match &*s.to_ascii_lowercase() {
                "application" => Ok(ChartType::Application),
                "library" => Ok(ChartType::Library),

                _ => Err(()),
            }
        }
    }

    #[cfg(feature = "__internal_db")]
    impl Iden for ChartType {
        fn unquoted(&self, s: &mut dyn Write) {
            s.write_str(match self {
                ChartType::Application => "application",
                ChartType::Library => "library",
            })
            .unwrap();
        }
    }
}

pub use helm::*;

#[cfg(feature = "__internal_db")]
#[macro_export]
#[doc(hidden)]
macro_rules! cfg_sea_orm {
    ($($item:item)*) => {
        $($item)*
    };
}

#[cfg(not(feature = "__internal_db"))]
#[macro_export]
#[doc(hidden)]
macro_rules! cfg_sea_orm {
    ($(tt:tt)*) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_openapi {
    ($($item:item)*) => {
        #[cfg(feature = "openapi")]
        #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "openapi")))]
        const _: () = {
            $($item)*
        };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfg_jsonschema {
    ($($item:item)*) => {
        #[cfg(feature = "jsonschema")]
        #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "jsonschema")))]
        const _: () = {
            $($item)*
        };
    };
}
