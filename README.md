# 🐻‍❄️📦 charted-server
> *Free, open sourced, and reliable [Helm](https://helm.sh) chart registry in [Rust](https://rustlang.org)*
>
> [<kbd>v0.1-beta</kbd>](https://github.com/charted-dev/charted/releases/0.1.0-beta) **|** [:scroll: **Documentation**](#)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=469212491&machine=standardLinux32gb&devcontainer_path=.devcontainer%2Fdevcontainer.json&location=WestUs2)

**charted-server** is a free and open platform to help you host, test, and build efficient Helm charts that allow you to use Helm efficiently in a single-user or multi-organization instance.

**charted-server** is also the first product released by [Noelware, LLC.](https://noelware.org). We use this same repository to host
[our official instance](https://charts.noelware.org)!

## Why was it rewritten in Rust and not kept in Kotlin?
As the project grew, compilation times and issues with [Gradle](https://gradle.org) arose, and the team didn't want to spend days or weeks trying to fix why our builds weren't being compiled correctly.

We also migrated our web interface, Helm plugin, and Rust SDK into this repository to keep one source of truth available. Our separate microservices and misc. projects will stay in the organization and won't be merged into this one.

## Getting started
As **charted-server** is being prepared for a beta release, we do not have an official "getting started" guide yet.

## Installation
As **charted-server** is being prepared for a beta release, we do not have official installations yet!

## License
**charted-server** is released under the **Apache 2.0** License with love by Noelware, LLC.! If you wish to know more,
you can read the [LICENSE](./LICENSE) file for more information
