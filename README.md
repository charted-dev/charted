 📦 Noelware's Charts Platform (charted-server)
[![Kotlin v1.7.22](https://img.shields.io/badge/kotlin-1.7.22-blue.svg?logo=kotlin)](https://kotlinlang.org)
[![GitHub License](https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat)](http://www.apache.org/licenses/LICENSE-2.0)
[![Linting and Unit Testing](https://github.com/charted-dev/charted/actions/workflows/Linting.yaml/badge.svg?branch=main)](https://github.com/charted-dev/charted/actions/workflows/Linting.yaml)
![](https://img.shields.io/github/languages/code-size/charted-dev/charted)

> *Free, open source, and reliable Helm Chart registry made in Kotlin!*
>
> [<kbd>v0.3.3-nightly</kbd>](https://github.com/charted-dev/charted/releases/0.3.3-nightly) | [:scroll: **Documentation**](#)

**charted-server** is the meat and potatoes of Noelware's Charts Platform. It is meant to be a free Helm Chart registry to distribute your Helm Charts
too much people as possible! As this is free open sourced software, you can create your own forks and features as you wish!

> **Warning**
> 
> Before we continue, I want you (the viewer) to know that this is PURE Alpha software! Be cautious using this
> in production while we (Noelware) work with the community to make our charts platform better~!

## Installation
### Docker
**charted-server**'s Docker images are available on [Noelware's Container Registry](https://cr.noelware.cloud) and on [GitHub's Container Registry](https://github.com/orgs/charted-dev/packages). The recommended
Docker engine version is **20.10** or higher.

The images support x86_64 and ARM cpus, but the ARM dockerfile uses Ubuntu while the x86_64 dockerfile uses Alpine.

```shell
# 1. Create a Docker volume. By default, the server will use the local filesystem to store tarballs and chart indexes.
$ docker volume create charted

# 2. Pull the image from Noelware or GitHub's Container Registry. Noelware's container registry only publishes stable versions while
# GitHub's container registry pushes nightly and stable releases.
$ docker pull ghcr.io/charted-dev/charted

# 3. Run the image!
$ docker run -d -p 3651:3651 --name charted-server \
  -v charted:/var/lib/noelware/charted/data \
  ghcr.io/charted-dev/charted
```

## License
**Noelware's Charts Platform** (charted-server) is released under the **Apache 2.0** License with love by Noelware~! If you wish to know more,
you can read the [LICENSE](./LICENSE) file for more information. 
