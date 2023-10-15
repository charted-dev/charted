# 🐻‍❄️📦 Contributing to charted-server
Thanks for considering to help build and make **charted-server** even better! We full heartily accept contributions from the community — you! We will accept any contributing from any major feature to small, grammatical bugs.

## Bug Reporting
Think you found an issue that you ran into? To submit a bug report, please use the latest release of the server because it might've been fixed by us, but if it hasn't then, you can surf through the [issue board](https://github.com/charted-dev/charted/issues) if the issue was already been reported, it'll be tagged with the `bug` label and be added onto Noelware's internal issue board.

- Be clear and concise with the title and description of the bug, it will help others link their issues & solutions to yours!
- Specify any way to reproduce the bug, so we know what to fix.

To test REST API-related issues, we recommend using `cURL` so it can be easier to reproduce on the latest version.

- Use number-formatted prefixes (i.e, `1. {step}`) to determine a step.
- Prefix the expected result with `#>`, and the actual result with `#?>`

Example:

```shell
# 1. Create a repository
$ curl -XPUT -H "Content-Type: application/json" -d '{"name":"repo1"}' http://localhost:3651/users/@me/repositories
#> {"success": true}

# 2. Invite a member.
$ curl -XPOST -H "Content-Type: application/json" -d '{"user_id":1234}' http://localhost:3651/repositories/1/members/invite
#> {"success": true}

# 3. Once they were invited, check if they can be queried.
$ curl http://localhost:3651/repositories/1/members
#> {"success": true,"data":[{...}]}

# 4. Kick them!
$ curl -XDELETE -H "Content-Type: application/json" http://localhost:3651/repositories/1/members/1234/kick
#> {"success":true}
#?> {"success":false,"errors":[{...}]}
```

## Security Vulnerabilities
If you found any security vulnerabilities when using **charted-server**, please refer to our [Security Policy](https://github.com/charted-dev/charted/blob/master/SECURITY.md).

## Code Contributions
We alweays accept code contributions since your contributions to anything related to the project makes it more powerful and secure than our team can if we just kept this closed sourced, since it is based off community feedback!

This repository is a monorepo, so the codebase might be intimidating, but this guide is here to help you aid in how we structured the project and such.

> **Note**: We do support using [GitHub Codespaces](https://github.com/codespaces) or [Coder](https://coder.com) (with [Noel's Coder templates](https://github.com/auguwu/coder-images) -- it is recommended since it'll run all preinit scripts and the Docker compose project in `.coder/docker-compose.yml`)
>
> Both Codespaces and Coder have the necessary tooling to help you build and run charted-server easily from a remote environment!

### How is the project structured?
The project is a monorepo that is structured into multiple folders:

* `build/` is meant to add [Bazel macros](https://bazel.build/extending/macros).
* `cli/` is the actual CLI source code.
* `crates/` is the different crates that is used through-out the `cli` and `server` folders.
* `distribution/` is related to how **charted-server** is distributed once a release is settled.
* `scripts/` is any script that helps to not write long commands, or anything really.
* `server/` is the actual REST API.
* `tools/` is tools and services that help aid the `server/` folder.
* `web/` is the actual web interface that is packaged with the server!

Originally, **charted-server** was written in Kotlin, which made it impossible to include both the `web/` and `server/` together without magic with Gradle, Rust and Bazel helps us build a monorepo that brings in what **charted-server** brings to the table without trying to separate it between repositories and make it harder on the team.

## FAQ
### :question: How can I use a different Rust channel?
You can use the `--@rules_rust//rust/toolchain/channel` setting when you run `bazel build`, or you can add it in your .user.bazelrc to persist on builds:

```shell
# we only support stable and nightly
build --@rules_rust//rust/toolchain/channel=nightly
```

### :question: How can I contribute on NixOS?
Since Rust toolchains are resolved with a remote source and are unpatched, you can add this to your .user.bazelrc file:

```shell
# signalify that OpenSSL is compiled statically and will use the nixpkgs version, which is patched.
build --//build/settings:nixos

# recommended by rules_nixpkgs
build --host_platform=@rules_nixpkgs_core//platforms:host

# use the nixpkgs_config_cc toolchain
build --crosstool_top=@nixpkgs_config_cc//:toolchain
```
