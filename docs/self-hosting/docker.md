---
title: Self-host charted-server with Docker
description: Curated guide to run charted-server with Docker.
---

**charted-server** comes with a [Docker image](https://cr.noelware.cloud/r/charted/server) that you can run on your own machine if
you wish to test out **charted-server**

## Prerequisites

- (optionally) Docker Compose
- Docker v20.10 or higher

## Docker Compose

You can use a Docker Compose file to start up the server. By default, if the server doesn't have a PostgreSQL connection, it will embed a PostgreSQL
server and stores in `/var/lib/noelware/db/data`. You can get an example compose file from the main repository:

```shell
$ curl -L -o docker-compose.yml https://raw.githubusercontent.com/charted-dev/charted/main/docker-compose.yml
```

Then, you can just run `docker-compose up` (or `docker compose up`) to start it up! You should be able to access the REST API with `http://localhost:3651`
and the web UI with `https://localhost:2134`.

## Docker

```shell
$ docker pull cr.noelware.cloud/charted/server:0.3-nightly
$ docker pull cr.noelware.cloud/charted/web:0.3-nightly

$ docker run -d -p 3651:3651 --name charted-server cr.noelware.cloud/charted/server:0.3-nightly
$ docker run -d -p 2134:2134 --name charted-web -e CHARTED_SERVER_URL=http://charted-server cr.noelware.cloud/charted/web:0.3-nightly
```
