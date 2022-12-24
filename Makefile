# 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 Noelware <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Usage: `make help`
.PHONY: help
help: ## Prints the help usage on the charted-server toolchain.
	@printf "\033[34m────────────────────────────────────────────────────────────────────────────────────────────\033[0m\n"
	@printf "This is the help command for building charted-server. To get started, run 'make run' to\n"
	@printf "begin the build process and run the API server."
	@printf "\n"
	@printf "\n:: Usage ::\n"
	@printf "make <target> [VARIABLE=value]\n"
	@printf "\n:: Targets ::\n"
	@awk 'BEGIN {FS = ":.*##"; } /^[a-zA-Z_-]+:.*?##/ { printf "  make \033[36m%-25s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 25) } ' $(MAKEFILE_LIST)

.PHONY: run
run: build ## Builds the project and runs the API server
	@./cli/build/install/charted/bin/charted server

.PHONY: build
build: spotless ## Runs the `spotless` target and builds the API server, CLI, and ClickHouse migrations
	@./gradlew :cli:installDist
	@chmod +x ./cli/build/install/charted/bin/charted

.PHONY: spotless
spotless: ## Runs the Spotless formatter on the project
	@./gradlew spotlessApply

.PHONY: clippy
clippy: ## Runs the `cargo clippy` command
	@(cd ./tools/helm-plugin && cargo clippy)

.PHONY: clippy-fix
clippy-fix: ## Runs `cargo clippy` with the --fix and --allow-dirty flags.
	@(cd ./tools/helm-plugin && cargo clippy --fix --allow-dirty)

## ( cd "$workdir" && somecommand )
.PHONY: clean
clean: ## Executes the `clean` Gradle task
	@./gradlew clean

.PHONY: test
test: clean ## Runs all the project tests
	@./gradlew test

# Not recommended but whatever
.PHONY: kill-gradle-daemons
kill-gradle-daemons: ## Kills all the Gradle daemons
	@pkill -f '.*GradleDaemon.*'
