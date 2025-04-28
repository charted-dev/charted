# OCI Registry

The **OCI registry** feature allows **charted-server** to act like a OCI Registry for storing and querying Helm charts. As of Helm 3, OCI registries are becoming more recommended to use than other Helm chart registries and **charted-server** will extend its API to support OCI registries.

<!-- prettier-ignore -->
> [!IMPORTANT]
> **charted-server** follows the [OCI Registry v1.1 Specification](https://github.com/opencontainers/distribution-spec/blob/v1.1.1/spec.md).

For authentication, our implementation will follow the authorization system we have in place so that you can use a centralized auth system for both systems.

## Configuration

To enable the **OCI registry** feature, you can either place the configuration under the `[features.oci]` table in **charted.toml** or place it in **features/oci.toml**:

```toml
# for features that are in its own file, the `[features.oci]` is not allowed.
[features.oci]
enable = true
```
