---
title: charted-server
description: You know, for Helm Charts?

---

# 📦 charted-server
Welcome to the main documentation hub of all the topics relating to the heart and soul of Noelware's Charts platform: **charted-server**

**charted-server** (case-sensitive) is the heart and soul of Noelware's Charts platform. It's an in-cloud solution to host your
Helm Charts with relative ease with similarities like **Docker Hub** or **GitHub**

## Comparisons
### chartmuseum
To be honest, we didn't know chart-museum existed until a few months in development of Noelware's Charts platform. And, it's
a pretty good server for hosting public and private Helm Charts, right?

chartmuseum was built to be in similar with Docker's [official registry](https://docs.docker.com/registry) but misses out on:

- An in-cloud solution to host your charts if you do not want to set up your own registry.
- No authorization system, it's all "public" so anyone can read, but you will create a user account like Docker Registry.
- If you really care that **charted-server** is primarily built in Kotlin/JVM, then you might like **chartmusuem** more.

The only pro we found is that it supports way more providers to host your charts like Google Cloud Storage, Microsoft Azure
Blob Storage, Alibaba Cloud OSS Storage, and more. **charted-server** only supports Amazon S3 and the local filesystem, since
we use Noelware's [remi](https://remi.noelware.org) library to handle storage, so if you wish to contribute and add a storage provider
to the main repository, then do so!

**charted-server** also doesn't support Helm's `--verify` flag when verifying the Helm Chart tarball.

### Artifact Hub
**charted-server** and Artifact Hub are way different when how it is operated. Artifact Hub hasn't been a real solution to host
your charts on the cloud, you still have to host them somehow! It also doesn't support most features **charted-server** brings
to the table, so, if you want a real registry, then you can use **charted-server** or **chartmuseum**.

## Ready to try?
You can head to the [new repository](https://charts.noelware.org/new) page to get started! If you want to know more about the
features in depth, you can look in the [Features](https://charts.noelware.org/docs/server/features) section.

If you want to try and self-host **charted-server**, you can look at the [Self Hosting](https://charts.noelware.org/docs/server/self-hosting)
page.
