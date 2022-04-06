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
	"context"
	"errors"
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal"
	"noelware.org/charted/server/internal/controllers"
	"noelware.org/charted/server/internal/result"
	"noelware.org/charted/server/prisma/db"
	"noelware.org/charted/server/util"
)

func NewRepositoriesRouter() chi.Router {
	router := chi.NewRouter()

	router.Put("/", func(w http.ResponseWriter, r *http.Request) {
		statusCode, data, err := util.GetJsonBody(r)
		if err != nil {
			res := result.Err(statusCode, "INVALID_JSON_PAYLOAD", fmt.Sprintf("Unable to decode JSON payload: %s", err))
			res.Write(w)

			return
		}

		name, ok := data["name"].(string)
		if !ok {
			result.Err(
				406,
				"MISSING_IDENTIFIER",
				"You are missing a required body parameter: `name` -> String",
			).Write(w)

			return
		}

		ownerID, ok := data["owner"].(string)
		if !ok {
			result.Err(
				406,
				"MISSING_IDENTIFIER",
				"You are missing a required body parameter: `owner` -> String",
			).Write(w)

			return
		}

		res := controllers.CreateRepository(name, ownerID)
		res.Write(w)
	})

	router.Get("/{id}", func(w http.ResponseWriter, r *http.Request) {
		res := controllers.GetRepository(chi.URLParam(r, "id"))
		res.Write(w)
	})

	router.Get("/{id}/index.yaml", func(w http.ResponseWriter, r *http.Request) {
		id := chi.URLParam(r, "id")
		repository, err := internal.GlobalContainer.Database.Repositories.FindUnique(db.Repositories.ID.Equals(id)).Exec(context.TODO())

		if err != nil {
			if errors.Is(err, db.ErrNotFound) {
				result.Err(404, "UNKNOWN_REPOSITORY", fmt.Sprintf("Unknown repository %s.", id)).Write(w)
				return
			}

			logrus.Errorf("Unable to query entry 'repositories.%s': %s", id, err)
			result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to retrieve repository %s.", id)).Write(w)

			return
		}

		indexYaml, err := internal.GlobalContainer.Storage.GetIndexYaml(repository.OwnerID, id)
		if err != nil {
			result.Err(500, "INTERNAL_SERVER_ERROR", "Unable to retrieve index.yaml").Write(w)
			return
		}

		w.Header().Add("Content-Type", "application/yaml")
		w.WriteHeader(200)
		_, _ = w.Write([]byte(indexYaml))
	})

	router.Post("/{id}/index.yaml", func(w http.ResponseWriter, r *http.Request) {
		// TODO: this shit LMAO
		res := result.Ok(nil)
		res.Write(w)
	})

	return router
}
