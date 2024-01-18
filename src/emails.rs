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

/// Generated protobuf definitions.
pub mod protos {
    // pub mod google {
    //     pub mod protobuf {
    //         tonic::include_proto!("google.protobuf");
    //     }
    // }

    pub mod noelware {
        pub mod charted {
            pub mod emails {
                tonic::include_proto!("noelware.charted.emails");
            }
        }
    }
}

use protos::noelware::charted::emails::emails_client::EmailsClient;
use tonic::transport::Channel;

/// Represents a [`EmailsClient`] that uses the [`tonic::transport::Channel`] as the generic.
pub type Client = EmailsClient<Channel>;
