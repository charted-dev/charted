# 📦 charted-server
> *Free, open source, and reliable Helm Chart registry made in Kotlin!*

<!--
## Why?
**charted** (case-sensitive) is a way to distribute your Helm Charts onto the cloud safely and reliable without using an S3 bucket,
Google Cloud Storage bucket, or your local filesystem, it's centralized in one place.

**charted** is supposed to be like **Docker Hub**, where you can see the available versions of a certain Helm Chart,
and shows you what contains in the helm chart, its dependencies, and much more.

## Installation
**charted-server** can be installed in a few different ways, you can use:

- using the [Helm Chart](#helm-chart),
- using the [Docker Image](#docker-image),
- binary install with [GitHub Releases](#) or [dl.noelware.org/download.sh](https://dl.noelware.org) script,
- locally with Git.

### Prerequisites
If you're going to be running **charted-server**, this is a list software and SDKs that are used with **charted-server**:

- (optional) [**Elasticsearch**](https://elastic.co)/[**Meilisearch**](https://meilisearch.com)/[**Tsubasa**](https://github.com/auguwu/tsubasa)
- PostgreSQL 10 or higher
- Redis 5 or higher
- Go 1.18 or higher (applicable with Git installation; not needed in most installations)
- `protoc` and `protoc-gen-go` binaries installed on your system (applicable with Git installation; not needed in most installations)
- **2 CPU cores** or higher (applicable with Git installation; not needed in most installations)
- **2~6GB** or higher of system RAM available.
    - Note: This is only needed for the Git installation, **charted-server** only allocates a bit of memory to run the server
            the 2-6GB or higher requirement is for developer tooling (i.e, Visual Studio Code or GoLand)

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

**charted-server** only supports the use of Linux containers.

```shell
# 1. We must pull the image so we can later run it. Read the tag specification for more information
# about the `[tag]` suffix in this example. You can append `ghcr.io` in the image (i.e: `ghcr.io/charted-dev/charted:[tag]`)
# to use GitHub's Container Registry rather than Docker Hub.
$ docker pull noelware/charted-server:[tag]

# 2. Run the image!
$ docker run -d -p 12152:12152 -v ~/config.toml:/app/noelware/charted/server/config.toml noelware/charted-server:[tag]
```

#### Tag Specification
**charted-server** has different tags that are published when a new release of **charted-server** is published. You can specify:

- `latest` with the architecture to run the exporter as the suffix (i.e, `latest` -> `latest-amd64` (uses `linux/amd64`))
- **Minor**.**Major** with the architecture to run the exporter as the suffix (i.e, `1.0-arm64` (uses `linux/arm64`))
- **Minor**.**Major**.**Patch** with the architecture to run the exporter as the suffix (i.e, `1.0.2-arm7` (uses `linx/armv7`))

### Locally with Git
; ^ ~ coming soon ~ ^ ;

## Contributing
; ^ ~ coming soon ~ ^ ;

## Configuration
The configuration format is formatted as a `.toml` file! **charted-server** will try to find the configuration path in the following order:

- Under `CHARTED_CONFIG_PATH` environment variable;
- Under the root directory (`./config.toml`)

If none was found, **charted-server** will panic and fail to run. You can generate the configuration using the `generate` subcommand once you built **charted-server** with **make**.

```toml
# secret_key_base is the JWT signature to use when validating user sessions.
# If this variable isn't here, charted-server will generate one for you and
# save it to the file.
secret_key_base = "<server generated>"

# registrations returns a bool if user creation should be enabled. If this is disabled,
# server administrators are required to generate a user on the fly with the Administration
# Dashboard. (requires Pak to do so, or you can use Parcel with `parcel api generate:user`)
registrations = true

# Telemetry enables Noelware's telemetry service on the server with the `instance.uuid` file
# it generates on the fly. Read more here: https://telemetry.noelware.org
telemetry = false

# Analytics enables Noelware's analytical service which will open up a gRPC server so that Noelware Analytics
# can request server information. Read more here: https://analytics.noelware.org
analytics = false

# Metrics enables Prometheus metrics to be scraped if enabled.
metrics = false

# This is for basic authentication on the server that doesn't need the Sessions or API Key authentication
# methods. This is just the username to parse.
username = null

# This is for basic authentication on the server that doesn't need the Sessions or API Key authentication
# methods. This is just the password to parse.
password = null

# sentry_dsn enables Sentry on the server so you can get errors when any errors were reported.
sentry_dsn = null

# Port is the HTTP port to use when connecting to charted-server via HTTP. This can be superseded with
# the `PORT` environment variable.
port = 3939

# Host returns the host when connecting to charted-server via HTTP. This can be superseded with the
# `HOST` environment variable.
host = "0.0.0.0"

# `email` enables the Email service to do invites and email verification for new users.
# This is disabled by default.
[email]
# The password when doing Plain authentication. This is required.
password = ""

# Address is the SMTP address to send emails to, i.e, for Gmail:
#    stmp.gmail.com
address = ""

# `search` enables any search engine of your choice. We currently support Elasticsearch,
# Meilisearch, and Tsubasa (extended version of ES to do queries more efficient.)
[search]

# `storage` enables the storage driver of your choice. We currently support Amazon S3 and the filesystem.
# The server will notify you to keep a volume of it if running on Docker or if ran on Kubernetes,
# to have a PersistentVolumeClaim of it available.
[storage]

# `redis` is the Redis server to connect to for caching users (to ease off PSQL queries), ratelimits,
# sessions, stargazers/downloads count, and more. Sentinel and Standalone connections are supported.
[redis]
```
-->

## License
**charted-server** is released under the **Apache 2.0** License by [Noelware](https://noelware.org), you can read the full
license in the root repository [here](https://github.com/charted-dev/charted/blob/master/LICENSE).
