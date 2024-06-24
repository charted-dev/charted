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

//! The `testing::containers::redis` module contains definitions for Redis. Since Noelware
//! uses [`bitnami/redis`] for running Redis servers for our official registry, we also test
//! both [`redis`] and [`bitnami/redis`] images.
//!
//! [`redis`]: https://hub.docker.com/_/redis
//! [`bitnami/redis`]: https://hub.docker.com/r/bitnami/redis
//!
//! ## Example (with TestKit)
//! ```no_run
//! # use charted_core::testing::containers::redis;
//! # use charted_testkit::test;
//! # use testcontainers::ContainerAsync;
//! #
//! #[test(
//!     containers(bitnami::container)
//! )]
//! async fn my_test(ctx: TestContext) {
//!     let container = ctx.container::<ContainerAsync<redis::bitnami::Image>>();
//! }
//! ```

/// The `testing::containers::redis::bitnami` module uses [`bitnami/redis`] as the specified
/// Redis image.
///
/// [`bitnami/redis`]: https://hub.docker.com/r/bitnami/redis
pub mod bitnami {
    use std::collections::HashMap;
    use testcontainers::{core::WaitFor, runners::AsyncRunner, ContainerAsync};

    /// Default tag for the [`bitnami/redis`] image.
    ///
    /// [`bitnami/redis`]: https://hub.docker.com/r/bitnami/redis
    // renovate: datasource=docker image=bitnami/redis
    pub const TAG: &str = "7.2.5-debian-12-r1";

    /// Represents a [`Image`] implementation for [`bitnami/redis`].
    ///
    /// [`bitnami/redis`]: https://hub.docker.com/r/bitnami/redis
    #[derive(Debug)]
    pub struct Image {
        vars: HashMap<String, String>,
    }

    impl Default for Image {
        fn default() -> Self {
            Image {
                vars: azalia::hashmap! {
                    "ALLOW_EMPTY_PASSWORD" => "yes"
                },
            }
        }
    }

    impl Image {
        /// Extend the environment variables for this [`Image`].
        pub fn with_env<I: IntoIterator<Item = (String, String)>>(mut self, iter: I) -> Self {
            self.vars.extend(iter);
            self
        }

        /// Sets a password via `REDIS_PASSWORD`.
        pub fn with_password<I: Into<String>>(mut self, pass: I) -> Self {
            // Since we want to provide a password, we'll remove the default
            // `ALLOW_EMPTY_PASSWORD` environment variable.
            let _ = self.vars.remove("ALLOW_EMPTY_PASSWORD");

            self.vars.insert("REDIS_PASSWORD".into(), pass.into());
            self
        }
    }

    impl testcontainers::Image for Image {
        type Args = Vec<String>;

        fn name(&self) -> String {
            String::from("bitnami/redis")
        }

        fn tag(&self) -> String {
            TAG.to_owned()
        }

        fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
            Box::new(self.vars.iter())
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::message_on_stdout("Ready to accept connections")]
        }

        fn expose_ports(&self) -> Vec<u16> {
            vec![6379]
        }
    }

    pub async fn spawn() -> ContainerAsync<Image> {
        let image = Image::default();
        image.start().await.expect("failed to run Redis(TM) container")
    }

    // pub async fn spawn<F: FnMut(&mut Image)>(mut build: F) -> ContainerAsync<Image> {
    //     let mut image = Image::default();
    //     build(&mut image);

    //     image.start().await.expect("failed to run Redis(TM) container")
    // }
}

/// The `testing::containers::redis::official` module uses [`redis`] as the specified
/// Redis image.
///
/// [`redis`]: https://hub.docker.com/_/redis
pub mod official {
    use std::collections::HashMap;
    use testcontainers::{core::WaitFor, runners::AsyncRunner, ContainerAsync};

    /// Default tag for the [`redis`] image.
    ///
    /// [`redis`]: https://hub.docker.com/_/redis
    // renovate: datasource=docker image=redis
    pub const TAG: &str = "7.2.5";

    /// Represents a [`Image`] implementation for [`redis`].
    ///
    /// [`redis`]: https://hub.docker.com/_/redis
    #[derive(Debug)]
    pub struct Image {
        vars: HashMap<String, String>,
    }

    impl Default for Image {
        fn default() -> Self {
            Image {
                vars: azalia::hashmap!(),
            }
        }
    }

    impl Image {
        /// Extend the environment variables for this [`Image`].
        pub fn with_env<I: IntoIterator<Item = (String, String)>>(mut self, iter: I) -> Self {
            self.vars.extend(iter);
            self
        }

        /// Sets a password via `REDIS_PASSWORD`.
        pub fn with_password<I: Into<String>>(mut self, pass: I) -> Self {
            self.vars.insert("REDIS_PASSWORD".into(), pass.into());
            self
        }
    }

    impl testcontainers::Image for Image {
        type Args = Vec<String>;

        fn name(&self) -> String {
            String::from("redis")
        }

        fn tag(&self) -> String {
            TAG.to_owned()
        }

        fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
            Box::new(self.vars.iter())
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::message_on_stdout("Ready to accept connections")]
        }

        fn expose_ports(&self) -> Vec<u16> {
            vec![6379]
        }
    }

    pub async fn spawn() -> ContainerAsync<Image> {
        let image = Image::default();
        image.start().await.expect("failed to run Redis(TM) container")
    }

    // pub async fn spawn<F: FnMut(&mut Image)>(mut build: F) -> ContainerAsync<Image> {
    //     let mut image = Image::default();
    //     build(&mut image);

    //     image.start().await.expect("failed to run Redis(TM) container")
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::{bitnami, official};
//     use axum::Router;
//     use charted_testkit::TestContext;

//     fn router() -> Router {
//         Router::new()
//     }

//     #[charted_testkit::test(containers = [bitnami::spawn()])]
//     async fn test_bitnami(ctx: &TestContext) {}
// }
