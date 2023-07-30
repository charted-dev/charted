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

package database

// Listener is a pure interface that reacts to events from Postgres'
// LISTEN interface
type Listener interface {
	// Connected is a event that is fired that the indexer is now listening
	// to new events.
	Connected()

	// Disposed is a event that is fired that the indexer is now disposed,
	// and future events will be fired.
	Disposed()

	// Create is a generic listener interface which reacts when a new
	// entry in the database from the `table` was inserted.
	//
	// The payload is the extracted information that was inserted
	// from the database.
	Create(table string, payload any)

	// Delete is a generic listener interface which reacts when a
	// entry from the database in `table` was deleted.
	//
	// The payload is the extracted information that was inserted
	// from the database.
	Delete(table string, payload any)

	// Delete is a generic listener interface which reacts when a
	// entry from the database in `table` was updated.
	//
	// The payload is new updated payload that should be indexed.
	Update(table string, payload any)
}
