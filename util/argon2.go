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

package util

import (
	"crypto/rand"
	"crypto/subtle"
	"encoding/base64"
	"fmt"
	"strings"

	"golang.org/x/crypto/argon2"
)

// GeneratePassword creates a password based off the Argon2 specification using
// the golang.org/x/crypto/argon2 package.
func GeneratePassword(password string) (string, error) {
	salt := make([]byte, 16)
	if _, err := rand.Read(salt); err != nil {
		return "", err
	}

	hash := argon2.IDKey([]byte(password), salt, 1, 64*1024, 4, 32)
	b64Salt := base64.RawStdEncoding.EncodeToString(salt)
	b64Hash := base64.RawStdEncoding.EncodeToString(hash)

	format := "$argon2id$v=%d$m=%d,t=%d,p=%d$%s$%s"
	return fmt.Sprintf(format, argon2.Version, 64*1024, 1, 4, b64Salt, b64Hash), nil
}

// VerifyPassword verifies the password to decode it and check if it's valid
// from the database entry.
func VerifyPassword(password string, hash string) (bool, error) {
	parts := strings.Split(hash, "$")
	memory := 64 * 1024
	t := 1
	p := 4

	_, err := fmt.Sscanf(parts[3], "m=%d,t=%d,p=%d", &memory, &t, &p)
	if err != nil {
		return false, err
	}

	salt, err := base64.RawStdEncoding.DecodeString(parts[4])
	if err != nil {
		return false, err
	}

	decoded, err := base64.RawStdEncoding.DecodeString(parts[5])
	if err != nil {
		return false, err
	}

	keyLen := uint32(len(decoded))
	compare := argon2.IDKey([]byte(password), salt, 1, 64*1024, 4, keyLen)

	return subtle.ConstantTimeCompare(decoded, compare) == 1, nil
}
