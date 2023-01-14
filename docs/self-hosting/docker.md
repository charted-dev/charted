---
title: Installing charted-server on Docker
description: Guide on how to run charted-server on Docker
---

To install **charted-server** with Docker, you will need to have the [Docker Engine](https://docker.com) installed on your machine. Once you have Docker
installed, you can pull the images from GitHub or Noelware's Container Registry, it depends on what you preferably recommend to use.

- If you wish to only run stable builds, you can use [Noelware's Container Registry](https://cr.noelware.cloud)
- If you're willingly wanting to run the cutting edge version of **charted-server**, you can do so with the nightly channel. The nightly channel's
  releases are only on [GitHub's Container Registry](https://github.com/orgs/charted-dev/packages).

The image can consist around multiple tags that are suited for your environment. **charted-server**'s images are built with
the `linux/amd64` and `linux/arm64` architectures.

- `latest`, `nightly` - The latest versions for each channel (`latest` for the **stable** channel, `nightly` for the **nightly** channel)
- `alpine` - This tag runs **charted-server** with the [Alpine](https://hub.docker.com/_/alpine) image instead of [Ubuntu](https://hub.docker.com/_/ubuntu),
  which is recommended for production environments since it's more compat and smaller.
- `{version}`, `{version}-nightly` - The **{version}** placeholder is for any specific version of **charted-server** to run, which you can view
  all the releases on [GitHub](https://github.com/charted-dev/charted/releases). The `-nightly` prefix is for users who want to preview upcoming features
  or to use a specific nightly version.
- `{version}-alpine` - Similarly to the stock `alpine` image tag, but uses a specific version of **charted-server** to run.

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

Now that a volume for **charted-server** exists, now we can pull the image from [Noelware's Container Registry](https://cr.noelware.cloud) or
[GitHub's Container Registry](https://github.com/orgs/charted-dev/packages).

```shell
# Noelware's Container Registry
$ docker pull cr.noelware.cloud/charted/server:{{ .Project.Version }}

# GitHub's Container Registry
$ docker pull ghcr.io/charted-dev/charted:{{ .Project.Version }}
```

Now, we can run the container with the following command:

```shell
# Noelware's Container Registry
$ docker run -d -p 3651:3651 --name charted-server \
  -v charted:/var/lib/noelware/charted/data \
  cr.noelware.cloud/charted/server:{{ .Project.Version }}

# GitHub's Container Registry
$ docker run -d -p 3651:3651 --name charted-server \
  -v charted:/var/lib/noelware/charted/data \
  ghcr.io/charted-dev/charted:{{ .Project.Version }}
```

## Docker Compose

If you wish to install **charted-server** with a Docker Compose deployment, you can easily fetch the `docker-compose.yml`
file in the root repository and use `docker compose up -d` and it'll run with the necessary components required.

```shell
# Linux/macOS with cURL
$ curl -Lo docker-compose.yml https://raw.githubusercontent.com/charted-dev/charted/{{ .Project.Version }}/docker-compose.yml

# Windows
$ irm https://raw.githubusercontent.com/charted-dev/charted/{{ .Project.Version }}/docker-compose.yml | Set-Content -Path ./docker-compose.yml
```
