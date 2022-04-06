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

package controllers

import (
	"context"
	"errors"
	"fmt"

	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal"
	dbtypes "noelware.org/charted/server/internal/db_types"
	"noelware.org/charted/server/internal/result"
	"noelware.org/charted/server/prisma/db"
)

var repositoryNameRegex = ``

func GetRepository(id string) *result.Result {
	repository, err := internal.GlobalContainer.Database.Repositories.FindUnique(db.Repositories.ID.Equals(id)).Exec(context.TODO())
	if err != nil {
		if errors.Is(err, db.ErrNotFound) {
			return result.Ok(nil)
		}

		logrus.Errorf("Unable to query entry 'repositories.%s': %s", id, err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to retrieve repository %s.", id))
	}

	return result.Ok(dbtypes.FromRepositoryDbModel(internal.GlobalContainer.Database, repository))
}

func GetRepositories(id string) *result.Result {
	repositories, err := internal.GlobalContainer.Database.Repositories.FindMany(db.Repositories.OwnerID.Equals(id)).Exec(context.TODO())
	if err != nil {
		logrus.Errorf("Unable to retrieve repositories for user '%s': %s", id, err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to retrieve repositories for user %s.", id))
	}

	var repos []*dbtypes.Repository
	for _, repo := range repositories {
		data := dbtypes.FromRepositoryDbModel(internal.GlobalContainer.Database, &repo)
		repos = append(repos, data)
	}

	return result.Ok(repos)
}

func CreateRepository(name string, ownerID string) *result.Result {
	// Check if `name`

	id := internal.GlobalContainer.Snowflake.Generate().String()
	repository, err := internal.GlobalContainer.Database.Repositories.CreateOne(
		db.Repositories.OwnerID.Set(ownerID),
		db.Repositories.Name.Set(name),
		db.Repositories.ID.Set(id),
	).Exec(context.TODO())

	if err != nil {
		logrus.Errorf("Unable to create repository %s belonging to owner %s: %s", name, ownerID, err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to create repository %s.", name))
	}

	return result.Ok(dbtypes.FromRepositoryDbModel(internal.GlobalContainer.Database, repository))
}

func UpdateRepository(id string, updates map[string]any) *result.Result {
	return nil
}
