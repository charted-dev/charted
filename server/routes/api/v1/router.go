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

package v1

import (
	"net/http"

	"github.com/go-chi/chi/v5"
	"noelware.org/charted/server/internal/result"
)

func NewApiV1Router() chi.Router { //nolint
	router := chi.NewRouter()

	router.Mount("/repositories", NewRepositoriesRouter())
	router.Mount("/users", NewUsersRouter())

	router.Get("/", func(w http.ResponseWriter, r *http.Request) {
		res := result.Ok(map[string]any{
			"message":  "hello world",
			"docs_url": "https://charts.noelware.org/docs/api/current",
		})

		res.Write(w)
	})

	return router
}
