<img align="right" src="https://cdn.floofy.dev/images/trans.png" alt="Noelware logo" />
<div align="center">
    <h3>🐻‍❄️📦 <code>charted-server</code> by <a href="https://noelware.org" target="_blank">Noelware, LLC.</a></h3>
    <h4>Free, open, and reliable <a href="https://helm.sh">Helm</a> chart registry made in <a href="https://rust-lang.org">Rust</a>.</h4>
    <hr />
</div>

**charted-server** is a free and open platform to help host, test, and build Helm charts all over the world to your side projects to enterprise uses. We built this platform to help run Helm registries that are reslilient and easily being maintainable.

## Installation

### Locally from source

**Required Tools / Prerequisites**:

-   [Rust](https://rust-lang.org)
-   [Git](https://git-scm.com)
-   20GB of storage
-   2GB of system RAM

To clone the repository, you can use the `git pull` command:

```shell
# HTTPS
$ git pull https://github.com/charted-dev/charted

# SSH
$ git pull git@github.com:charted-dev/charted
```

Once you cloned the repository, you can `cd` into it and run:

```shell
$ cargo dev cli
```

This will build the charted CLI in debug mode. To run the CLI, you can use:

```shell
$ cargo dev cli -- -h
```

This will run the actual CLI, to run the server, you will need to use this instead:

```shell
$ cargo dev server
```

## License

**charted-server** is released under the [**Apache 2.0** License](/LICENSE) with love and care by the Noelware team!
