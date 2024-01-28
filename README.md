# 🐻‍❄️📦 charted-server

> _Free, open sourced, and reliable [Helm](https://helm.sh) chart registry in [Rust](https://rustlang.org)_
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

-   [protoc](https://protobuf.dev)
-   [Rust](https://rust-lang.org)
-   [Git](https://git-scm.com)
-   20GB of storage
-   2GB of system RAM

To clone the repository, you can use the `git pull` command:

```shell
# HTTPS
$ git pull https://github.com/charted-dev/charted

# SSH
$ git pull git@github.com:charted-dev/charted
```

Once you cloned the repository, you can `cd` into it and run:

```shell
$ ./dev cli --release
```

This will build the charted CLI in release mode, to run it, you can use `--run` (but it'll produce a debug build):

```shell
$ ./dev cli --run
```

This will run the actual CLI, to run the server, you will need to use `./dev server` instead:

```shell
$ ./dev server
```

## FAQ

### :question: Can I use `cargo install` from the Git repository?

Yes! To do so, you can use the following commands:

-   [charted CLI](https://charts.noelware.org/docs/cli/latest): `cargo install https://github.com/charted-dev/charted`
-   [charted Helm plugin](https://charts.noelware.org/docs/helm-plugin/latest): `cargo install https://github.com/charted-dev/charted charted-helm-plugin`

We don't recommend installing `charted-devtools` as it is an internal tool.

## License

**charted-server** is released under the **Apache 2.0** License with love by Noelware, LLC.! If you wish to know more,
you can read the [LICENSE](./LICENSE) file for more information
