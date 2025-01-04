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

macro dummy_const($($tt:tt)*) {
    #[allow(unused_imports)]
    const _: () = {
        $($tt)*
    };
}

/// Our own version of <code>#\[derive([Selectable][diesel::expression::Selectable])\]</code> that
/// only works on what databases we support since Diesel doesn't do that as of v2.2.3
pub macro selectable {
    ($table:ident for $ty:ty => [$($field:ident: $field_ty:ty),*]) => {
        $crate::util::selectable! {
            @__impl sqlite($table) => $ty [$($field: $field_ty),*]
        }

        $crate::util::selectable! {
            @__impl postgresql($table) => $ty [$($field: $field_ty),*]
        }
    },

    (@__impl sqlite($table:ident) => $ty:ty [$($field:ident: $field_ty:ty),*]) => {
        $crate::util::dummy_const! {
            impl ::diesel::expression::Selectable<
                ::diesel::sqlite::Sqlite
            > for $ty {
                type SelectExpression = (
                    $(
                        ::charted_database::schema::sqlite::$table::$field,
                    )*
                );

                fn construct_selection() -> Self::SelectExpression {
                    (
                        $(
                            ::charted_database::schema::sqlite::$table::$field,
                        )*
                    )
                }
            }

            fn __type_check_compat()
            where
                $(
                    $field_ty: ::diesel::deserialize::FromSql<
                        ::diesel::dsl::SqlTypeOf<
                            ::charted_database::schema::sqlite::$table::$field,
                        >,
                        ::diesel::sqlite::Sqlite,
                    >,
                )*
            {}
        }
    },

    (@__impl postgresql($table:ident) => $ty:ty [$($field:ident: $field_ty:ty),*]) => {
        $crate::util::dummy_const! {
            impl ::diesel::expression::Selectable<
                ::diesel::pg::Pg
            > for $ty {
                type SelectExpression = (
                    $(
                        ::charted_database::schema::postgresql::$table::$field,
                    )*
                );

                fn construct_selection() -> Self::SelectExpression {
                    (
                        $(
                            ::charted_database::schema::postgresql::$table::$field,
                        )*
                    )
                }
            }

            fn __type_check_compat()
            where
                $(
                    $field_ty: ::diesel::deserialize::FromSql<
                        ::diesel::dsl::SqlTypeOf<
                            ::charted_database::schema::postgresql::$table::$field,
                        >,
                        ::diesel::pg::Pg,
                    >,
                )*
            {}
        }
    }
}
