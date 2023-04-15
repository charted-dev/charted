# 🐻‍❄️📦 Web UI for [charted-server](https://github.com/charted-dev/charted)

> _Web interface for interacting with [charted-server](https://github.com/charted-dev/charted), made with [Vue](https://vuejs.org)_

**charted-web** is a beautiful and crafted web interface that allows you to interact directly with
[charted-server](https://github.com/charted-dev/charted) without having to write complex cURL commands
to achieve what you want.

**charted-web** always will be compatible with the latest major and minor releases that
[charted-server](https://github.com/charted-dev/charted) releases, it has the same release cycle of using
SemVer 2 for indicating release impact. It is recommended to run **charted-web** with the same version as
**charted-server** to have better compatibility.

| API Server Version | Web UI Version | Compatible? Why?                                                                                                   |
| ------------------ | -------------- | ------------------------------------------------------------------------------------------------------------------ |
| `v0.4-nightly`     | `v0.4-nightly` | Yes! It matches with the same version of the API server.                                                           |
| `v1.2.3`           | `v2.3.0`       | No since **charted-server** has a lower version than **charted-web**.                                              |
| `v1.2.3`           | `v1.3.4`       | **charted-web** will run but will warn that the **charted-server** version is lower than itself.                   |
| `v1.2.3`           | `v1.2.4`       | **charted-web** will run but will not indicate any warnings since patch versions do not include major differences. |
| `v2.3.1`           | `v1.2.4`       | **charted-web** will not run due to the **charted-server** version being higher than **charted-web**.              |
| `v2.3.1`           | `v2.2.1`       | **charted-web** will run but will warn that the **charted-server** version is higher than itself.                  |
| `v2.3.2`           | `v2.3.1`       | Yes! Even though that **charted-web** has a lower patch version, it doesn't matter if it is compatible or not.     |

Do note that any version that uses the nightly branch and the other is using a stable release will not be acceptable since the Nightly pipeline is unstable than the stable builds. You can read up on how Noelware built the foundation of the **Stable**, **Nightly**, and **Unstable** channels.

## Installation

### Locally with Git

To build and run **charted-web**, you will need to meet the minimum requirements:

-   Instance of [charted-server](https://github.com/charted-dev/charted) running that it can access
-   [Bazel](https://bazel.build) build tool
-   [Git](https://git-scm.com) for cloning the repository to the host machine

To get started, clone the repository:

```sh
$ git clone https://github.com/charted-dev/web
```

To easily update the repository (you will need to re-build your changes everytime), add a upstream remote that can be easily fetched:

```sh
$ git remote add upstream https://github.com/charted-dev/web
```

Since **charted-web** is a monorepo built with [Bazel](https://bazel.build), you will need to install it. You can easily install it if you have `npm` installed:

```sh
$ npm i -g @bazel/bazelisk
```

Otherwise, you can use the [Bazelisk](https://github.com/bazelbuild/bazelisk) command line tool to get a version of Bazel that we use.

To build all the packages into a distribution you can run, use the `bazel build //:distribution` command to build a distribution for your specific environment, it will be in `dist/`.

To run the newly built distribution, you can use the `bazel run //:distribution` or `./dist/charted-web` to launch up the web server.

### Artifacts Registry

We also have an option to run a "installer" of **charted-web** to pull and run the web server without you building it yourself. There is also a contained **tar** and **ZIP** file that is pub
lished on every release if you wish to do it yourself.

```shell
# Linux, macOS with cURL
$ curl -fsSL https://artifacts.noelware.cloud/charted/web/latest/install.sh | bash -
```

```powershell
# Windows
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser # needed to run this the first time
irm https://artifacts.noelware.cloud/charted/web/latest/install.ps1 | iex
```

To run **charted-web** in a Docker environment, we will need to create a Docker network to allow requests to **charted-server**.
To set up a Docker network, you can run the `docker network` command:

```sh
$ docker network create charted --driver bridge
```

Since we need a **charted-server** instance, you will need to
[pull and run the server Docker image](https://cr.noelware.cloud/~/charted/server). Since the web UI
doesn't require persistence, we don't need to configure volumes.

The images are hosted on [Noelware's Container Registry](https://cr.noelware.cloud/~/charted) and on
[GitHub's Container Registry](https://github.com/orgs/charted-dev/packages) as a second alternative if **cr.noelware.cloud**
is ever up for maintenance or is down. Only GitHub's container registry has all the nightly AND stable builds,
so we do not clutter up Noelware's container registry with nightly and stable releases.

```shell
$ docker pull ghcr.io/charted-dev/web
$ docker run -d -p 2134:2134 --name charted-web --network charted \
  -e CHARTED_SERVER_HOST="http://charted-server:3651"             \
  ghcr.io/charted-dev/web
```

### Helm Chart

**charted-web** can also be used from **charted-server**'s Helm chart if you're deploying only one Helm Chart in your
Kubernetes cluster. To do so, you can set this in your `values.yaml`:

```yaml
web.enabled: true
web.ingress.enabled: true
web.ingress.hostname: http://charts.example.org
```

Otherwise, you can run the web interface as a standalone Helm Chart since **charted-server** has a optional dependency on it.
You can do so with:

```shell
$ helm repo add charted https://charts.noelware.org/~/charted
$ helm install charted-web charted/web --set ingress.enabled=true --set ingress.hostname=http://charts.example.org
```

## Configuration

## Contributing

## License

**charted-web** is released under the **Apache 2.0** License with love by [Noelware, LLC.](https://noelware.org).
You can read the LICENSE file that is [attached in this repository](https://github.com/charted-dev/charted/blob/main/LICENSE)
