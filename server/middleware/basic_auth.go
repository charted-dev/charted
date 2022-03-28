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

package middleware

import (
	"crypto/subtle"
	"net/http"
	"noelware.org/charted/server/internal"
	"noelware.org/charted/server/internal/result"
)

func BasicAuth(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, req *http.Request) {
		// Check if there is basic authentication enabled
		if internal.GlobalContainer.Config.Username != nil && internal.GlobalContainer.Config.Password != nil {
			u := internal.GlobalContainer.Config.Username
			p := internal.GlobalContainer.Config.Password

			// If there is an Authorization token, let's skip it
			auth := req.Header.Get("Authorization")
			if auth != "" {
				next.ServeHTTP(w, req)
				return
			}

			user, pass, ok := req.BasicAuth()
			if !ok {
				w.Header().Add("WWW-Authenticate", `Basic realm="Noelware/charted-server"`)

				res := result.Err(http.StatusUnauthorized, "UNABLE_TO_OBTAIN", "Couldn't authenticate due to server being secured by basic auth.")
				res.Write(w)
				return
			}

			if user != *u {
				w.Header().Add("WWW-Authenticate", `Basic realm="Noelware/charted-server"`)

				res := result.Err(http.StatusUnauthorized, "INVALID_USERNAME", "Invalid username.")
				res.Write(w)
				return
			}

			if subtle.ConstantTimeCompare([]byte(*p), []byte(pass)) != 1 {
				w.Header().Add("WWW-Authenticate", `Basic realm="Noelware/charted-server"`)

				res := result.Err(http.StatusUnauthorized, "INVALID_PASSWORD", "Invalid password.")
				res.Write(w)
				return
			}

			next.ServeHTTP(w, req)
		} else {
			next.ServeHTTP(w, req)
		}
	})
}
