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
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
	"noelware.org/charted/server/internal/controllers"
	"noelware.org/charted/server/internal/result"
	"noelware.org/charted/server/util"
)

func NewUsersRouter() chi.Router {
	router := chi.NewRouter()
	controller := controllers.UserController{}

	router.Get("/{id}", func(w http.ResponseWriter, r *http.Request) {
		id := chi.URLParam(r, "id")
		res := controller.Get(id)

		res.Write(w)
	})

	router.Put("/", func(w http.ResponseWriter, r *http.Request) {
		statusCode, data, err := util.GetJsonBody(r)
		if err != nil {
			res := result.Err(statusCode, "INVALID_JSON_PAYLOAD", fmt.Sprintf("Unable to decode JSON payload: %s", err))
			res.Write(w)

			return
		}

		username, ok := data["username"].(string)
		if !ok {
			result.Err(
				406,
				"MISSING_IDENTIFIER",
				"You are missing a required body parameter: `username` -> String",
			).Write(w)

			return
		}

		email, ok := data["email"].(string)
		if !ok {
			result.Err(
				406,
				"MISSING_IDENTIFIER",
				"You are missing a required body parameter: `email` -> String",
			).Write(w)

			return
		}

		password, ok := data["password"].(string)
		if !ok {
			result.Err(
				406,
				"MISSING_IDENTIFIER",
				"You are missing a required body parameter: `password` -> String",
			).Write(w)

			return
		}

		res := controller.Create(username, password, email)
		res.Write(w)
	})

	return router
}
