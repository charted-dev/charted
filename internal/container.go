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
	"errors"
	"fmt"
	"os"
	"time"

	"github.com/bwmarrin/snowflake"
	"github.com/getsentry/sentry-go"
	"github.com/go-redis/redis/v8"
	ratelimit "github.com/noelware/chi-ratelimit"
	redisrl "github.com/noelware/chi-ratelimit-redis"
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal/email"
	"noelware.org/charted/server/internal/search"
	"noelware.org/charted/server/internal/search/elastic"
	"noelware.org/charted/server/internal/search/meilisearch"
	"noelware.org/charted/server/internal/search/noop"
	"noelware.org/charted/server/internal/storage"
	"noelware.org/charted/server/internal/storage/filesystem"
	"noelware.org/charted/server/prisma/db"
)

// GlobalContainer represents the global Container instance that is constructed using
// the NewContainer function.
var GlobalContainer *Container

type Container struct {
	Ratelimiter *ratelimit.Ratelimiter
	Database    *db.PrismaClient
	Snowflake   *snowflake.Node
	Storage     storage.BaseStorageTrailer
	Search      search.Engine
	Sentry      *sentry.Client
	Redis       *redis.Client
	Email       *email.Service
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
		logrus.WithField("step", "bootstrap->snowflake").Fatalf("Unable to create Twitter Snowflake generator because: %s", err)
	}

	logrus.WithField("step", "bootstrap->postgres").Info("Now connecting to PostgreSQL...")
	prisma := db.NewClient()
	if err := prisma.Connect(); err != nil {
		logrus.WithField("step", "bootstrap->postgres").Fatalf("Unable to connect to PostgreSQL: %s", err)
	}

	logrus.WithField("step", "bootstrap->postgres").Infof("Connected to PostgreSQL! Now connecting to Redis...")
	var redisClient *redis.Client

	if len(config.Redis.Sentinels) > 0 {
		logrus.WithField("step", "bootstrap->redis").Debug("Detected a Redis sentinel connection!")

		password := ""
		if config.Redis.Password != nil {
			password = *config.Redis.Password
		}

		masterName := ""
		if config.Redis.MasterName == nil {
			logrus.WithField("step", "bootstrap->redis").Fatal("Missing configuration option 'redis.master_name' to use a Sentinel connection.")
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
		logrus.WithField("step", "bootstrap->redis").Debug("Detected a Redis standalone connection!")

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

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	if err := redisClient.Ping(ctx).Err(); err != nil {
		logrus.WithField("step", "bootstrap->redis").Fatalf("Unable to connect to Redis: %s", err)
	}

	// the only error we'll get if the client is nil,
	// which shouldn't be in this case >:(
	redisProvider, _ := redisrl.New(
		redisrl.WithClient(redisClient),
		redisrl.WithKeyPrefix("charted:ratelimits"),
	)

	ratelimiter := ratelimit.NewRatelimiter(
		ratelimit.WithProvider(redisProvider),
		ratelimit.WithDefaultLimit(1200))

	logrus.WithField("step", "bootstrap->redis").Info("Connected to Redis! Creating storage trailer...")
	var trailer storage.BaseStorageTrailer

	if config.Storage.Fs != nil {
		// fs is just an alias to filesystem, so let's prevent it.
		if config.Storage.Filesystem != nil {
			logrus.WithField("step", "bootstrap->storage->fs").Fatalf("Cannot have 'storage.fs' and 'storage.filesystem' present at once.")
		}

		trailer = filesystem.NewTrailer(config.Storage.Fs)
		trailer.Init()
	}

	if config.Storage.Filesystem != nil {
		// fs is just an alias to filesystem, so let's prevent it.
		if config.Storage.Fs != nil {
			logrus.WithField("step", "bootstrap->storage->fs").Fatalf("Cannot have 'storage.fs' and 'storage.filesystem' present at once.")
		}

		trailer = filesystem.NewTrailer(config.Storage.Filesystem)
		trailer.Init()
	}

	if config.Storage.S3 != nil {
		logrus.WithField("step", "bootstrap->storage->s3").Info("coming soon!")
	}

	if trailer == nil {
		logrus.WithField("step", "bootstrap->storage").Fatal("Was not able to create storage trailer. Did you provide the right config?")
	} else {
		logrus.WithField("step", "bootstrap->storage").Infof("Initialized the %s storage trailer!", trailer.Name())
	}

	service := noop.New()
	if config.Search != nil {
		logrus.WithField("step", "bootstrap->search").Info("Search configuration was defined, now determining which one to use...")

		if config.Search.Elastic != nil {
			logrus.WithField("step", "bootstrap->search->elastic").Info("Detected Elasticsearch configuration, now using!")

			service = elastic.NewService(config.Search.Elastic)
		}

		if config.Search.Meili != nil {
			logrus.WithField("step", "bootstrap->search->meili").Info("Detected Meilisearch configuration!")
			service = meilisearch.NewService(config.Search.Meili)
		}
	}

	if service.Type().String() != search.Unknown.String() {
		logrus.WithField("step", "bootstrap->search").Infof("Initialized the %s search engine!", service.Type())
	}

	var sentryClient *sentry.Client
	if config.SentryDSN != nil {
		logrus.WithField("step", "bootstrap->sentry").Infof("Sentry DSN was provided, now installing...")
		hostName, err := os.Hostname()
		if err != nil {
			hostName = "localhost"
		}

		client, err := sentry.NewClient(sentry.ClientOptions{
			Dsn:              *config.SentryDSN,
			AttachStacktrace: true,
			SampleRate:       1.0,
			ServerName:       fmt.Sprintf("noelware.charted_server v%s @ %s", Version, hostName),
		})

		if err != nil {
			logrus.WithField("step", "bootstrap->sentry").Fatalf("Unable to initialize Sentry: %s", err)
		}

		sentryClient = client
	}

	var emailService *email.Service
	if config.Email != nil {
		logrus.WithField("step", "bootstrap->email").Info("Enabling email service...")
		emailService = email.NewEmailService(config.Email)
	}

	container := &Container{
		Ratelimiter: ratelimiter,
		Database:    prisma,
		Snowflake:   node,
		Storage:     trailer,
		Search:      service,
		Sentry:      sentryClient,
		Redis:       redisClient,
		Config:      config,
		Email:       emailService,
	}

	GlobalContainer = container
}
