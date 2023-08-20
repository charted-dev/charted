// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use serde::Serialize;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Deref,
    sync::{
        atomic::{AtomicU16, AtomicU64, Ordering},
        Arc,
    },
    time::{SystemTime, UNIX_EPOCH},
};
use utoipa::{
    openapi::{KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, SchemaType},
    ToSchema,
};

use crate::SNOWFLAKE_EPOCH;

const SEQUENCE_BITS: usize = 12;
const NODE_BITS: usize = 10;
const MAX_SEQUENCE_BITS: usize = (1 << SEQUENCE_BITS) - 1;

#[derive(Debug, Clone)]
pub struct Snowflake {
    exhausted_at_time: Arc<AtomicU64>,
    last_timestamp: Arc<AtomicU64>,
    sequence: Arc<AtomicU16>,
    node_id: u16,
}

unsafe impl Send for Snowflake {}
unsafe impl Sync for Snowflake {}

impl Snowflake {
    #[inline(never)]
    fn current_timestamp() -> u64 {
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock is configured wrong?")
            .as_millis() as u64)
            - SNOWFLAKE_EPOCH as u64
    }

    pub fn new(node_id: u16) -> Snowflake {
        Snowflake {
            exhausted_at_time: Arc::new(AtomicU64::new(0)),
            last_timestamp: Arc::new(AtomicU64::new(0)),
            sequence: Arc::new(AtomicU16::new(0)),
            node_id,
        }
    }

    #[inline]
    pub fn generate(&self) -> ID {
        let now = Snowflake::current_timestamp();
        let seq = self.sequence.load(Ordering::Relaxed);
        let exhaused = self.exhausted_at_time.load(Ordering::Relaxed);

        if seq == 4095 && now == exhaused {
            while Snowflake::current_timestamp() - now < 1 {
                continue;
            }
        }

        self.sequence.store(
            match seq {
                4095 => 0,
                _ => seq + 1,
            },
            Ordering::Relaxed,
        );

        if self.sequence.load(Ordering::Relaxed) >= 4095 {
            self.last_timestamp.store(now, Ordering::Relaxed);
        }

        ID((now << (NODE_BITS + SEQUENCE_BITS)) | ((self.node_id as u64) << (SEQUENCE_BITS as u64)) | seq as u64)
    }
}

/// Represents a snowflake ID.
#[derive(Clone, Copy)]
pub struct ID(u64);

impl Debug for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Snowflake")
            .field("value", &self.0)
            .field("node_id", &self.node_id())
            .field("seq", &self.sequence())
            .finish()
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl From<ID> for u64 {
    fn from(val: ID) -> Self {
        val.0
    }
}

impl From<u64> for ID {
    fn from(value: u64) -> Self {
        ID(value)
    }
}

impl Deref for ID {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ID {
    const MASK_NODE_ID: usize = ((1 << NODE_BITS) - 1) << SEQUENCE_BITS;

    /// Returns the ID itself.
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Returns the node ID that this ID was generated from.
    pub fn node_id(&self) -> usize {
        (self.0 as usize & ID::MASK_NODE_ID) >> SEQUENCE_BITS
    }

    /// Returns the sequence number.
    pub fn sequence(&self) -> usize {
        self.0 as usize & MAX_SEQUENCE_BITS
    }

    /// Timestamp (in milliseconds) of when this snowflake
    /// was created.
    pub fn timestamp(&self) -> usize {
        (self.0 as usize >> (NODE_BITS + SEQUENCE_BITS)) + SNOWFLAKE_EPOCH
    }
}

impl Serialize for ID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'s> ToSchema<'s> for ID {
    fn schema() -> (&'s str, RefOr<Schema>) {
        (
            "Snowflake",
            RefOr::T(Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::Integer)
                    .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                    .description(Some("Unique identifier for a resource. Based off the [Twitter Snowflake](https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake) spec."))
                    .min_length(Some(15))
                    .build(),
            )),
        )
    }
}
