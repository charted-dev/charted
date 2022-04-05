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
	"net/http"
	"strings"

	"google.golang.org/grpc"
	"noelware.org/charted/server/internal/result"
)

func GrpcConnection(server *grpc.Server) func(http.Handler) http.Handler {
	fn := func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if strings.HasPrefix(r.URL.Path, "/grpc") {
				contentType := r.Header.Get("Content-Type")
				if contentType == "" || strings.HasPrefix(contentType, "application/json") {
					// Error if we are using HTTP to connect to the gRPC service.
					res := result.Err(400, "CANNOT_CONNECT", "You are required to have `application/grpc` as the Content-Type to use the gRPC service.")
					res.Write(w)

					return
				}

				if r.ProtoMajor == 2 && strings.HasPrefix(contentType, "application/grpc") {
					server.ServeHTTP(w, r)
				} else {
					next.ServeHTTP(w, r)
				}

				return
			}

			next.ServeHTTP(w, r)
		})
	}

	return fn
}
