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

package sessions

import (
	"context"
	"encoding/json"
	"time"

	"github.com/go-redis/redis/v8"
	"github.com/google/uuid"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/util/jwt"
)

// Manager represents the current session manager that is holding all the
// sessions.
type Manager struct {
	redisClient *redis.Client
}

// Session represents the current session available.
type Session struct {
	RefreshToken string    `json:"refresh_token"`
	LoggedInAt   time.Time `json:"logged_in_at"`
	SessionId    string    `json:"session_id"` //nolint
	UserId       string    `json:"user_id"`    //nolint
	Token        string    `json:"token"`
}

func New(userId string) *Session { //nolint
	// Create refresh token which is a JWT that lasts for 7 days.
	refreshToken, err := jwt.CreateRefreshToken(userId)
	if err != nil {
		return nil
	}

	accessToken, err := jwt.CreateAccessToken(userId)
	if err != nil {
		return nil
	}

	return &Session{
		RefreshToken: refreshToken,
		LoggedInAt:   time.Now(),
		SessionId:    uuid.NewString(),
		UserId:       userId,
		Token:        accessToken,
	}
}

func NewManager(redisClient *redis.Client) *Manager {
	manager := &Manager{redisClient}

	t := time.Now()
	logrus.Debug("Checking how many sessions are available...")

	data, err := redisClient.HGetAll(context.TODO(), "charted:sessions").Result()
	if err != nil {
		logrus.Fatalf("Unable to retrieve session data from Redis (took %s): %s", time.Since(t).String(), err)
	}

	logrus.Debugf("Took %s to collect %d sessions, now checking expired sessions...", time.Since(t).String(), len(data))
	t = time.Now() //nolint

	for key, value := range data {
		var session *Session
		err := json.Unmarshal([]byte(value), &session)
		if err != nil {
			logrus.Warnf("Unable to unmarshal JSON packet for session %s: %s", key, err)
			err = redisClient.HDel(context.TODO(), "charted:sessions", key).Err()
			if err != nil {
				logrus.Errorf("Unable to delete document with key '%s' in 'charted:sessions' hash table: %s", key, err)
			}

			continue
		}
	}

	return manager
}

// Get returns a *Session object if it exists or not.
func (m *Manager) Get(sessionId string) (*Session, error) { //nolint
	data, err := m.redisClient.HGet(context.TODO(), "charted:sessions", sessionId).Result()
	if err != nil {
		if err == redis.Nil {
			return nil, nil
		}

		return nil, err
	}

	var session *Session
	if err := json.Unmarshal([]byte(data), &session); err != nil {
		return nil, err
	}

	return session, nil
}

// GetAll returns all the *Session objects available in Redis, if any.
func (m *Manager) GetAll() ([]*Session, error) {
	data, err := m.redisClient.HGetAll(context.TODO(), "charted:sessions").Result()
	if err != nil {
		logrus.Errorf("Unable to retrieve session data from Redis: %s", err)
		return nil, err
	}

	sessions := make([]*Session, 0)
	for key, value := range data {
		var session *Session
		err := json.Unmarshal([]byte(value), &session)
		if err != nil {
			logrus.Warnf("Unable to unmarshal JSON packet for session %s: %s", key, err)
			err = m.redisClient.HDel(context.TODO(), "charted:sessions", key).Err()
			if err != nil {
				logrus.Errorf("Unable to delete document with key '%s' in 'charted:sessions' hash table: %s", key, err)
			}

			continue
		}

		sessions = append(sessions, session)
	}

	return sessions, nil
}
