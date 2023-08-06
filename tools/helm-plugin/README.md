# 🐻‍❄️🌺 Helm Plugin for charted-server
> *Helm plugin to help you push your Helm charts to [charted-server](https://github.com/charted-dev/charted) easily!~*

**helm-plugin** is the canonical source for charted's Helm plugin, which helps you push your Helm charts into [charted-server](https://github.com/charted-dev/charted) with `helm charted push` easily.

## Usage
To install the plugin, you can use the `helm plugin install` command:

```shell
$ helm plugin install https://artifacts.noelware.cloud/charted/helm-plugin/latest/helm-plugin.tgz
```

Next, you can setup a `.charted.yaml` file in the directory you wish to setup a repository in with the following contents:

```yaml
# This can be a Name (me/my-repo) or a Snowflake.
repository: me/my-repo

# This will overwrite the `CHARTED_HELM_REGISTRY`/`--registry` flags when pushing
# to force-pushing to a separate charted-server instance.
registry: https://example.com
```

> **Note**: You can also run `helm charted init` to give some prompts to help create this file.

To push a new version, you can run `helm charted push` and it will do the following things:

* Validate any piece of authentication it can.
* Read your `Chart.yaml` to determine the new version.
    * Note: If the version is the same and `--force` isn't specified, this will not push a new version.
* Creates a new release with the [`PUT /repositories/{id}/releases`](https://charts.noelware.org/docs/server/latest/api/resources/repository/releases#PUT-/releases) REST endpoint.
* Create the tarball with `helm package .`
* Upload the tarball with the [`PUT /repositories/{id}/releases/{version}/tarball`](https://charts.noelware.org/docs/server/latest/api/resources/repository/releases#PUT-/releases/{version}/tarball) REST endpoint.
