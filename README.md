# 📦 charted-server
> *Free, open source, and reliable Helm Chart registry made in Go.*

## Why?
**charted** (case-sensitive) is a way to distribute your Helm Charts onto the cloud safely and reliable without using an S3 bucket,
Google Cloud Storage bucket, or your local filesystem, it's centralized in one place.

**charted** is supposed to be like **Docker Hub**, where you can see the available versions of a certain Helm Chart,
and shows you what contains in the helm chart, its dependencies, and much more.

## Installation
**charted** is meant to be used on the [charts.noelware.org](https://charts.noelware.org) server, but you can bootstrap
your own instance if you wish! Before you do, we recommend having the following specs on your machine or the server
you're running it on:

- **6GB** or higher of system RAM available.
    - Note: Running **Charted** only allocates **2GB**, the rest is for developer tooling if you are
      contributing, if not, only **2GB** or higher is recommended.
- **2 CPU Cores** or higher.
- **Go 1.18** or higher installed
    - Note: This is only if you're planning to contribute or run it using the Git repository. The Docker image and
            Helm Chart don't use the Go SDK once ran.

### Helm Chart
Surprisingly, **charted** can be installed as a Helm Chart! Before you install **Charted** on your Kubernetes
cluster, you will need **Kubernetes** >=1.22 and Helm 3 installed.

Since **charted** is distributed using the official server, you can easily grab the Noelware organization's repositories
for future installations of Noelware's products:

```shell
$ helm repo add noelware https://charts.noelware.org/~/noelware
```

Now you have indexed all of Noelware's repositories from the official server, you can install **Charted**:

```shell
$ helm install <my-release> noelware/charted-server
```

This will bootstrap the server and the frontend UI available at [charted-dev/pak](https://github.com/charted-dev/pak). If you want
to see the official server's frontend source code, you can visit the [Noelware/charts.noelware.org](https://github.com/Noelware/charts.noelware.org)
repository.

### Docker Image
You can bootstrap **charted** using the Docker images hosted on [Docker Hub](https://hub.docker.com/r/noelware/charted-server) or
on the [GitHub Container Registry]().

### Locally with Git
; ^ ~ coming soon ~ ^ ;

## Contributing
; ^ ~ coming soon ~ ^ ;

## Configuration
; ^ ~ coming soon ~ ^ ;

## License
**charted-server** is released under the **Apache 2.0** License by [Noelware](https://noelware.org), you can read the full
license in the root repository [here](https://github.com/charted-dev/charted/blob/master/LICENSE).
