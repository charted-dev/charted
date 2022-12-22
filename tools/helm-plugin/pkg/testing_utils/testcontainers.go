// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

package testingutils

import (
	"context"

	testcontainers "github.com/testcontainers/testcontainers-go"
)

type TestcontainerOption func(req *testcontainers.ContainerRequest)

// WithExposedPort returns a TestcontainerOption to add an option to any testcontainers-related function.
func WithExposedPort(port string) TestcontainerOption {
	return func(req *testcontainers.ContainerRequest) {
		req.ExposedPorts = append(req.ExposedPorts, port)
	}
}

func WithEnvMappings(mappings map[string]string) TestcontainerOption {
	return func(req *testcontainers.ContainerRequest) {
		for k, v := range mappings {
			req.Env[k] = v
		}
	}
}

// RunRedisContainer will run a Redis container backed by testcontainers.
func RunRedisContainer(options ...TestcontainerOption) (testcontainers.Container, error) {
	ctx := context.Background()
	req := testcontainers.ContainerRequest{
		Image: "bitnami/redis:7.0.7",
	}

	for _, opt := range options {
		opt(&req)
	}

	container, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})

	if err != nil {
		return nil, err
	}

	return container, nil
}

// RunPostgresContainer will run a Postgres database as a Docker container backed by testcontainers.
func RunPostgresContainer(options ...TestcontainerOption) (testcontainers.Container, error) {
	ctx := context.Background()
	req := testcontainers.ContainerRequest{
		Image: "bitnami/postgresql:15.1.0",
		Env:   map[string]string{},
		Cmd:   []string{"postgres", "-c", "fsync=off"},
	}

	for _, opt := range options {
		opt(&req)
	}

	req.Env["POSTGRESQL_USERNAME"] = "charted"
	req.Env["POSTGRESQL_PASSWORD"] = "charted"
	req.Env["POSTGRESQL_DATABASE"] = "charted"

	container, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})

	if err != nil {
		return nil, err
	}

	return container, err
}
