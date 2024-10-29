# 🐻‍❄️📦 Contributing to `charted-server`

Before we begin, thanks for considering your time to help make **charted-server** even better than we can! We full heartly accept contributions from everyone — including you! We accept major features, minor features to small, grammatical bugs.

## Bug Reporting

Think you might've ran into a bug that should never happen? It happens to the best of us sometimes! To submit a bug report, you can create one via [GitHub Issues][issue-board].

Before you do, make sure that it's something that someone hasn't already reported! You can surf through the [issue board] to see if anyone has already reported it. It'll be tagged with the `bug` label.

-   Be clear and concise with the title and description of the bug! It'll help others link their issues and possible solution to yours.
-   Please specify any way to reproduce the bug, so we know where to look and fix it!

If the bug is related to the REST API, please reproduce the bug with the `curl` utility. We recommend this format:

-   Use a number-formatted list to determine a step
-   Prefix expected results with `#>` and actual results with `#!>`.

```shell
# 1. Ensure a repository exists
$ curl -u myuser:password -XPUT \
    -H "Content-Type: application/json" \
    -d '{"name":"weow","private":true,"type":"application"}' \
    http://localhost:3651/users/@me/repositories

#> {"success": true}

# 2. Invite a member to the repository
$ curl -u myuser:password -XPUT http://localhost:3651/repositories/1/members/invite?user_id=1&force=true
#> {"success": true,"data":{...}}

# 3. Check if they are invited
$ curl -u myuser:password http://localhost:3651/repositories/1/members
#> {"success":true,"data":[{"account":1,...}]}

# 4. Kick them!
$ curl -u myuser:password -XDELETE http://localhost:3651/repositories/1/members/1
#> {"success":true}
#!> {"success":false,"errors":[{"code":"INTERNAL_SERVER_ERROR","message":"Internal Server Error"}]}
```

## Security Vulnerabilities

If you find any security-related issues when using **charted-server**: please refer to our [security policy] for more information. We do not recommend creating an issue UNTIL you contact the team and fix the issue already.

## Development Environment

For Nix/NixOS users, we maintain a [direnv] to standardise the development workflow, it includes the following tools:

-   **rustc**, **cargo**, **rustfmt**, **clippy**
-   **cargo-machete** (`cargo machete`), **cargo-expand** (`cargo expand`), **cargo-deny** (`cargo-deny`)
-   [Diesel] CLI (for database migrations)
-   Hadolint (for Dockerfile linting)
-   Git

To enter the workflow, you can use `direnv allow` and it'll propagate a [direnv]:

```shell
direnv: loading ~/git/charted/.envrc
direnv: loading https://raw.githubusercontent.com/nix-community/nix-direnv/3.0.6/direnvrc (sha256-RYcUJaRMf8oF5LznDrlCXbkOQrywm0HDv1VjYGaJGdM=)
direnv: using flake
direnv: nix-direnv: Using cached dev shell
direnv: export +AR +AR_FOR_TARGET +AS +AS_FOR_TARGET +CC +CC_FOR_TARGET +CHARTED_DISTRIBUTION_KIND +CONFIG_SHELL +CXX +CXX_FOR_TARGET +HOST_PATH +IN_NIX_SHELL +LD +LD_FOR_TARGET +NIX_BINTOOLS +NIX_BINTOOLS_FOR_TARGET +NIX_BINTOOLS_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu +NIX_BINTOOLS_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu +NIX_BUILD_CORES +NIX_CC +NIX_CC_FOR_TARGET +NIX_CC_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu +NIX_CC_WRAPPER_TARGET_TARGET_x86_64_unknown_linux_gnu +NIX_CFLAGS_COMPILE +NIX_CFLAGS_COMPILE_FOR_TARGET +NIX_ENFORCE_NO_NATIVE +NIX_HARDENING_ENABLE +NIX_LDFLAGS +NIX_LDFLAGS_FOR_TARGET +NIX_PKG_CONFIG_WRAPPER_TARGET_HOST_x86_64_unknown_linux_gnu +NIX_STORE +NM +NM_FOR_TARGET +OBJCOPY +OBJCOPY_FOR_TARGET +OBJDUMP +OBJDUMP_FOR_TARGET +PKG_CONFIG +PKG_CONFIG_PATH +RANLIB +RANLIB_FOR_TARGET +READELF +READELF_FOR_TARGET +RUSTFLAGS +SIZE +SIZE_FOR_TARGET +SOURCE_DATE_EPOCH +STRINGS +STRINGS_FOR_TARGET +STRIP +STRIP_FOR_TARGET +__structuredAttrs +buildInputs +buildPhase +builder +cmakeFlags +configureFlags +depsBuildBuild +depsBuildBuildPropagated +depsBuildTarget +depsBuildTargetPropagated +depsHostHost +depsHostHostPropagated +depsTargetTarget +depsTargetTargetPropagated +doCheck +doInstallCheck +dontAddDisableDepTrack +mesonFlags +name +nativeBuildInputs +out +outputs +patches +phases +preferLocalBuild +propagatedBuildInputs +propagatedNativeBuildInputs +shell +shellHook +stdenv +strictDeps +system ~LD_LIBRARY_PATH ~PATH ~XDG_DATA_DIRS
```

You can also use `nix shell` or `nix develop` as well. Since we use [Nix flakes], we provide a fallback for people who don't use Nix flakes.

---

For non Nix/NixOS users, we recommend installing the following tools on your system (if you haven't already):

-   Rustup
-   Git

**Rustup** is recommended since it'll bootstrap the Rust toolchain that is defined in the `rust-toolchain.toml` file.

[security policy]: ../SECURITY.md
[issue-board]: https://github.com/charted-dev/charted/issues
[Nix flakes]: https://nixos.wiki/wiki/flakes
[Diesel]: https://diesel.rs
[direnv]: https://github.com/direnv/direnv
