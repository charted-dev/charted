<img src="https://cdn.floofy.dev/images/trans.png" width="96" height="96" alt="Noelware logo" align="right" />

### 🐻‍❄️📦 [`charted-server`] by [Noelware, LLC.]

#### _Open, powerful, and reliable Helm chart registry made in [Rust]._

**charted-server** is a free and open platform that helps host, test, and build [Helm] charts all over the world to any side project to enterprise work. Noelware built this platform to build the cloud that we put our sweat and tears into.

## Installation

### Locally via Git

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

Once you cloned the repository, you can `cd` into it and run the `charted` CLI:

```shell
$ cargo cli
```

This will run the actual CLI, to run the server, you will need to use this instead:

```shell
$ cargo server
```

### Docker

> [!IMPORTANT]
> You can checkout the [`charted-dev/self-hosted`] repository for a production ready **charted-server** instance with Docker Compose.

### Kubernetes/Helm

Refer to the [`charted-dev/helm-charts`] repository for more information on how to deploy a **charted-server** instance on Kubernetes.

### Nix/NixOS

#### Nix

You can use the **charted** and **charted-helm-plugin** Nix derivations from [`nixpkgs-noelware`]:

```nix
{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        noelware = {
            url = "github:Noelware/nixpkgs-noelware";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { nixpkgs, noelware, ... }: let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
            inherit system;

            overlays = [(import noelware)];
        };
    in
    {
        devShells.${system}.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                charted

                (wrapHelm kubernetes-helm {
                    plugins = [charted-helm-plugin];
                })
            ];
        };
    };
}
```

#### NixOS

The [`nixpkgs-noelware`] repository contains a NixOS module to run a **charted-server** instance:

```nix
{
    services.charted = {
        enable = true;
        statePath = "/var/lib/noelware/charted/data";

        settings = {
            single_user = true;
            sessions.backend.static = {
                noel = "<argon2 hashed password>";
            };
        };
    };
}
```

## Contributing

Thanks for considering contributing to **charted-server**! Before you boop your heart out on your keyboard ✧ ─=≡Σ((( つ•̀ω•́)つ, we recommend you to do the following:

-   Read the [Code of Conduct](./.github/CODE_OF_CONDUCT.md)
-   Read the [Contributing Guide](./.github/CONTRIBUTING.md)

If you read both if you're a new time contributor, now you can do the following:

-   [Fork me! ＊\*♡( ⁎ᵕᴗᵕ⁎ ）](https://github.com/charted-dev/charted/fork)
-   Clone your fork on your machine: `git clone https://github.com/your-username/charted`
-   Create a new branch: `git checkout -b some-branch-name`
-   BOOP THAT KEYBOARD!!!! ♡┉ˏ͛ (❛ 〰 ❛)ˊˎ┉♡
-   Commit your changes onto your branch: `git commit -am "add features （｡>‿‿<｡ ）"`
-   Push it to the fork you created: `git push -u origin some-branch-name`
-   Submit a Pull Request and then cry! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡

## License

**charted-server** by [Noelware, LLC.] is released under the **Apache 2.0** License with love and care by the team. Please read the [`LICENSE`](/LICENSE) file in the canonical repository for more information on what you can do with the source code for **charted-server**.

[`charted-dev/helm-charts`]: https://github.com/charted-dev/helm-charts
[`charted-dev/self-hosted`]: https://github.com/charted-dev/self-hosted
[`nixpkgs-noelware`]: https://github.com/Noelware/nixpkgs-noelware
[`charted-server`]: https://charts.noelware.org/
[Noelware, LLC.]: https://noelware.org
[Helm]: https://helm.sh
[Rust]: https://rustlang.org

<!-- <div align="center">
    <img src="https://cdn.floofy.dev/images/trans.png" alt="Noelware logo" />
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

**charted-server** is released under the [**Apache 2.0** License](/LICENSE) with love and care by the Noelware team! -->
