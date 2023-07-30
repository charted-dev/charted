// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package indexer

import (
	"encoding/json"
	"os"
	"os/signal"
	"strings"
	"sync"
	"syscall"
	"time"

	"charts.noelware.org/search-indexer/pkg/config"
	"charts.noelware.org/search-indexer/pkg/database"
	"github.com/lib/pq"
	"github.com/sirupsen/logrus"
)

// Indexer represents the indexer that is used to index objects in
// Elasticsearch or Meilisearch.
type Indexer struct {
	listeners []database.Listener
	config    *config.Config
}

// New creates a new Indexer object with a listener established. No events
// will be polled until Indexer.SpawnRunner() is called.
func New(config *config.Config) (*Indexer, error) {
	return &Indexer{listeners: make([]database.Listener, 0), config: config}, nil
}

// Spawn will spawn a long-running indexer, which will react to new events from the pq.Listener
// interface. This will react to syscalls to exit the program, which will teardown the
// pq.Listener interface and stop processing new events.
func (i *Indexer) Spawn() {
	logrus.WithField("reporter", "indexer.Spawn").Info("now starting long-running indexer!")
	listen := pq.NewListener(i.config.DatabaseURL, 5*time.Second, time.Minute, func(event pq.ListenerEventType, err error) {
		switch event {
		case pq.ListenerEventConnected:
			logrus.WithField("reporter", "pq.Listener").Info("established connection successfully")
		case pq.ListenerEventReconnected:
			logrus.WithField("reporter", "pq.Listener").Info("db connection has reconnected!")
		case pq.ListenerEventConnectionAttemptFailed:
			logrus.WithField("reporter", "pq.Listener").Fatal("unable to connect to database: ", err)
		default:
			/* do nothing */
		}
	})

	if err := listen.Listen(database.ListenChannel); err != nil {
		logrus.WithField("reporter", "indexer.Spawn").WithError(err).Fatal("unable to listen to channel [", database.ListenChannel, "]")
		return
	}

	canceller := make(chan os.Signal, 1)
	signal.Notify(canceller, syscall.SIGTERM, syscall.SIGINT)

	go func() {
		for {
			select {
			case notif := <-listen.Notify:
				logrus.WithField("reporter", "indexer.Spawn").WithField("bpid", notif.BePid).Debug("received new event")
				logrus.Trace("payload: ", notif.Extra)

				var payload NotificationPayload
				if err := json.Unmarshal([]byte(notif.Extra), &payload); err != nil {
					logrus.WithField("reporter", "indexer.Spawn").WithField("bpid", notif.BePid).WithError(err).Error("unable to unmarshal from json")
					continue
				}

				if !contains(database.Tables, payload.Table) {
					continue
				}

				for _, listener := range i.listeners {
					switch payload.Action {
					case "INSERT":
						listener.Create(payload.Table, payload.Data)
					case "DELETE":
						listener.Delete(payload.Table, payload.Data)
					case "UPDATE":
						listener.Update(payload.Table, payload.Data)
					}
				}

			case <-time.After(90 * time.Second):
				go listen.Ping()
				logrus.WithField("reporter", "indexer.Spawn").Debug("received no new events in 90 seconds, checking for new events")
			}
		}
	}()

	<-canceller
	logrus.WithField("reporter", "indexer.Spawn").Warn("caught SIGINT/SIGTERM syscall, disposing listeners")
	for _, l := range i.listeners {
		l.Disposed()
	}

	if err := listen.Close(); err != nil {
		logrus.WithField("reporter", "indexer.Spawn").WithError(err).Error("unable to dispose pq.Listener interface, things might go wrong")
	}
}

// AppendListener will insert a new listener.
func (i *Indexer) AppendListener(listener database.Listener) {
	i.listeners = append(i.listeners, listener)
}

// IndexAll will perform indexing on all objects and close the
// database connection.
func IndexAll(db *database.Database) error {
	logrus.WithField("reporter", "indexer.IndexAll").Info("Now performing indexes on tables: [", strings.Join(database.Tables, ", "), "]")

	completed := make(chan any)
	wg := sync.WaitGroup{}
	for _, table := range database.Tables {
		wg.Add(1)
		table := table

		go func() {
			defer wg.Done()
			if err := processBulkIndexing(db, table, completed); err != nil {
				logrus.WithField("reporter", "indexer.IndexAll").WithField("table", table).Error("unable to process all indices for table: ", err)
			}
		}()
	}

	go func() {
		wg.Wait()
		close(completed)
	}()

	// Waits for the channel to be closed.
	for range completed {
	}

	return nil
}

func processBulkIndexing(db *database.Database, table string, completed chan any) error {
	//start := time.Now()
	logrus.WithField("reporter", "indexer.process").WithField("table", table).Debug("starting indexing...")

	// the last snowflake
	//var lastId uint64

	return nil
}

func contains[T comparable](list []T, item T) bool {
	for _, i := range list {
		if i == item {
			return true
		}
	}

	return false
}
