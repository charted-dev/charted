# 🐻‍❄️📦 charted-server
> *Free, open sourced, and reliable [Helm](https://helm.sh) chart registry in [Rust](https://rustlang.org)*
>
> [<kbd>v0.1-beta</kbd>](https://github.com/charted-dev/charted/releases/0.1.0-beta) **|** [:scroll: **Documentation**](#)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=469212491&machine=standardLinux32gb&devcontainer_path=.devcontainer%2Fdevcontainer.json&location=WestUs2)

**charted-server** is a free and open platform to help you host, test, and build efficient Helm charts that allow you to use Helm efficiently in a single-user or multi-organization instance.

**charted-server** is also the first product released by [Noelware, LLC.](https://noelware.org). We use this same repository to host
[our official instance](https://charts.noelware.org)!

## Getting started
As **charted-server** is being prepared for a beta release, we do not have an official "getting started" guide yet.

## Installation
### Locally from source
To build charted-server from the canonical Git repository, you are required to have the following tools:

* [Bazel](https://bazel.build)
* [Git](https://git-scm.com)
* 2GB of system RAM
* 4GB of storage

To clone the repository, you can use the `git pull` command:

```shell
# HTTPS
$ git pull https://github.com/charted-dev/charted

# SSH
$ git pull git@github.com:charted-dev/charted
```

Once you cloned the repository, you can `cd` into it and run `bazel build //cli:release_binary`:

```shell
$ bazel build //cli:release_binary
```

This will build the charted CLI in release mode, to run it, you can use `run` instead of `build`:

```shell
$ bazel run //cli:release_binary
```

This will run the actual CLI, to run the server, you will need to append `-- server --config=$PWD/config.yml`:

```shell
$ bazel run //cli:release_binary -- server --config=$PWD/config.yml
```

> **Note**: The `--config=$PWD/config.yml` is required when you invoke Bazel by itself since it'll run it in
> the sandbox if you're on Linux or macOS.

## FAQ
### :question: Why Bazel? Couldn't you done this with `node`, `cargo`, and `go` together?
**Bazel** is a build tool by Google to provide fast and correct builds, and can handle multiple languages in the same workspace. While it is possible to use `node` (Node.js), `cargo` (Rust), and `go` (Golang) together, we don't recommend it as we are tailouring the workflow to Bazel only.

### :question: Can I use `cargo install` from the Git repository?
Yes, but soon we plan to drop Cargo support since Cargo is only available for [rust-analyzer](https://github.com/rust-lang/rust-analyzer) and IDE support.

## License
**charted-server** is released under the **Apache 2.0** License with love by Noelware, LLC.! If you wish to know more,
you can read the [LICENSE](./LICENSE) file for more information
