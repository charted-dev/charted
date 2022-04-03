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
		if _, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(userId)).Update(
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

		if _, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(userId)).Update(
			db.Users.Description.Set(description),
		).Exec(context.TODO()); err != nil {
			logrus.Errorf("Unable to update enetry 'users.%s.description' = '%s': %s", userId, description, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to update entry 'users.%s.description'.", userId))
		} else {
			operations["description"] = true
		}
	}

	if username, ok := update["username"].(string); ok {
		if len(username) > 32 {
			return result.Err(406, "USERNAME_TOO_LONG", "The username to update was too long, exceeded over 32 characters.")
		}

		// Check if there is non-ascii characters
		if !util.IsASCII(username) {
			return result.Err(406, "INVALID_USERNAME", "The username to update included non-ascii characters.")
		}

		// Check if someone has the username already
		user, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.Username.Equals(username)).Exec(context.TODO())
		if err != nil {
			if !errors.Is(err, db.ErrNotFound) {
				logrus.Errorf("Unable to query information to check if username %s exists: %s", username, err)
				return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to check if username %s existed.", username))
			}
		}

		if user != nil {
			return result.Err(403, "USERNAME_ALREADY_TAKEN", fmt.Sprintf("Username %s is already taken.", username))
		}

		// Update it!
		if _, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(userId)).Update(
			db.Users.Username.Set(username),
		).Exec(context.TODO()); err != nil {
			logrus.Errorf("Unable to update entry 'users.%s.username' = '%s': %s", userId, username, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to update entry 'users.%s.username'.", userId))
		} else {
			operations["username"] = true
		}
	}

	// TODO: should passwords be its own seperate controller function?
	//       brain is probably mush so idk if im overthinking this or what
	//       but at this rate, i dont care imma do this >:(
	if password, ok := update["password"].(string); ok {
		// Generate the hash for it
		hash, err := util.GeneratePassword(password)
		if err != nil {
			logrus.Errorf("Unable to generate Argon2 password: %s", err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", "Unknown password algorithm error had occurred.")
		}

		if _, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(userId)).Update(
			db.Users.Password.Set(hash),
		).Exec(context.TODO()); err != nil {
			logrus.Errorf("Unable to update 'users.%s.password' = '[REDACTED]': %s", userId, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to update password entry."))
		} else {
			operations["password"] = true
		}
	}

	// TODO: include mail service to send out email verification
	//       and invites if the server is in an invite-only basis.
	//       so, updating email for users will not be here for now. :pensive:

	// Well, if there is no email updates, what about user flag updates?
	// You are so wrong. Since flags typically apply administration permissions
	// (and we do NOT need that); this is only included in the administration
	// controller.

	// Atleast you can change your display name. :)
	if name, ok := update["name"].(string); ok {
		// TODO: should display name include non ascii characters?
		//       for now, sure it can include non ascii characters.
		if _, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(userId)).Update(
			db.Users.Name.Set(name),
		).Exec(context.TODO()); err != nil {
			logrus.Errorf("Unable to update 'users.%s.name' = '%s': %s", userId, name, err)
			return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to update display name entry."))
		} else {
			operations["name"] = true
		}
	}

	return result.Ok(operations)
}

func (UserController) Delete(userId string) *result.Result {
	if _, err := internal.GlobalContainer.Database.Users.FindUnique(db.Users.ID.Equals(userId)).Delete().Exec(context.TODO()); err != nil {
		logrus.Errorf("Unable to delete entry 'users.%s': %s", userId, err)
		return result.Err(500, "INTERNAL_SERVER_ERROR", fmt.Sprintf("Unable to delete user with ID %s.", userId))
	} else {
		return result.NoContent()
	}
}
