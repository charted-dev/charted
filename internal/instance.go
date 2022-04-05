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
	"io/ioutil"
	"os"

	"github.com/google/uuid"
	"github.com/sirupsen/logrus"
)

var instanceUUID *string

func GetInstanceUUID() string {
	// Since we already populated this, it'll just return it.
	if instanceUUID != nil {
		return *instanceUUID
	}

	// Check if it's as an environment variable, let's check
	if uuid, ok := os.LookupEnv("CHARTED_INSTANCE_UUID"); ok {
		instanceUUID = &uuid
		return uuid
	}

	// Check if it's under instance.uuid in the main directory
	if _, err := os.Stat("./instance.uuid"); err != nil {
		if os.IsNotExist(err) {
			logrus.Debugf("Missing `instance.uuid` file to detect for Noelware Analytics (if enabled in config)")

			i := uuid.NewString()
			if err := ioutil.WriteFile("./instance.uuid", []byte(i), 0o600); err != nil {
				logrus.Errorf("Unable to write UUID '%s' to ./instance.uuid: %s", i, err)
				instanceUUID = &i

				return i
			}

			instanceUUID = &i
			return i
		}

		logrus.Fatalf("Unable to stat ./instance.uuid: %s", err)
	}

	contents, err := ioutil.ReadFile("./instance.uuid")
	if err != nil {
		logrus.Fatalf("Unable to read contents of instance.uuid: %s", err)
	}

	return string(contents)
}
