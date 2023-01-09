# 🐻‍❄️🐳 charted Helm Plugin :: Docker Images

This is the canonical source root for the Helm plugin images that are available to be used. This extends from the [alpine/helm](https://hub.docker.com/r/alpine/helm) Docker image
which bundles in charted's [Helm Plugin](https://charts.noelware.org/docs/helm-plugin/latest).

The Docker image tags support up to Helm 3.9, so you can specify which Helm release to use:

- ghcr.io/charted-dev/helm-plugin:nightly-3.10.2 :: Uses the **nightly** versions of charted's Helm Plugin with Helm **3.10.2**
- ghcr.io/charted-dev/helm-plugin:v0.4-nightly-3.10.2 :: Uses the Helm plugin v0.4-nightly with Helm **3.10.2**
- cr.noelware.cloud/charted/helm-plugin:latest-3.9.4 :: Uses the latest version of the Helm plugin with Helm **3.9.4**

## Usage

```shell
$ docker run --rm \
  -e CHARTED_HELM_CONFIG_PATH="~/.config/charted-helm/config.json" \
  ghcr.io/charted-dev/helm-plugin:nightly helm charted push .
```
