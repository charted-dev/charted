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

package internal

import (
	"context"
	"crypto/tls"
	"database/sql"
	"errors"
	"fmt"
	"github.com/bwmarrin/snowflake"
	"github.com/getsentry/sentry-go"
	"github.com/go-redis/redis/v8"
	ratelimit "github.com/noelware/chi-ratelimit"
	"github.com/noelware/chi-ratelimit/providers/inmemory"
	"github.com/sirupsen/logrus"
	"github.com/uptrace/bun"
	"github.com/uptrace/bun/dialect/pgdialect"
	"github.com/uptrace/bun/driver/pgdriver"
	"noelware.org/charted/server/internal/search"
	"noelware.org/charted/server/internal/storage"
	"noelware.org/charted/server/internal/storage/filesystem"
	"time"
)

// GlobalContainer represents the global Container instance that is constructed using
// the NewContainer function.
var GlobalContainer *Container = nil

type Container struct {
	Ratelimiter *ratelimit.Ratelimiter
	Snowflake   *snowflake.Node
	Database    *bun.DB
	Storage     storage.BaseStorageTrailer
	Search      *search.Engine
	Sentry      *sentry.Client
	Redis       *redis.Client
	Config      *Config
}

func NewContainer(config *Config) {
	if GlobalContainer != nil {
		panic(errors.New("global container was already constructed"))
	}

	logrus.Info("Creating global container...")

	// May 2022
	snowflake.Epoch = int64(1651388400000)

	// TODO: increment counter once charted-server should
	// be distributed.
	node, err := snowflake.NewNode(0)
	if err != nil {
		logrus.Fatalf("Unable to create Twitter Snowflake generator because: %s", err)
	}

	ctx, cancel1 := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel1()

	logrus.Info("Connecting to PostgreSQL...")

	opts := []pgdriver.Option{
		pgdriver.WithNetwork("tcp"),
		pgdriver.WithAddr(fmt.Sprintf("%s:%d", config.Database.Host, config.Database.Port)),
		pgdriver.WithUser(config.Database.Username),
		pgdriver.WithPassword(config.Database.Password),
		pgdriver.WithDatabase(config.Database.Db),
		pgdriver.WithApplicationName("charted_server"),
		pgdriver.WithReadTimeout(15 * time.Second),
		pgdriver.WithWriteTimeout(30 * time.Second),
		pgdriver.WithTLSConfig(nil),
	}

	if config.Database.EnableTls {
		opts = append(opts, pgdriver.WithTLSConfig(new(tls.Config)))
	} else {
		opts = append(opts, pgdriver.WithTLSConfig(nil))
	}

	conn := pgdriver.NewConnector(opts...)
	sqldb := sql.OpenDB(conn)
	db := bun.NewDB(sqldb, pgdialect.New())

	if _, err := db.Conn(ctx); err != nil {
		logrus.Fatalf("Unable to connect to PostgreSQL: %s", err)
	}

	logrus.Infof("Connected to PostgreSQL! Now connecting to Redis...")
	var redisClient *redis.Client

	if len(config.Redis.Sentinels) > 0 {
		logrus.Debug("Detected a Redis sentinel connection!")

		password := ""
		if config.Redis.Password != nil {
			password = *config.Redis.Password
		}

		masterName := ""
		if config.Redis.MasterName == nil {
			logrus.Fatal("Missing configuration option 'redis.master_name' to use a Sentinel connection.")
		} else {
			masterName = *config.Redis.MasterName
		}

		redisClient = redis.NewFailoverClient(&redis.FailoverOptions{
			SentinelAddrs: config.Redis.Sentinels,
			MasterName:    masterName,
			Password:      password,
			DB:            config.Redis.DbIndex,
			DialTimeout:   10 * time.Second,
			ReadTimeout:   15 * time.Second,
			WriteTimeout:  15 * time.Second,
		})
	} else {
		logrus.Debug("Detected a Redis standalone connection!")

		password := ""
		if config.Redis.Password != nil {
			password = *config.Redis.Password
		}

		redisClient = redis.NewClient(&redis.Options{
			Password:     password,
			Addr:         fmt.Sprintf("%s:%d", config.Redis.Host, config.Redis.Port),
			DB:           config.Redis.DbIndex,
			DialTimeout:  10 * time.Second,
			ReadTimeout:  15 * time.Second,
			WriteTimeout: 15 * time.Second,
		})
	}

	ctx2, cancel2 := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel2()

	if err := redisClient.Ping(ctx2).Err(); err != nil {
		logrus.Fatalf("Unable to connect to Redis: %s", err)
	}

	ratelimiter := ratelimit.NewRatelimiter(
		ratelimit.WithProvider(inmemory.NewProvider()),
		ratelimit.WithDefaultLimit(1200))

	logrus.Info("Connected to Redis! Creating storage trailer...")
	var trailer storage.BaseStorageTrailer

	if config.Storage.Fs != nil {
		// fs is just an alias to filesystem, so let's prevent it.
		if config.Storage.Filesystem != nil {
			logrus.Fatalf("Cannot have 'storage.fs' and 'storage.filesystem' present at once.")
		}

		trailer = filesystem.NewTrailer(config.Storage.Fs)
		trailer.Init()
	}

	if config.Storage.Filesystem != nil {
		// fs is just an alias to filesystem, so let's prevent it.
		if config.Storage.Fs != nil {
			logrus.Fatalf("Cannot have 'storage.fs' and 'storage.filesystem' present at once.")
		}

		trailer = filesystem.NewTrailer(config.Storage.Filesystem)
		trailer.Init()
	}

	if config.Storage.S3 != nil {
		logrus.Info("coming soon!")
	}

	if trailer == nil {
		logrus.Fatal("Was not able to create storage trailer. Did you provide the right config?")
	}

	logrus.Infof("Initialized the %s storage trailer!", trailer.Name())

	container := &Container{
		Ratelimiter: ratelimiter,
		Snowflake:   node,
		Database:    db,
		Storage:     trailer,
		Search:      nil,
		Sentry:      nil,
		Redis:       redisClient,
		Config:      config,
	}

	GlobalContainer = container
}
