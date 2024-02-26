# OCI Registry

This feature allows `charted-server` to act as a [OCI-based registry](https://github.com/opencontainers/distribution-spec/blob/main/spec.md) for ease of convience.

This would mean, _yes_, you can upload Docker images onto `charted-server`, even though it is primarily a Helm chart registry. We won't care!

## Why? Would that defeat the purpose of charted-server?

Not exactly. It would allow tools to ingest Helm charts from charted-server from a respected and probably official way to load up any container-based image like a Helm chart or Docker image.

<!-- prettier-ignore -->
> [!IMPORTANT]
> `charted-server`'s API is imaginary, it's not official.

## How would I enable it?

You can use the `[[features.oci]]` TOML table:

```toml
[[features.oci]]
allow_docker_images = false # allows uploading docker images
```
