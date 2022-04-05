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
	"time"

	"noelware.org/charted/server/prisma/db"
)

type User struct {
	GravatarEmail *string   `json:"gravatar_email"`
	Description   *string   `json:"description"`
	AvatarHash    *string   `json:"avatar_hash"` //nolint
	UpdatedAt     time.Time `json:"updated_at"`
	CreatedAt     time.Time `json:"created_at"`
	Username      string    `json:"username"`
	Flags         int       `json:"flags"`
	Name          *string   `json:"name"`
	Id            string    `json:"id"` //nolint
}

func FromUserDbModel(user *db.UsersModel) *User {
	return &User{
		GravatarEmail: user.InnerUsers.GravatarEmail,
		Description:   user.InnerUsers.Description,
		AvatarHash:    user.InnerUsers.AvatarHash,
		UpdatedAt:     user.UpdatedAt,
		CreatedAt:     user.CreatedAt,
		Username:      user.Username,
		Flags:         user.Flags,
		Name:          user.InnerUsers.Name,
		Id:            user.ID,
	}
}

type Connection struct {
	NoelwareAccountID *string `json:"noelware_account_id"`
	GoogleAccountID   *string `json:"google_account_id"`
	AppleAccountID    *string `json:"apple_account_id"`
	ID                string  `json:"id"`
}

func FromConnectionDbModel(model *db.UserConnectionsModel) *Connection {
	return &Connection{
		NoelwareAccountID: model.InnerUserConnections.NoelwareAccountID,
		GoogleAccountID:   model.InnerUserConnections.GoogleAccountID,
		AppleAccountID:    model.InnerUserConnections.AppleAccountID,
		ID:                model.ID,
	}
}
