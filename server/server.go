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

package server

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/go-chi/chi/v5"
	middie "github.com/go-chi/chi/v5/middleware"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal"
	"noelware.org/charted/server/internal/result"
	"noelware.org/charted/server/server/middleware"
	"noelware.org/charted/server/server/routes"
	v1 "noelware.org/charted/server/server/routes/api/v1"
)

// Start boots the HTTP service that you can interact with.
func Start() error {
	logrus.Info("Starting up HTTP service...")

	router := chi.NewRouter()
	router.NotFound(func(w http.ResponseWriter, req *http.Request) {
		res := result.Err(404, "METHOD_NOT_FOUND", fmt.Sprintf("Unable to find route \"%s %s\"! Are you lost? :blbctscared:",
			req.Method,
			req.URL.EscapedPath(),
		))

		res.Write(w)
	})

	router.MethodNotAllowed(func(w http.ResponseWriter, req *http.Request) {
		res := result.Err(405, "METHOD_NOT_ALLOWED", fmt.Sprintf(":blbctscared: Cannot call route \"%s %s\" due to methods being different. :(",
			req.Method,
			req.URL.EscapedPath(),
		))

		res.Write(w)
	})

	router.Use(
		internal.GlobalContainer.Ratelimiter.Middleware,
		middleware.Log,
		middleware.ErrorHandler,
		middie.GetHead,
		middie.Heartbeat("/ping"),
		middie.RealIP,
	)

	router.Mount("/", routes.NewMainRouter())
	router.Mount("/v1", v1.NewApiV1Router())
	router.Mount("/version", routes.NewVersionRouter())

	port := 3939
	if internal.GlobalContainer.Config.Port != nil {
		port = *internal.GlobalContainer.Config.Port
	}

	host := "0.0.0.0"
	if internal.GlobalContainer.Config.Host != nil {
		host = *internal.GlobalContainer.Config.Host
	}

	addr := fmt.Sprintf("%s:%d", host, port)
	server := &http.Server{
		Addr:         addr,
		Handler:      router,
		WriteTimeout: 10 * time.Second,
		ReadTimeout:  30 * time.Second,
	}

	sigint := make(chan os.Signal, 1)
	signal.Notify(sigint, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		onWalk := chi.WalkFunc(func(method string, route string, handler http.Handler, middlewares ...func(http.Handler) http.Handler) error {
			logrus.Debugf("Registered route %s %s!", method, route)
			return nil
		})

		if err := chi.Walk(router, onWalk); err != nil {
			logrus.Errorf("Unable to print routes: %s", err)
		}

		logrus.Infof("charted-server is now listening under address => %s", addr)
		err := server.ListenAndServe()
		if err != nil && err != http.ErrServerClosed {
			logrus.Errorf("Unable to run HTTP server: %s", err)
		}
	}()

	<-sigint

	logrus.Warn("Shutting down HTTP service...")
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)

	go func() {
		<-ctx.Done()
		if ctx.Err() == context.DeadlineExceeded {
			logrus.Warn("Exceeded deadline when dialing off requests...")
		}
	}()

	defer func() {
		logrus.Warn("Closing off PostgreSQL connection...")
		if err := internal.GlobalContainer.Database.Prisma.Disconnect(); err != nil {
			logrus.Errorf("Unable to close PostgreSQL connection: %s", err)
		}

		logrus.Warn("Closed off PostgreSQL connection! Now closing off Redis...")
		if err := internal.GlobalContainer.Redis.Close(); err != nil {
			logrus.Errorf("Unable to close Redis connection: %s", err)
		}

		logrus.Warn("Closed off everything, goodbye.")
		cancel()
	}()

	if err := server.Shutdown(ctx); err != nil {
		return err
	}

	return nil
}
