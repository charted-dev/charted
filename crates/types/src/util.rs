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

#[doc(hidden)]
pub macro dummy_const($($tt:tt)*) {
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

#[doc(hidden)]
#[macro_export]
macro_rules! mk_db_based_types {
    ($table:ident for $ty:ty => [$($field:ident: $fty:ty),*]) => {
        $crate::mk_db_based_types!(@__impl sqlite $table $ty [$($field -> $fty),*]);
        $crate::mk_db_based_types!(@__impl postgresql $table $ty [$($field -> $fty),*]);
    };

    (@__impl sqlite $table:ident $ty:ty [$($field:ident -> $fty:ty),*]) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct [<Sqlite $ty>]($ty);

            impl From<$ty> for [<Sqlite $ty>] {
                fn from(value: $ty) -> Self {
                    Self(value)
                }
            }

            impl $ty {
                #[doc = " Takes ownership of `self` and returns a [`" [<Sqlite $ty>] "`] object."]
                pub fn into_sqlite(self) -> [<Sqlite $ty>] {
                    [<Sqlite $ty>](self)
                }
            }

            $crate::util::dummy_const! {
                use ::diesel::ExpressionMethods;

                impl ::diesel::AsChangeset for [<Sqlite $ty>] {
                    type Target = ::charted_database::schema::sqlite::$table::table;
                    type Changeset = <(
                        $(
                            ::diesel::dsl::Eq<
                                ::charted_database::schema::sqlite::$table::$field,
                                $fty,
                            >,
                        )*
                    ) as ::diesel::AsChangeset>::Changeset;

                    fn as_changeset(self) -> Self::Changeset {
                        (
                            $(
                                ::charted_database::schema::sqlite::$table::$field.eq(self.0.$field),
                            )*
                        ).as_changeset()
                    }
                }
            }
        }
    };

    (@__impl postgresql $table:ident $ty:ty [$($field:ident -> $fieldty:ty),*]) => {
        paste::paste! {
            #[derive(Debug, Clone)]
            pub struct [<PG $ty>]($ty);

            impl From<$ty> for [<PG $ty>] {
                fn from(value: $ty) -> Self {
                    Self(value)
                }
            }

            impl $ty {
                #[doc = " Takes ownership of `self` and returns a [`" [<PG $ty>] "`] object."]
                pub fn into_pg(self) -> [<PG $ty>] {
                    [<PG $ty>](self)
                }
            }

            $crate::util::dummy_const! {
                use ::diesel::ExpressionMethods;

                impl ::diesel::AsChangeset for [<PG $ty>] {
                    type Target = ::charted_database::schema::postgresql::$table::table;
                    type Changeset = <(
                        $(
                            ::diesel::dsl::Eq<
                                ::charted_database::schema::postgresql::$table::$field,
                                $fieldty,
                            >,
                        )*
                    ) as ::diesel::AsChangeset>::Changeset;

                    fn as_changeset(self) -> Self::Changeset {
                        (
                            $(
                                ::charted_database::schema::postgresql::$table::$field.eq(self.0.$field),
                            )*
                        ).as_changeset()
                    }
                }
            }
        }
    };
}
