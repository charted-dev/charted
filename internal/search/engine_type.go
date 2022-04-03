// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Go.
// Copyright 2022 Noelware <team@noelware.org>
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

package search

import "errors"

// EngineType represents the engine type that the Engine is representing.
type EngineType string

var (
	// Elasticsearch is the Elastic engine, to use a purified Elastic cluster
	// to perform search.
	Elasticsearch EngineType = "elastic"

	// Unknown is a mystery. Good luck.
	Unknown EngineType = "?"

	// Tsubasa is the engine to use Noel's Tsubasa microservice that abstracts
	// using Elasticsearch for search.
	Tsubasa EngineType = "tsubasa"

	// Meili is the Meilisearch engine, to use a Meilisearch instance
	// rather than Elasticsearch.
	Meili EngineType = "meili"
)

// AllEngines represents all the engine types available.
var AllEngines = []EngineType{Elasticsearch, Tsubasa, Meili}

// DetermineEngineType determines the EngineType of a string.
func DetermineEngineType(e string) EngineType {
	for _, engine := range AllEngines {
		if engine.String() == e {
			return engine
		}
	}

	return Unknown
}

// String stringifies the EngineType.
func (e EngineType) String() string {
	switch e {
	case Elasticsearch:
		return "elastic"

	case Tsubasa:
		return "tsubasa"

	case Meili:
		return "meili"

	case Unknown:
		return "?"

	default:
		return "?"
	}
}

func (e EngineType) MarshalJSON() ([]byte, error) {
	item := e.String()
	if item == "?" {
		return nil, errors.New("engine type was not correctly defined :(")
	}

	return []byte(item), nil
}

func (e EngineType) UnmarshalJSON(data []byte) error {
	item := string(data)
	engineType := DetermineEngineType(item)

	if engineType == Unknown {
		return errors.New("unable to unmarshal Unknown engine type :(")
	}

	return nil
}
