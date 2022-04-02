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
	"time"

	"github.com/go-chi/chi/v5/middleware"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/util"
)

func Log(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, req *http.Request) {
		s := time.Now()
		ww := middleware.NewWrapResponseWriter(w, req.ProtoMajor)
		next.ServeHTTP(ww, req)

		uid, ok := req.Context().Value("user_id").(string)

		if ok {
			code := util.GetStatusCode(ww.Status())
			logrus.
				WithField("remote-addr", req.RemoteAddr).
				WithField("ua", req.Header.Get("User-Agent")).
				WithField("user_id", uid).
				Infof("%s %s %s => %d %s (%d bytes written | %s)",
					req.Method,
					req.URL.EscapedPath(),
					req.Proto,
					ww.Status(),
					code,
					ww.BytesWritten(),
					time.Since(s).String(),
				)
		} else {
			code := util.GetStatusCode(ww.Status())
			logrus.
				WithField("remote-addr", req.RemoteAddr).
				WithField("ua", req.Header.Get("User-Agent")).
				Infof("%s %s %s => %d %s (%d bytes written | %s)",
					req.Method,
					req.URL.EscapedPath(),
					req.Proto,
					ww.Status(),
					code,
					ww.BytesWritten(),
					time.Since(s).String(),
				)
		}
	})
}
