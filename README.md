# 📦 charted-server
> *Free, open source, and reliable Helm Chart registry made in Kotlin!*

## What is this?
**charted-server** is the main backend of the charted project. It is a free, and reliable way to distribute Helm Charts without configuring
Helm to use a S3 bucket, your local disk, GCS, and more. It is centralized in one place!

**charted** as a whole is supposed to be a **Docker Hub** equivalent, where you can view different versions of the Helm Chart, and shows
what dependencies it uses, and more.

## Installation
**charted-server** can be installed in a few different ways! If you want a small, Rust version of **charted-server**, you might be 
interested in the Server Lite project that is developed on the side! The [lite edition](https://github.com/charted-dev/server-lite) is for
very small use cases but not limited to:

- Small configuration
- Overhead of CPU and memory usage
- Easy to contribute

but misses out on the features:

- Proper Helm Chart installation,
- Proper distribution and load balancing,
- Proper authorization system,
- Enterprise use cases like chart visualisation.

You can install the JVM version (which is recommended) from:

- the [Helm Chart](#helm-chart),
- the [Docker Image](#docker-image),
- binary install with [GitHub Releases](#binary-github-releases) or with [cURL](#binary-curl),
- locally with [Git](#git)
 
### Prerequisites
There is a list of other software that **charted-server** supports that you can use:

- (optional) Elasticsearch or Meilisearch -- Search backend,
- PostgreSQL 10 or higher,
- Redis 5 or higher
- Java 17,
- 2 CPU cores or higher (applicable with Git/local installation; not needed in Docker/Helm),
- **2-6**GB or higher on the system.
   - Note: This is only needed to build from source. The server only really uses ~512-780MB to run, it is only
           for developer tooling if contributing or building from source (which can take a while!)

### Helm Chart
### Helm Chart
Surprisingly, **charted-server** can be installed as a Helm Chart! Before you install **charted-server** on your Kubernetes
cluster, you will need **Kubernetes** >=1.22 and Helm 3 installed.

Since **charted-server** is distributed using the official server, you can easily grab the Noelware organization's repositories
for future installations of Noelware's products:

```shell
$ helm repo add noelware https://charts.noelware.org/~/noelware
```

Now you have indexed all of Noelware's repositories from the official server, you can install **charted-server**:

```shell
$ helm install <my-release> noelware/charted-server
```

This will bootstrap the server and the frontend UI available at [charted-dev/pak](https://github.com/charted-dev/pak). If you want
to see the official server's frontend source code, you can visit the [Noelware/charts.noelware.org](https://github.com/Noelware/charts.noelware.org)
repository.

### Docker Image
You can bootstrap **charted** using the Docker images hosted on [Docker Hub](https://hub.docker.com/r/noelware/charted-server) or
on the [GitHub Container Registry]().

**charted-server** only supports the use of Linux containers on x86_64 architectures.

```shell
# 1. We must pull the image so we can later run it. Read the tag specification for more information
# about the `[tag]` suffix in this example. You can append `ghcr.io` in the image (i.e: `ghcr.io/charted-dev/charted:[tag]`)
# to use GitHub's Container Registry rather than Docker Hub.
$ docker pull noelware/charted-server:[tag]

# 2. Run the image!
$ docker run -d -p 12152:12152 -v ~/config.toml:/app/noelware/charted/server/config.toml noelware/charted-server:[tag]
```

#### Tag Specification
#### Version Specification
**Noelware Analytics** supports a unofficial specification for versioning for Docker images. The versions can look like:

- **latest** | **latest-[arch]** | **latest-[arch][-os]**
- **[major].[minor]** | **[major].[minor][-arch]** | **[major].[minor][-arch][-os]**
- **[major].[minor].[patch]** | **[major].[minor].[patch][-arch]** | **[major].[minor].[patch][-arch][-os]**

| Image Version       | Acceptable | Why?                                                                            |
|---------------------|------------|---------------------------------------------------------------------------------|
| `latest`            | 💚         | defines as **linux/amd64** or **linux/arm64** with the latest release.          |
| `latest-amd64`      | 💚         | defines as **linux/amd64** with the latest release.                             |
| `latest-windows`    | 💚         | defines as **windows/amd64** with the latest release.                           |
| `0.2`               | 💚         | defines as **linux/amd64** or **linux/amd64** with the **0.2** release.         |
| `0.2-windows`       | 💚         | defines as **windows/amd64** with the **0.2** release.                          |
| `0.2-arm64`         | 💚         | defines as **linux/arm64** with the **0.2** release.                            |
| `latest-linux`      | ❤️         | Linux releases do not need a `-os` appended.                                    |
| `0.2-amd64-windows` | ❤️         | Windows releases do not need an architecture since it only uses **amd64** only. |
| `linux-amd64`       | ❤️         | What version do we need to run? We only know the OS and Architecture.           |
| `amd64`             | ❤️         | What version or operating system to run? We only know the architecture.         |

### Binary (GitHub Releases)
You can use [eget](https://github.com/zyedidia/eget) to get the tarball or ZIP version of **charted-server**:

```shell
$ eget charted-dev/charted
```

#### Binary (cURL)
You can use **cURL** to easily get an installation running very quickly:

```shell
$ curl -fsSl https://cdn.noelware.org/charted/server/install.sh | bash
```

## Contributing
Thanks for considering contributing to **charted-server**! Before you boop your heart out on your keyboard ✧ ─=≡Σ((( つ•̀ω•́)つ, we recommend you to do the following:

- Read the [Code of Conduct](./.github/CODE_OF_CONDUCT.md)
- Read the [Contributing Guide](./.github/CONTRIBUTING.md)

If you read both if you're a new time contributor, now you can do the following:

- [Fork me! ＊*♡( ⁎ᵕᴗᵕ⁎ ）](https://github.com/charted-dev/charted/fork)
- Clone your fork on your machine: `git clone https://github.com/your-username/charted`
- Create a new branch: `git checkout -b some-branch-name`
- BOOP THAT KEYBOARD!!!! ♡┉ˏ͛ (❛ 〰 ❛)ˊˎ┉♡
- Commit your changes onto your branch: `git commit -am "add features （｡>‿‿<｡ ）"`
- Push it to the fork you created: `git push -u origin some-branch-name`
- Submit a Pull Request and then cry! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡

## Configuration
**charted-server** can be configured using a TOML-like file. It will only support the `.toml` extension. It will try to find the configuration path
in the following order:

- Under the `CHARTED_CONFIG_PATH` environment path to a valid file,
- Under the root directory where the **charted-server** binary is.

If none was found, **charted-server** will panic and fail to run. In the future, it will generate one instead of just falling into
exceptions.

```toml
# soon? :eyes:
```

## License
**charted-server** is released under the **Apache 2.0** License with love (´｡• ᵕ •｡`) ♡ by [Noelware](https://noelware.org) 💜, you can read the full
license in the root repository [here](https://github.com/charted-dev/charted/blob/master/LICENSE).
