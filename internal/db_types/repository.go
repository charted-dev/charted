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

package dbtypes

import (
	"context"
	"errors"
	"time"

	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/prisma/db"
)

type Repository struct {
	StargazersCount int       `json:"stargazers_count"`
	DownloadsCount  int       `json:"downloads_count"`
	Description     *string   `json:"description,omitempty"`
	UpdatedAt       time.Time `json:"updated_at"`
	CreatedAt       time.Time `json:"created_at"`
	IconHash        *string   `json:"icon_hash,omitempty"`
	Owner           any       `json:"owner,omitempty"`
	Name            string    `json:"name"`
	ID              string    `json:"id"`
}

func FromRepositoryDbModel(client *db.PrismaClient, model *db.RepositoriesModel) *Repository {
	var owner any

	user, err := client.Users.FindUnique(db.Users.ID.Equals(model.OwnerID)).Exec(context.TODO())
	if err != nil {
		if errors.Is(err, db.ErrNotFound) {
			org, err := client.Organization.FindUnique(db.Organization.ID.Equals(model.OwnerID)).Exec(context.TODO())
			if err != nil {
				logrus.Errorf("Unable to find a entry from 'repositories.%s.owner_id' = '%s': %s", model.InnerRepositories.ID, model.OwnerID, err)
				return nil
			}

			owner = FromOrgDbModel(org)
		}
	} else {
		owner = FromUserDbModel(user)
	}

	return &Repository{
		StargazersCount: model.InnerRepositories.StargazersCount,
		DownloadsCount:  model.InnerRepositories.DownloadsCount,
		Description:     model.InnerRepositories.Description,
		UpdatedAt:       model.InnerRepositories.UpdatedAt,
		CreatedAt:       model.InnerRepositories.CreatedAt,
		IconHash:        model.InnerRepositories.IconHash,
		Owner:           owner,
		Name:            model.InnerRepositories.Name,
		ID:              model.InnerRepositories.ID,
	}
}
