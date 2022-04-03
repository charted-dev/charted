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

package elastic

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"strings"
	"time"

	"github.com/elastic/go-elasticsearch/v8"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal/search"
)

// Config represents the Elasticsearch cluster configuration
// to connect to.
type Config struct {
	// The password to use if Basic authentication is enabled on the server.
	Password *string `toml:"password,omitempty"`

	// The username to use if Basic authentication is enabled on the server.
	Username *string `toml:"username,omitempty"`

	// The list of nodes to use when connecting to Elasticsearch.
	Nodes []string `toml:"nodes"`

	// CACertPath is the path to a .pem file to use TLS connections within
	// Elasticsearch.
	CACertPath *string `toml:"ca_path,omitempty"`

	// SkipSSLVerify skips the SSL certificates.
	SkipSSLVerify bool `toml:"skip_ssl_verify"`
}

type Service struct {
	config *Config
	client *elasticsearch.Client
}

var indexes = map[string]string{
	"charted_users": `{
	"settings": {
		"number_of_shards": 1,
		"number_of_replicas": 1
	},
	"mappings": {
		"properties": {
			"id": { "type": "keyword" },
			"name": { "type": "keyword" },
			"description": { "type": "keyword" },
			"created_at": { "type": "date" },
			"updated_at": { "type": "date" }
		}
	}
}`,
	"charted_repositories": `{
	"settings": {
		"number_of_shards": 1,
		"number_of_replicas": 1
	},
	"mappings": {
		"properties": {
			"id": { "type": "keyword" },
			"name": { "type": "keyword" },
			"readme": { "type": "keyword" },
			"description": { "type": "keyword" },
			"downloads_count": { "type": "integer" },
			"stargazers_count": { "type": "integer" },
			"created_at": { "type": "date" },
			"updated_at": { "type": "date" }
		}
	}
}`,
}

func NewService(config *Config) search.Engine {
	logrus.Info("Connecting to Elastic...")

	transport := http.DefaultTransport.(*http.Transport).Clone()
	transport.MaxIdleConnsPerHost = 50

	cfg := elasticsearch.Config{
		Addresses:            config.Nodes,
		DiscoverNodesOnStart: true,
	}

	if config.Username != nil {
		cfg.Username = *config.Username
	}

	if config.Password != nil {
		cfg.Password = *config.Password
	}

	if config.SkipSSLVerify {
		transport.TLSClientConfig = &tls.Config{
			InsecureSkipVerify: true, //nolint
		}
	}

	if config.CACertPath != nil {
		logrus.Infof("Specified TLS certificate for Elastic in path '%s'!", *config.CACertPath)

		var err error
		if transport.TLSClientConfig.RootCAs, err = x509.SystemCertPool(); err != nil {
			logrus.Fatalf("Unable to create certificate pool because: %s", err)
		}

		cacert, err := ioutil.ReadFile(*config.CACertPath)
		if err != nil {
			logrus.Fatalf("Unable to read client cert in path %s because: %s", *config.CACertPath, err)
		}

		transport.TLSClientConfig.RootCAs.AppendCertsFromPEM(cacert)
		transport.TLSClientConfig.ClientAuth = tls.RequireAnyClientCert
		transport.TLSClientConfig.InsecureSkipVerify = true
	}

	cfg.Transport = transport
	client, err := elasticsearch.NewClient(cfg)
	if err != nil {
		logrus.Fatalf("Unable to create Elastic client because: %s", err)
	}

	// Check if we can query from the server
	res, err := client.Info()
	if err != nil {
		logrus.Fatalf("Unable to query cluster information because: %s", err)
	}

	defer res.Body.Close()

	var data map[string]any
	if err = json.NewDecoder(res.Body).Decode(&data); err != nil {
		logrus.Fatalf("Unable to decode body from Elastic: %s", err)
	}

	version := data["version"].(map[string]any)["number"].(string)
	logrus.Infof("Server: v%s | Client: v%s", version, elasticsearch.Version)

	service := &Service{config, client}
	service.createIndexes()

	return service
}

func (s *Service) createIndexes() {
	logrus.Info("Now creating indexes if not found...")

	for key, index := range indexes {
		logrus.Infof("Checking if index %s exists...", key)

		res, err := s.client.Indices.Exists([]string{key}, s.client.Indices.Exists.WithErrorTrace())
		if err != nil {
			logrus.Errorf("Received error (%s) while checking if index %s exists, skipping!", err, key)
		}

		if res.StatusCode == 404 {
			logrus.Infof("Index %s does not exist, now creating!", key)
			_, err := s.client.Indices.Create(key,
				s.client.Indices.Create.WithErrorTrace(),
				s.client.Indices.Create.WithBody(strings.NewReader(index)))

			if err != nil {
				logrus.Errorf("Unable to create index %s because: %s", key, err)
				continue
			}

			logrus.Infof("Created index %s!", key)
		} else {
			logrus.Infof("Index %s should already exist?", key)
		}
	}
}

func (*Service) Type() search.EngineType {
	return search.Elasticsearch
}

func (s *Service) Search(index string, query string, options ...search.OptionFunc) (*search.Result, error) {
	logrus.Debugf("Now searching '%s' on index %s...", query, index)

	opts := &search.Options{}
	for _, override := range options {
		override(opts)
	}

	matchType := "match_all"
	if opts.Fuzzy {
		matchType = "fuzzy"
	}

	var buf bytes.Buffer
	q := map[string]any{
		"query": map[string]any{
			matchType: map[string]any{
				opts.WithClause: query,
			},
		},
	}

	// this should never happen but whatever
	if err := json.NewEncoder(&buf).Encode(q); err != nil {
		logrus.Errorf("Unable to encode query object: %s", err)
		return nil, errors.New("unable to encode query object to json")
	}

	t := time.Now()
	res, err := s.client.Search(
		s.client.Search.WithIndex(index),
		s.client.Search.WithContext(context.TODO()),
		s.client.Search.WithBody(&buf),
		s.client.Search.WithTrackTotalHits(true),
		s.client.Search.WithErrorTrace(),
		s.client.Search.WithPretty())

	if err != nil {
		logrus.Errorf("Unable to search from Elastic in index %s: %s", index, err)
		return nil, errors.New("internal server error during elastic request :<")
	}

	defer res.Body.Close()

	if res.IsError() {
		var data map[string]any
		if err := json.NewDecoder(res.Body).Decode(&data); err != nil {
			logrus.Errorf("Unable to decode JSON payload from Elastic when received an non-acceptable status code (index '%s'): %s", index, err)
			return nil, errors.New("unable to decode payload from elastic when received an non-acceptable status code")
		}

		e := data["error"].(map[string]any)
		logrus.Errorf("Unable to search '%s' on index %s (%s): %s",
			query,
			index,
			e["type"],
			e["reason"])

		return nil, fmt.Errorf("unable to search on index '%s' :<", index)
	}

	var d map[string]any
	if err := json.NewDecoder(res.Body).Decode(&d); err != nil {
		logrus.Errorf("Unable to decode JSON payload from Elastic (index '%s'): %s", index, err)
		return nil, errors.New("unable to decode payload from elasticsearch")
	}

	since := time.Since(t).Milliseconds()
	took := d["took"].(float64)
	hits := d["hits"].(map[string]any)
	maxScore, ok := hits["max_score"].(float64)
	if !ok {
		maxScore = float64(0)
	}

	totalHits := hits["total"].(map[string]any)["value"].(float64)
	rawData, ok := hits["hits"].([]map[string]any)
	if !ok {
		rawData = make([]map[string]any, 0)
	}

	actual := make([]any, 0)
	if rawData != nil {
		for _, hit := range rawData {
			source := hit["_source"]
			actual = append(actual, source)
		}
	}

	return &search.Result{
		RequestTimeProcessing: since,
		ServiceTimeProcessing: int64(took),
		TotalHits:             int64(totalHits),
		MaxScore:              &maxScore,
		Data:                  actual,
	}, nil
}
