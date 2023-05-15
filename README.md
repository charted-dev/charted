# 🐻‍❄️📦 charted-server

[![Kotlin v1.8.21](https://img.shields.io/badge/kotlin-1.8.21-blue.svg?logo=kotlin)](https://kotlinlang.org)
[![GitHub License](https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat)](http://www.apache.org/licenses/LICENSE-2.0)
[![Linting and Unit Testing](https://github.com/charted-dev/charted/actions/workflows/Linting.yaml/badge.svg?branch=main)](https://github.com/charted-dev/charted/actions/workflows/Linting.yaml)
[![ktlint](https://img.shields.io/badge/code%20style-%E2%9D%A4-FF4081.svg)](https://ktlint.github.io/)
![](https://img.shields.io/github/languages/code-size/charted-dev/charted)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=469212491&machine=standardLinux32gb&devcontainer_path=.devcontainer%2Fdevcontainer.json&location=WestUs2)

> _Free, open source, and reliable Helm Chart registry made in Kotlin!_
>
> [<kbd>v0.3.2-nightly</kbd>](https://github.com/charted-dev/charted/releases/0.3.2-nightly) | [:scroll: **Documentation**](#)

**charted-server** is a free, open source, and reliable [Helm](https://helm.sh) chart registry made with [Kotlin](https://kotlinlang.org)! It
is meant to be a very easy solution to distribute Helm charts on the cloud without having to configure most things, that it just works.

**charted-server** is also Noelware's first official product that we host our [official instance](https://charts.noelware.org)!

## Installation

### Docker

To install **charted-server** with Docker, you will need to have the [Docker Engine](https://docker.com) installed on your machine.
Once you have Docker installed, you can pull the images from Noelware's Container Registry, it depends on what you preferably
recommend to use.

The image can consist around multiple tags that are suited for your environment. **charted-server**'s images are built with
the `linux/amd64` and `linux/arm64` architectures.

-   `latest`, `nightly`, `unstable` - The latest versions for each channel (`latest` for the **stable** channel, `nightly` for the **nightly** channel, `unstable` for the **unstable** channel)
-   `alpine` - This tag runs **charted-server** with the [Alpine](https://hub.docker.com/_/alpine) image instead of [Ubuntu](https://hub.docker.com/_/ubuntu), which is recommended for production environments since it's more compact and smaller.
-   `{version}`, `{version}-nightly` - The **{version}** placeholder is for any specific version of **charted-server** to run, which you can view all the releases on [GitHub](https://github.com/charted-dev/charted/releases). The `-nightly` prefix is for users who want to preview upcoming features or to use a specific nightly version.
-   `{version}-alpine` - Similarly to the stock `alpine` image tag, but uses a specific version of **charted-server** to run.

Before we run **charted-server** on Docker, we will need to create a volume with the `docker volume` subcommand, or you can
use a regular filesystem mount (i.e, `~/.local/containers/charted-server`).

**charted-server** holds persistence between your Helm chart indexes, releases, and user-registered avatars. It is not recommended to
not persist your data, only if you're not going to use **charted-server** in the long-term!

You can create a volume with `docker volume create` as shown below:

```shell
$ docker volume create charted
```

Or mount it with a directory on your filesystem.

> **Note**: You can substitute `charted` with any volume name, but you will have to change `charted` to the volume
> name in later examples if you went with creating a volume with `docker volume`.
>
> For regular filesystem-mounted directories, you will need to change the ownership of the directory so
> the server can read & write to it. You can use the `chown` command to do so:
>
> ```shell
> $ chown -R 1001:1001 <directory>
> ```

Now that a volume for **charted-server** exists, now we can pull the image from [Noelware's Container Registry](https://cr.noelware.cloud):

```shell
$ docker pull cr.noelware.cloud/charted/server:latest
```

Now, we can run the container with the following command:

```shell
$ docker run -d -p 3651:3651 --name charted-server \
  -v charted:/var/lib/noelware/charted/data \
  cr.noelware.cloud/charted/server:latest
```

## License

**charted-server** is released under the **Apache 2.0** License with love by Noelware~! If you wish to know more,
you can read the [LICENSE](./LICENSE) file for more information.
