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

package routes

import (
	"github.com/go-chi/chi/v5"
	"net/http"
	"noelware.org/charted/server/internal"
	"noelware.org/charted/server/internal/result"
	"time"
)

func NewVersionRouter() chi.Router {
	router := chi.NewRouter()
	router.Get("/", func(w http.ResponseWriter, req *http.Request) {
		flavour := "git"
		buildDate, _ := time.Parse(time.RFC3339, internal.BuildDate)

		if internal.Docker() {
			flavour = "docker"
		}

		res := result.Ok(map[string]any{
			"version":    internal.Version,
			"commit_sha": internal.CommitSHA,
			"build": map[string]any{
				"date":    buildDate.Format(time.RFC1123),
				"flavour": flavour,
			},
		})

		res.Write(w)
	})

	return router
}
