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
	"net/mail"

	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal"
	dbtypes "noelware.org/charted/server/internal/db_types"
	"noelware.org/charted/server/internal/result"
	"noelware.org/charted/server/prisma/db"
	"noelware.org/charted/server/util"
)

type UserController struct{}

func (UserController) Get(id string) *result.Result {
	user, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(id)).Exec(context.TODO())
	if err != nil {
		if errors.Is(err, db.ErrNotFound) {
			return result.Err(404, "USER_NOT_FOUND", fmt.Sprintf("User with ID '%s' was not found.", id))
		} else {
			logrus.Errorf("Unable to fetch user '%s': %s", id, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to fetch user '%s'.", id))
		}
	}

	return result.Ok(dbtypes.FromUserDbModel(user))
}

func (UserController) Create(
	username string,
	password string,
	email string,
) *result.Result {
	if !internal.GlobalContainer.Config.Registrations {
		return result.Err(403, "REGISTRATIONS_DISABLED", "The server administrators has disabled registrations server-side, please use an invite code.")
	}

	// Check if the username is taken
	userByName, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.Username.Equals(username)).Exec(context.TODO())
	if err != nil && !errors.Is(err, db.ErrNotFound) {
		logrus.Errorf("Unable to query information from PostgreSQL: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unknown database error had occurred.")
	}

	if userByName != nil {
		return result.Err(400, "USERNAME_ALREADY_TAKEN", fmt.Sprintf("Username '%s' is already taken.", username))
	}

	// Check if the email is valid
	_, err = mail.ParseAddress(email)
	if err != nil {
		return result.Err(406, "INVALID_EMAIL_ADDRESS", fmt.Sprintf("Email %s is not a valid email address.", email))
	}

	// Check if the email is taken
	userByEmail, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.Email.Equals(email)).Exec(context.TODO())
	if err != nil && !errors.Is(err, db.ErrNotFound) {
		logrus.Errorf("Unable to query information from PostgreSQL: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unknown database error had occurred.")
	}

	if userByEmail != nil {
		return result.Err(400, "USERNAME_ALREADY_TAKEN", fmt.Sprintf("Email '%s' is already taken.", email))
	}

	hash, err := util.GeneratePassword(password)
	if err != nil {
		logrus.Errorf("Unable to generate Argon2 password: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unknown password algorithm error had occurred.")
	}

	// Generate a user ID
	id := internal.GlobalContainer.Snowflake.Generate().String()

	user, err := internal.GlobalContainer.Database.Users.CreateOne(
		db.Users.Username.Set(username),
		db.Users.Password.Set(hash),
		db.Users.Email.Set(email),
		db.Users.ID.Set(id),
		db.Users.Repositories.Link(),
	).Exec(context.TODO())

	if err != nil {
		logrus.Errorf("Unable to create user in database: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unable to create user, try again later.")
	}

	// Create the user connections
	connId := internal.GlobalContainer.Snowflake.Generate().String()
	_, err = internal.GlobalContainer.Database.UserConnections.CreateOne(
		db.UserConnections.Owner.Link(db.Users.ID.Equals(user.ID)),
		db.UserConnections.ID.Set(connId),
	).Exec(context.TODO())

	if err != nil {
		logrus.Errorf("Unable to create user connections: %s", err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", "Unable to create user connections row, try again later.")
	}

	return result.OkWithStatus(201, dbtypes.FromUserDbModel(user))
}

func (UserController) Update(
	userId string,
	update map[string]any,
) *result.Result {
	if util.IsEmpty(update) {
		return result.Err(406, "MISSING_UPDATE_PAYLOAD", "You are missing properties to update.")
	}

	operations := map[string]bool{}

	user, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(userId)).Exec(context.TODO())
	if err != nil {
		if errors.Is(err, db.ErrNotFound) {
			return result.Err(404, "USER_DOESNT_EXIST", fmt.Sprintf("User with ID '%s' doesn't exist.", userId))
		} else {
			logrus.Errorf("Unable to find user with ID %s: %s", userId, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to find user with ID '%s' :(", userId))
		}
	}

	// Check if we need to set the gravatar email to be used.
	if gravatarEmail, ok := update["gravatar_email"].(string); ok {
		_, err := mail.ParseAddress(gravatarEmail)
		if err != nil {
			logrus.Errorf("Unable to parse email address '%s': %s", gravatarEmail, err)
			return result.Err(406, "INVALID_EMAIL_ADDRESS", fmt.Sprintf("Email %s is not a valid email address.", gravatarEmail))
		}

		// Compare if they are the same email.
		if user.InnerUsers.GravatarEmail != nil && *user.InnerUsers.GravatarEmail == gravatarEmail {
			return result.Err(406, "SAME_GRAVATAR_EMAIL_ADDRESS", fmt.Sprintf("Email %s is the same the one stored in the database; cannot update", gravatarEmail))
		}

		// Let's update it
		if _, err := internal.GlobalContainer.Database.Users.FindMany(db.Users.ID.Equals(userId)).Update(
			db.Users.GravatarEmail.Set(gravatarEmail),
		).Exec(context.TODO()); err != nil {
			logrus.Errorf("Unable to update entry 'users.%s.gravatar_email' = '%s': %s", userId, gravatarEmail, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to update entry 'users.%s.gravatar_email'.", userId))
		} else {
			operations["gravatar_email"] = true
		}
	}

	// Check if we need to update the description
	if description, ok := update["description"].(string); ok {
		if len(description) > 240 {
			return result.Err(406, "DESCRIPTION_TOO_LONG", "The description to update was too long, exceeded over 240 characters.")
		}

		if _, err := internal.GlobalContainer.Database.Users.FindMany(db.Users.ID.Equals(userId)).Update(
			db.Users.Description.Set(description),
		).Exec(context.TODO()); err != nil {
			logrus.Errorf("Unable to update enetry 'users.%s.description' = '%s': %s", userId, description, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to update entry 'users.%s.description'.", userId))
		} else {
			operations["description"] = true
		}
	}

	return result.Ok(operations)
}
