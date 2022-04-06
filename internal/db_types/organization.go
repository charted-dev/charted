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

type Organization struct {
	SingleSaslEnabled bool      `json:"single_sasl_enabled"`
	VerifiedPublisher bool      `json:"verified_publisher"`
	TwitterHandle     *string   `json:"twitter_handle,omitempty"`
	GravatarEmail     *string   `json:"gravatar_email,omitempty"`
	Description       *string   `json:"description,omitempty"`
	DisplayName       *string   `json:"display_name,omitempty"`
	AvatarHash        *string   `json:"avatar_hash,omitempy"`
	UpdatedAt         time.Time `json:"updated_at"`
	CreatedAt         time.Time `json:"created_at"`
	Flags             int       `json:"flags"`
	Name              string    `json:"name"`
	ID                string    `json:"id"`
}

func FromOrgDbModel(model *db.OrganizationModel) *Organization {
	return &Organization{
		SingleSaslEnabled: model.InnerOrganization.SingleSaslEnabled,
		VerifiedPublisher: model.InnerOrganization.VerifiedPublisher,
		TwitterHandle:     model.InnerOrganization.TwitterHandle,
		GravatarEmail:     model.InnerOrganization.GravatarEmail,
		Description:       model.InnerOrganization.Description,
		DisplayName:       model.InnerOrganization.DisplayName,
		AvatarHash:        model.InnerOrganization.AvatarHash,
		UpdatedAt:         model.InnerOrganization.UpdatedAt,
		CreatedAt:         model.InnerOrganization.CreatedAt,
		Flags:             model.InnerOrganization.Flags,
		Name:              model.InnerOrganization.Name,
		ID:                model.InnerOrganization.ID,
	}
}
