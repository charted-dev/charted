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

package jwt

import (
	"errors"
	"fmt"
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal"
)

func CreateRefreshToken(userId string) (string, error) { //nolint
	days := 24 * time.Hour
	claims := jwt.MapClaims{}
	claims["user_id"] = userId
	claims["exp"] = time.Now().UTC().Add(7 * days).Unix()

	token := jwt.NewWithClaims(jwt.SigningMethodHS512, claims)

	signed, err := token.SignedString([]byte(internal.GlobalContainer.Config.SecretKeyBase))
	if err != nil {
		return "", err
	}

	return signed, nil
}

func CreateAccessToken(userId string) (string, error) { //nolint
	claims := jwt.MapClaims{}
	claims["user_id"] = userId
	claims["exp"] = time.Now().UTC().Add(24 * time.Hour).Unix()

	token := jwt.NewWithClaims(jwt.SigningMethodHS512, claims)

	signed, err := token.SignedString([]byte(internal.GlobalContainer.Config.SecretKeyBase))
	if err != nil {
		logrus.Errorf("Unable to sign token: %s", err)
		return "", err
	}

	return signed, nil
}

func ValidateToken(token string) (bool, error) {
	t, err := jwt.Parse(token, func(t *jwt.Token) (interface{}, error) {
		if _, ok := t.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", t.Header["alg"])
		}

		return []byte(internal.GlobalContainer.Config.SecretKeyBase), nil
	})

	if err != nil {
		logrus.Errorf("Unable to parse JWT token: %s", err)
		return false, err
	}

	if _, ok := t.Claims.(jwt.MapClaims); ok && t.Valid {
		return true, nil
	}

	return false, errors.New("unable to verify token (was it expired?)")
}

func DecodeToken(token string) (jwt.MapClaims, error) {
	t, err := jwt.Parse(token, func(t *jwt.Token) (interface{}, error) {
		if _, ok := t.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", t.Header["alg"])
		}

		return []byte(internal.GlobalContainer.Config.SecretKeyBase), nil
	})

	if err != nil {
		logrus.Errorf("Unable to parse JWT token: %s", err)
		return nil, err
	}

	if claims, ok := t.Claims.(jwt.MapClaims); ok && t.Valid {
		return claims, nil
	}

	return nil, errors.New("unable to verify token (is it expired?)")
}
