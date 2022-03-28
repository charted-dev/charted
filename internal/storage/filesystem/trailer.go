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

package filesystem

import (
	"github.com/sirupsen/logrus"
	"noelware.org/charted/server/internal/storage"
	"os"
	"path/filepath"
)

// Config represents the configuration for the Trailer.
type Config struct {
	Directory string `toml:"directory"`
}

type Trailer struct {
	directory string
}

func NewTrailer(config *Config) storage.BaseStorageTrailer {
	return Trailer{config.Directory}
}

func (fs Trailer) Name() string {
	return "filesystem"
}

func (fs Trailer) Init() {
	logrus.Debugf("Pre-init: Checking if directory '%s' exists...", fs.directory)

	if _, err := os.Stat(fs.directory); err != nil {
		if os.IsNotExist(err) {
			logrus.Debugf("Pre-init: Directory '%s' doesn't exist, now creating...", fs.directory)
			err = os.MkdirAll(filepath.Dir(fs.directory), 0755)
			if err != nil {
				logrus.Fatalf("Pre-init: Unable to recursively create parent/sibling directories for directory '%s' because: %s", fs.directory, err)
			}
		} else {
			logrus.Fatalf("Pre-init: Unable to stat directory '%s' because: %s", fs.directory, err)
		}
	}

	logrus.Debugf("Pre-initialization has completed, we are fine.")
}

func (fs Trailer) HandleUpload(_ []storage.UploadRequest) error {
	return nil
}

func (fs Trailer) GetMetadata(_ string, _ string) (*storage.RepositoryMetadata, error) {
	return nil, nil
}
