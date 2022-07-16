# charted-server on AUR
This is the official distribution of **charted-server** that you can install via the Arch User Repository.

## Installation
```shell
$ yay -S charted-server
```

It will install the binary via GitHub Releases, create a systemd service, and you can start it via:

```shell
$ sudo systemctl start charted-server
$ sudo systemctl enable charted-server
```

You can access charted-server via `http://localhost:4321`!

## Development
The basic [PKGBUILD](./template.PKGBUILD) is a template to building the server. You can run the following command to get a proper,
and valid PKGBUILD in `./distribution/aur/.repo/PKGBUILD`:

```shell
$ ./gradlew :distribution:aur:buildPackage
```

You will need [base-devel](https://archlinux.org/groups/x86_64/base-devel) before you run the command.
