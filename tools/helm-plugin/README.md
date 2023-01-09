# charted-server | Helm Plugin

> _Helm plugin to manage Helm chart repositories with [charted-server](https://charts.noelware.org)_
>
> <kbd>v0.3-nightly</kbd> | [:scroll: **Documentation**](https://charts.noelware.org/docs/helm-plugin/current)

This is the source code root for the [`helm charted`](https://charts.noelware.org/docs/helm-plugin/current) subcommand. This exists to make
it easy to push your Helm charts into **charted-server** easily. :3

## Usage

> **Note** - Replace `[os]` with your operating system (`windows`, `darwin` [macOS], `linux`) and `[arch]` with your host
> architecture (`amd64` [x86_64], `arm64`)

```shell
$ helm plugin install https://artifacts.noelware.cloud/charted/helm-plugin/{{VERSION}}/helm-plugin.[os]-[arch].tar.gz

# Login into charted-server. The Helm plugin creates its own API key to upload in repositories
$ helm charted login

# Push your Helm chart
$ helm charted push noel/hazel
```

You can also have a `.charted.yaml` file in the directory where you are executing `helm charted` to represent what chart it is:

```yaml
repository: noel/hazel
```
