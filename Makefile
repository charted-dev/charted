# 📦 charted: Free, open source, and robust Helm Chart service made in Go.
# Copyright 2022 Noelware <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

GOOS       := $(shell go env GOOS)
GOARCH     := $(shell go env GOARCH)
VERSION    := $(shell cat ./version.json | jq .version | tr -d '"')
GIT_COMMIT := $(shell git rev-parse --short=8 HEAD)
BUILD_DATE := $(shell go run ./cmd/build-date/main.go)

ifeq ($(GOOS),linux)
	TARGET_OS ?= linux
else ifeq ($(GOOS),windows)
	TARGET_OS ?= windows
else ifeq ($(GOOS),darwin)
	TARGET_OS ?= darwin
else
	$(error System $(GOOS)/$(GOARCH) is not supported at this time)
endif

EXTENSION :=
ifeq ($(TARGET_OS),windows)
	EXTENSION := .exe
endif

# Usage: `make help`
.PHONY: help
help: ## Prints this help usage on how to build `charted-server`
    @awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

# Usage: `make deps`
.PHONY: deps
deps: ## Updates the dependency tree
	@echo Updating dependency tree...
	go mod tidy && go mod download
	@echo Updated successfully.

# Usage: `make build`
.PHONY: build
build: ## Builds `charted-server` into a binary in ./bin/charted-server
	@echo Now building charted-server for platform $(GOOS)/$(GOARCH)!
	go build -ldflags "-s -w -X noelware.org/charted/server/internal.Version=${VERSION} -X noelware.org/charted/server/internal.CommitSHA=${GIT_COMMIT} -x \"noelware.org/charted/server/internal.BuildDate=${BUILD_DATE}\"" -o ./bin/charted-server$(EXTENSION)
	@echo Successfully built charted-server! Use './bin/charted-server$(EXTENSION) -c ./config.toml' to run the server.

# Usage: `make clean`
.PHONY: clean
clean: ## Cleans any build artifacts
	@echo Cleaning build artifacts...
	rm -rf bin/ dist/
	go clean
	@echo Done!

# Usage: `make fmt`
fmt: ## Formats the project using `go fmt`.
	go fmt
