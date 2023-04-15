# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

# https://stackoverflow.com/a/14061796
# we only do this with the `charted` or `web` recipes.
ifeq (charted, $(firstword $(MAKECMDGOALS)))
  runargs := $(wordlist 2, $(words $(MAKECMDGOALS)), $(MAKECMDGOALS))
  $(eval $(runargs):;@true)
endif

ifeq (web, $(firstword $(MAKECMDGOALS)))
  runargs := $(wordlist 2, $(words $(MAKECMDGOALS)), $(MAKECMDGOALS))
  $(eval $(runargs):;@true)
endif

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

.PHONY: charted
charted: ## Runs a command from the charted CLI.
	@./cli/build/install/charted/bin/charted $(runargs)

.PHONY: run
run: build ## Builds the project and runs the API server
	@make charted server

.PHONY: build
build: spotless ## Runs the `spotless` target and builds the API server
	@./gradlew :cli:installDist
	@chmod +x ./cli/build/install/charted/bin/charted

.PHONY: spotless
spotless: ## Runs the Spotless formatter on the project
	@./gradlew :buildSrc:spotlessApply
	@./gradlew spotlessApply

.PHONY: clean
clean: ## Executes the `clean` Gradle task
	@./gradlew clean

.PHONY: test
test: spotless ## Runs all the project tests
	@./gradlew test

# Not recommended but whatever
.PHONY: kill-gradle-daemons
kill-gradle-daemons: ## Kills all the Gradle daemons
	@pkill -f '.*GradleDaemon.*'

.PHONY: build-web
build-web: fmt eslint ## Builds `charted-web`. Required Node.js to be installed
	@(cd web && pnpm run build)
	@chmod +x ./web/dist/bin/charted-web

.PHONY: web
web:
	@(cd web && ./dist/bin/charted-web $(runargs))

.PHONY: eslint
eslint: ## Runs `eslint` in web/
	@(cd web && pnpm run lint)

.PHONY: fmt
fmt: ## Runs `prettier` in web/
	@(cd web && pnpm run fmt)
