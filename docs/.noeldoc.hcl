# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
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

registry "default" {
    version = 1
    url     = "https://registry.noeldoc.dev"
}

noeldoc {
    version = ">=0.1-beta"
    experimental {
        # Enables sandboxing where modules and renderers can be permitted
        # from doing anything else, unless explicitly told to.
        sandboxing = true

        # Enables modules and renderers to read files from the filesystem.
        permissions = ["+filesystem:read"]
    }
}

project "charted-server" {
    description = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust"
    version     = "${noeldoc.filesystem.read("../.charted-version")}"

    module {
        registry = "default"
        id       = "dev.noeldoc.modules/markdown"
        opts     = {
            "validate" = true,
            "tree"     = noeldoc.glob(["src/**/*.md"]),
            "mdx"      = false
        }
    }

    renderer {
        registry = "default"
        modules  = ["dev.noeldoc.modules/markdown"]
        id       = "dev.noeldoc.renderers/markdown"
    }
}
