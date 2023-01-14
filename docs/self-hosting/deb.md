---
title: Installing charted-server on Debian-based systems
description: Guide on running charted-server on Debian-based systems.
---

**charted-server** is easily distributed through [Noelware's Artifacts Registry](https://artifacts.noelware.cloud). To install
**charted-server** on your system, you will need to follow the following prerequisites:

- You will need a **PostgreSQL** installation running. In future releases, charted-server will use an embedded PostgreSQL server for development and trial
  purposes.
- You will need a **Redis** installation running. In future releases, charted-server will use an embedded Redis server for development and trial purposes.
- You will need **JDK 17** installed on your system.
- You will need ~512MB of system RAM and ~2GB of storage

First, we need to update APT's package index and install libraries that will help aid installing **charted-server** from Noelware's
repositories:

```shell
$ sudo apt update
$ sudo apt install -y ca-certificates curl gnupg lsb-release
```

Then, we will need to add Noelware's official GPG key, so you know that we are hosting the content, not any malicious actor:

```shell
$ sudo mkdir -p /etc/apt/keyrings
$ curl -fsSL https://artifacts.noelware.cloud/debian/gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/noelware.gpg
```

Now, we can add Noelware's repositories into the system's repository list, which can be viewed from `/etc/apt/sources.list.d` or from
the `/etc/apt/sources.list` file.

```shell
$ echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/noelware.gpg] https://artifacts.noelware.cloud/debian \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/noelware.list
```

Now we just need to refresh `apt`'s package index and install **charted-server**:

```shell
$ sudo apt update
$ sudo apt install -y charted-server
```

To verify that it has been installed on your system, you can run the cURL command to see if it is working:

```shell
$ curl http://localhost:3651
{"success":true,"data":{"message":"Hello, world!","tagline":"You know, for Helm Charts?","docs_uri":"https://charts.noelware.org/docs/server/{{ .Project.Version }}"}}
```
