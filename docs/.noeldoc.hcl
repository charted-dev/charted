# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

noeldoc {
    version = "~> 0.1-beta"
}

registry "default" {
    version = 1
    url     = "https://registry.noeldoc.dev"
}

project "charted-server" {
    description = "🐻‍❄️📦 Free, open source, and reliable Helm chart registry in Rust"
    homepage    = "https://charts.noelware.org/docs/server"
    base_url    = "https://charts.noelware.org/docs/server"
    version     = "${readfile("${cwd}/.charted-version")}"

    github {
        repo = "charted-dev/charted"
    }

    versioning {
        master = ingitbranch("main")
    }

    extension "markdown" {
        registry = "default"
        id = "extensions/markdown"
        options {
            validate = true
            srcs = glob(["${cwd}/docs/src/**/*.md"])
            mdx = false
        }
    }

    extension "openapi" {
        registry = "default"
        id = "openapi"
        options {
            json_file = "${cwd}/assets/openapi.json"
        }
    }

    renderer "markdown" {
        registry = "default"
        extensions = [extension.markdown, extension.openapi]
        options {
            validate = true
        }
    }
}
