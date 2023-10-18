# 🐻‍❄️📦 charted-server Helm chart
This is the canonical Helm chart repository for [charted-server](https://github.com/charted-dev/charted). This Helm chart is made to help ease the entrypoint to run **charted-server** on your own hardware.

## Requirements
* [Kubernetes](https://kubernetes.io) v1.24 or higher
* [Helm](https://helm.sh) 3.9 or higher

## Installation
You can either use the `helm install` command, or if you have the [Helm plugin](https://charts.noewlare.org/docs/helm-plugin/latest) for charted, you can use the `charted://` protocol in `helm install`.

### HTTP Protocol
```shell
$ helm repo add charted https://charts.noelware.org/~/charted
$ helm install charted charted/server
```

### `charted://` protocol
```shell
# you will need the Helm plugin installed to do this!
# $ helm plugin install charted https://artifacts.noelware.cloud/charted/helm-plugin/latest
$ helm install charted charted://charted/server
```

## Values
<!-- ~ HELM_VALUES: START ~ -->

<!-- ~ HELM_VALUES: END ~ -->
