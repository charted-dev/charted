# 📦 charted-server
[![Kotlin v1.7.10](https://img.shields.io/badge/kotlin-1.7.10-blue.svg?logo=kotlin)](https://kotlinlang.org)
[![GitHub License](https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat)](http://www.apache.org/licenses/LICENSE-2.0)
[![Linting and Unit Testing](https://github.com/charted-dev/charted/actions/workflows/lint.yml/badge.svg)](https://github.com/charted-dev/charted/actions/workflows/lint.yml)
![](https://img.shields.io/github/languages/code-size/charted-dev/charted)

> *Free, open source, and reliable Helm Chart registry made in Kotlin!*

**charted-server** is a reliable, highly available, and free registry for distributing Helm Charts without configuring anything! All
you need is a Postgres and Redis database, and you're all set

## :warning: WARNING :warning:
**charted-server** is currently in alpha stages right now! Things can go wrong, you can report bugs and issues on our [GitHub issue tracker](https://github.com/charted-dev/charted/issues)!

This README used to be filled with information, but as we reached a stage that **charted-server** will be in Alpha, we removed the information
since it'll be outdated from alpha to release!

## Installation
At the moment, you can use the [Docker image](https://github.com/charted-dev/charted/pkgs/container/charted) on the GitHub Container Registry to get started. And,
this is currently alpha software and things can break!

You are required a PostgreSQL and Redis cluster before running the server.

```shell
$ docker pull ghcr.io/charted-dev/charted:nightly-amd64
$ docker run -d -p 3651:3651 -v $(pwd)/config.yml:/app/noelware/charted/server/charted.yml --name charted-server ghcr.io/charted-dev/charted:nightly-amd64
```

### Example `config.yml` file
```yml
storage:
  filesystem:
    directory: /var/lib/noelware/charted/server/data
```

## License
**charted-server** is released under the **Apache 2.0** License with love (´｡• ᵕ •｡`) ♡ by [Noelware](https://noelware.org) 💜, you can read the full
license in the root repository [here](https://github.com/charted-dev/charted/blob/master/LICENSE).
