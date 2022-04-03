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

package meilisearch

import (
	"time"

	"github.com/meilisearch/meilisearch-go"
	"github.com/sirupsen/logrus"
	"github.com/valyala/fasthttp"
	"noelware.org/charted/server/internal/search"
)

// Service represents the service for Meilisearch which implements search.Engine.
type Service struct {
	config *Config
	client *meilisearch.Client
}

// Config represents the configuration details for the 'search.meili' config table.
type Config struct {
	// MasterKey is the master key if registered on the server.
	MasterKey *string `toml:"master_key"`

	// Endpoint is the endpoint to Meilisearch.
	Endpoint string `toml:"endpoint"`
}

func NewService(config *Config) search.Engine {
	cfg := meilisearch.ClientConfig{
		Host: config.Endpoint,
	}

	if config.MasterKey != nil {
		cfg.APIKey = *config.MasterKey
	}

	http := &fasthttp.Client{
		Name: "charted/meilisearch",
	}

	client := meilisearch.NewFastHTTPCustomClient(cfg, http)
	service := Service{config, client}
	service.createIndexes()

	res, err := client.Version()
	if err != nil {
		logrus.Fatalf("Unable to retrieve server version from Meilisearch because: %s", err)
	}

	logrus.Debugf("Using v%s (commit=%s, build-date=%s) of Meilisearch!",
		res.PkgVersion,
		res.CommitSha,
		res.CommitDate)

	return service
}

func (s Service) createIndexes() {
	logrus.Debug("Now creating indexes...")
}

func (s Service) Type() search.EngineType {
	return search.Meili
}

func (s Service) Search(index string, query string, options ...search.OptionFunc) (*search.Result, error) {
	logrus.Debugf("Now searching query %s on index %s...", query, index)

	optionsObj := &search.Options{}
	for _, override := range options {
		override(optionsObj)
	}

	idx := s.client.Index(index)
	searchOptions := &meilisearch.SearchRequest{}

	if optionsObj.Offset != 0 {
		searchOptions.Offset = optionsObj.Offset
	}

	if optionsObj.Limit != 0 {
		searchOptions.Limit = optionsObj.Limit
	}

	t := time.Now()
	res, err := idx.Search(query, searchOptions)

	if err != nil {
		return nil, err
	}

	return &search.Result{
		RequestTimeProcessing: time.Since(t).Milliseconds(),
		ServiceTimeProcessing: res.ProcessingTimeMs,
		TotalHits:             res.NbHits,
		MaxScore:              nil,
		Data:                  res.Hits,
	}, nil
}
