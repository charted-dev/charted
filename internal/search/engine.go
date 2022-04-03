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

// Result represents the result of Engine.Search.
type Result struct {
	// RequestTimeProcessing returns an int64 of how long it took
	// to process the request.
	RequestTimeProcessing int64 `json:"request_time_ms"`

	// ServiceTimeProcessing returns an int64 of how long the service
	// took to process the request, this is from the service request
	// body itself, can return -1 if it doesn't support it.
	ServiceTimeProcessing int64 `json:"service_time_ms"`

	// MaxScore returns the max score of the search itself, this can
	// be nil if it doesn't support it. This is only in the Elasticsearch
	// and Tsubasa engines only.
	MaxScore *float64 `json:"max_score,omitempty"`

	// TotalHits returns how many hits in total from the request body itself.
	TotalHits int64 `json:"total_hits,omitempty"`

	// Data represents the raw data that came from the service.
	Data []any `json:"data"`
}

// Engine is the blueprint to use to implement the search engines.
type Engine interface {
	// Search searches through the engine and returns a tuple of []any and error.
	Search(index string, query string, options ...OptionFunc) (*Result, error)

	// Type returns the EngineType of this Engine.
	Type() EngineType
}
