# 🐻‍❄️🌺 Helm Plugin for charted-server
> *Helm plugin to help you push your Helm charts to [charted-server](https://github.com/charted-dev/charted) easily!~*

**helm-plugin** is the canonical source for charted's Helm plugin, which helps you push your Helm charts into [charted-server](https://github.com/charted-dev/charted) with `helm charted push` easily.

## Usage
To install the plugin, you can use the `helm plugin install` command:

```shell
$ helm plugin install https://artifacts.noelware.cloud/charted/helm-plugin/latest/helm-plugin.tgz
```

Next, you can setup a `.charted.hcl` file in the directory you wish to setup a repository workspace in with the following contents:

```hcl
helm {
    # Cargo-based SemVer of what Helm version we should support
    version = "~ 3.12.1"

    # Cargo-based SemVer of what charted-helm-plugin version we should support
    plugin = "> 0.1.0-beta"
}

registry "my-registry" {
    version = 1
    url     = "https://my-registry.com"
}

repository "my-project" {
    registry = "my-registry"
    publish  = true
    source   = "${cwd}/charts/my-project"
}
```

> **Note**: You can also run `helm charted init` to give some prompts to help create this file.

To push a new version, you can run `helm charted push --all` and it will do the following things:

* Validate any piece of authentication it can.
* Read your `Chart.yaml` to determine the new version.
    * Note: If the version is the same and `--force` isn't specified, this will not push a new version.
* Creates a new release with the [`PUT /repositories/{id}/releases`](https://charts.noelware.org/docs/server/latest/api/resources/repository/releases#PUT-/releases) REST endpoint.
* Create the tarball with `helm package .`
* Upload the tarball with the [`PUT /repositories/{id}/releases/{version}/tarball`](https://charts.noelware.org/docs/server/latest/api/resources/repository/releases#PUT-/releases/{version}/tarball) REST endpoint.
