# charted-server | Helm Plugin
> *Helm plugin to manage Helm chart repositories with [charted-server](https://charts.noelware.org)*
>
> <kbd>v0.3-nightly</kbd> | [:scroll: **Documentation**](https://charts.noelware.org/docs/helm-plugin/current)

This is the source code root for the [`helm charted`](https://charts.noelware.org/docs/helm-plugin/current/cli-reference) subcommand. This exists to make it easy to push your Helm charts into **charted-server** easily. :3

## Usage
```shell
$ helm plugin install https://artifacts.noelware.cloud/charted/helm-plugin

# Login into charted-server. The Helm plugin creates its own API key to upload repositories
$ helm charted login

# Push your Helm chart
$ helm charted push noel/hazel
```
