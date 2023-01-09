---
title: charted-server
description: You know, for Helm Charts?
---

# Noelware's Charts Platform

Hello viewer! This is the preliminary documentation for **charted-server** that ranges from the REST API, to self-hosting it,
and how to contribute to the source code!

**charted-server** is the backend API service for Noelware's Charts Platform. It is served to host Helm Charts and distribute it
all over the world with a web UI to show what this Helm Chart is and how it operates. Think of Noelware's Charts Platform as **Docker Hub**,
but for Helm Charts.

## Comparisons

### chartmusuem

When the development of the platform was being first developed, we had no idea the Helm developers created this! And we think it's a pretty
good server to host a bare-bones, feature-less version of **charted-server**. As **chartmusuem** is like Docker's [official registry](https://docs.docker.com/registry),
**charted-server** is meant to be like **Docker Hub**.

Some pros we have found that **chartmusemum** has a lot more storage handlers like Alibaba Cloud Storage, we only support the local filesystem and Amazon S3
(the official instance uses [Minio](https://min.io)!). If you wish to add more storage handlers, you can contribute to [Noelware's Remi library](https://github.com/Noelware/remi).

Another pro is that, **chartmusumem** supports [Helm Provenance](https://helm.sh/docs/topics/provenance) whilst **charted-server** does not support it, but plans to!

Some cons when using **chartmusuem** is:

- No "official server" to host your Helm Charts, you will need to install it on your own bare metal or in the cloud.
- No authorization system that does RBAC for the sake of simplicity
- It is very lightweight than **charted-server** since ChartMusuem is primarily made in **Go** while charted-server is built with Kotlin.

### Artifact Hub

As of late, **Artifact Hub** has been the defacto standard of hosting Helm Charts, but it is very different on how **charted-server** and Artifact Hub
are operated. While **charted-server** tends to be more configuration, less simple for the price of customizing to your liking, Artifact Hub is just a _hub_
to display information, you still need to use your own server to distribute Helm Charts. **charted-server** handles the storage and display information for you!

## Ready to try?

You can get started by creating a new [repository](https://charts.noelware.org/new) to get started! If you wish to look at what features
**charted-server** comes with, you can look in the Features section.

Wish to self-host **charted-server**? You can look in the [Self Hosting](https://charts.noelware.org/docs/server/self-hosting) section
and get started~!
