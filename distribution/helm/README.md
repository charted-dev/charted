# Helm Chart for [charted-server](https://github.com/charted-dev/charted)

This is the canonical source for charted-server's Helm chart, to deploy a instance on your Kubernetes cluster as fast and as reliable as possible!~

## TL;DR

```shell
$ helm repo add charted https://charts.noelware.org/~/charted
$ helm install charted-server charted/server
```

## Prerequisites

-   Kubernetes 1.23+
-   Helm 3.2+
-   ReadWriteMany provisioners

## Parameters

### Global Parameters

| Name                       | Description                                                                        | Value                |
| -------------------------- | ---------------------------------------------------------------------------------- | -------------------- |
| `fullNameOverride`         | Override the Helm installation name for all Helm-managed objects that we control   | `""`                 |
| `nameOverride`             | Override the Helm installation name for all Helm-managed objects that we control   | `""`                 |
| `replicas`                 | How many replicas for the Deployment. As of 0.4.0-unstable.2, we do not include HA | `1`                  |
| `jvmOptions`               | JVM options to use when running the server.                                        | `-Xmx2040 -Xms1024m` |
| `debug`                    | If debug mode should be enabled. This can be controlled from the configuration.    | `false`              |
| `nodeSelector`             | Constrainted node labels for the charted-server pod                                | `{}`                 |
| `tolerations`              | Tolerations for the Pod                                                            | `[]`                 |
| `affinity`                 | Pod affinity to apply                                                              | `{}`                 |
| `annotations`              | Object of all annotations to apply to each Helm-managed object                     | `{}`                 |
| `extraEnvVars`             | List of environment variables to apply to the Pod                                  | `[]`                 |
| `initContainers`           | List of init containers to apply when running the main container.                  | `[]`                 |
| `podSecurityContext`       | Security context for each Pod that is created by the deployment                    | `{}`                 |
| `containerSecurityContext` | Security context for the container                                                 | `{}`                 |

### Service Parameters

| Name                     | Description                                  | Value       |
| ------------------------ | -------------------------------------------- | ----------- |
| `service.type`           | Service type                                 | `ClusterIP` |
| `service.port`           | Port for connecting to the API server        | `3651`      |
| `service.clusterIP`      | service Cluster IP                           | `""`        |
| `service.loadBalancerIP` | Load Balancer IP                             | `""`        |
| `service.annotations`    | List of annotations to append to the service | `{}`        |

### Ingress Parameters

| Name                  | Description                                                    | Value                    |
| --------------------- | -------------------------------------------------------------- | ------------------------ |
| `ingress.enabled`     | Enable ingress record generation                               | `false`                  |
| `ingress.pathType`    | Ingress path type                                              | `ImplementationSpecific` |
| `ingress.hostname`    | Default hostname for the ingress record                        | `charted.local`          |
| `ingress.annotations` | List of annotations to append to the ingress record            | `{}`                     |
| `ingress.className`   | IngressClass that will be used to implement the Ingress record | `""`                     |
| `ingress.path`        | Default path for the ingress record                            | `/`                      |

### PostgreSQL Parameters

| Name                                          | Description                                                                                                                                                                                                        | Value        |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------ |
| `postgresql.enabled`                          | Whether if PostgreSQL should be installed alongside charted-server. This will install Bitnami's PostgreSQL distribution from their Helm Charts. This can be set to `false` if you already have PostgreSQL running. | `true`       |
| `postgresql.auth.username`                    | Name for a custom user to create                                                                                                                                                                                   | `charted`    |
| `postgresql.auth.password`                    | Password for the custom user to create                                                                                                                                                                             | `""`         |
| `postgresql.auth.database`                    | Name for a custom database to create                                                                                                                                                                               | `charted`    |
| `postgresql.auth.existingSecret`              | Name of existing secret to use for PostgreSQL credentials                                                                                                                                                          | `""`         |
| `postgresql.architecture`                     | PostgreSQL architecture (`standalone` or `replication`)                                                                                                                                                            | `standalone` |
| `postgresql.primary.service.ports.postgresql` | PostgreSQL service port                                                                                                                                                                                            | `5432`       |

### Redis Parameters

| Name                               | Description                                                                                                                                                                                              | Value        |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| `redis.enabled`                    | Whether if Redis should be installed alongside charted-server. This will install Bitnami's Redis distribution from their Helm Charts. This can be set to `false` if you already have PostgreSQL running. | `true`       |
| `redis.existingSecret`             | Name of a secret containing redis credentials                                                                                                                                                            | `""`         |
| `redis.architecture`               | Set Redis architecture                                                                                                                                                                                   | `standalone` |
| `redis.master.service.ports.redis` | Redis port                                                                                                                                                                                               | `6379`       |
| `redis.auth.enabled`               | Enable Redis auth                                                                                                                                                                                        | `true`       |
| `redis.auth.password`              | Redis password                                                                                                                                                                                           | `""`         |
| `redis.auth.existingSecret`        | Name of a secret containing the Redis password                                                                                                                                                           | `""`         |

### Metrics

| Name                               | Description                                                                                                                                                         | Value   |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `metrics.enabled`                  | Whether if metrics should be enabled                                                                                                                                | `true`  |
| `metrics.prometheusRule.enabled`   | If the Helm chart should provide a PrometheusRule object                                                                                                            | `false` |
| `metrics.prometheusRule.rules`     | A list of rules to apply                                                                                                                                            | `[]`    |
| `metrics.metricSets.elasticsearch` | A list of Elasticsearch metric key-sets to enable. Refer to https://charts.noelware.org/docs/server/latest/self-hosting/configuration#keysets for more information. | `[]`    |
| `metrics.metricSets.postgresql`    | A list of PostgreSQL metric key-sets to enable. Refer to https://charts.noelware.org/docs/server/latest/self-hosting/configuration#keysets for more information.    | `[]`    |
| `metrics.metricSets.server`        | A list of API server metric key-sets to enable. Refer to https://charts.noelware.org/docs/server/latest/self-hosting/configuration#keysets for more information.    | `[]`    |
| `metrics.metricSets.redis`         | A list of Redis metric key-sets to enable. Refer to https://charts.noelware.org/docs/server/latest/self-hosting/configuration#keysets for more information.         | `[]`    |
| `metrics.metricSets.jvm`           | A list of JVM-specific metric key-sets to enable. Refer to https://charts.noelware.org/docs/server/latest/self-hosting/configuration#keysets for more information.  | `[]`    |

### Server Configuration

| Name                                 | Description                                                                          | Value                                                    |
| ------------------------------------ | ------------------------------------------------------------------------------------ | -------------------------------------------------------- |
| `config.mapName`                     | The ConfigMap name for the server configuration                                      | `config`                                                 |
| `config.path`                        | Absolute path to load the server configuration file from                             | `/app/noelware/charted/server/config/charted.yaml`       |
| `config.jwtSecretKey.generate`       | Generates a JWT secret key if this is true as a secret.                              | `true`                                                   |
| `config.jwtSecretKey.existingSecret` | The existing secret name for the JWT secret key. It must have only one key: `secret` | `""`                                                     |
| `config.logback.mapName`             | The ConfigMap name for the Logback properties file                                   | `logback-config`                                         |
| `config.logback.path`                | Absolute path to load the logback.properties file from                               | `/app/noelware/charted/server/config/logback.properties` |

### Docker Image Parameters

| Name               | Description                                                                 | Value                         |
| ------------------ | --------------------------------------------------------------------------- | ----------------------------- |
| `image.repository` | Full repository URL to the charted-server Docker image                      | `ghcr.io/charted-dev/charted` |
| `image.pullPolicy` | Pull policy for the image. If the `image.tag` is latest, then use `Always`. | `IfNotPresent`                |
| `image.tag`        | The tag of the Docker image to run                                          | `""`                          |

### Service Account Parameters

| Name                         | Description                                                                                                            | Value  |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ------ |
| `serviceAccount.create`      | Specifies whether a service account should be created                                                                  | `true` |
| `serviceAccount.annotations` | Annotations to add to the service account                                                                              | `{}`   |
| `serviceAccount.name`        | The name of the service account to use. If not set and create is true, a name is generated using the fullname template | `""`   |

### Storage Driver Parameters

| Name                                          | Description                                                                                                                                  | Value                            |
| --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- |
| `storage.driver`                              | The driver to use when configuring storage. (allowed: filesystem, s3)                                                                        | `filesystem`                     |
| `storage.filesystem.directory`                | The container directory path to mount it to                                                                                                  | `/var/lib/noelware/charted/data` |
| `storage.filesystem.persistence.enabled`      | If persistence is enabled or not                                                                                                             | `true`                           |
| `storage.filesystem.persistence.claimName`    | The name to the PVC name                                                                                                                     | `data`                           |
| `storage.filesystem.persistence.storageClass` | Storage class of backing PVC                                                                                                                 | `""`                             |
| `storage.filesystem.persistence.annotations`  | List of annotations to append to this PVC                                                                                                    | `{}`                             |
| `storage.filesystem.persistence.accessModes`  | Persistent Volume Access Modes                                                                                                               | `["ReadWriteMany"]`              |
| `storage.filesystem.persistence.size`         | Max size of this PVC                                                                                                                         | `8Gi`                            |
| `storage.filesystem.persistence.selector`     | Selector to match an existing Persistent Volume for the data PVC                                                                             | `{}`                             |
| `storage.s3.enableSignerV4Requests`           | If we should enable signer v4 requests when requesting to Amazon S3.                                                                         | `false`                          |
| `storage.s3.enforcePathAccessStyle`           | If the S3 client should be configured to use the new path style for S3 connections. This is recommended to be `true` for Minio installations | `false`                          |
| `storage.s3.defaultObjectAcl`                 | Access Control Level for creating objects into S3.                                                                                           | `bucket-owner-full-control`      |
| `storage.s3.defaultBucketAcl`                 | Access Control Level for creating the bucket in S3 if it doesn't exist                                                                       | `bucket-owner-full-control`      |
| `storage.s3.endpoint`                         | AWS endpoint to hit when connecting to S3                                                                                                    | `s3.amazonaws.com`               |
| `storage.s3.region`                           | Region to connect to when connecting to S3                                                                                                   | `us-east-1`                      |
| `storage.s3.bucket`                           | Bucket name                                                                                                                                  | `charted`                        |
| `storage.s3.secrets.create`                   | If the Helm chart should create a secret for holding AWS credentials or not                                                                  | `true`                           |
| `storage.s3.secrets.name`                     | Secret name to use                                                                                                                           | `aws-creds`                      |
| `storage.s3.secrets.existingSecret`           | Existing secret resource name                                                                                                                | `""`                             |
| `storage.s3.secrets.keys.secretAccessKey`     | Key value for the secret access key                                                                                                          | `aws-secret-access-key`          |
| `storage.s3.secrets.keys.accessKeyId`         | Key value for the access key ID                                                                                                              | `aws-access-key-id`              |

### External Services

| Name                                                   | Description                                                                                   | Value     |
| ------------------------------------------------------ | --------------------------------------------------------------------------------------------- | --------- |
| `external.redis.host`                                  | External Redis host                                                                           | `""`      |
| `external.redis.port`                                  | External Redis port                                                                           | `6379`    |
| `external.redis.password`                              | Redis password, if auth is needed                                                             | `""`      |
| `external.redis.database`                              | Redis database                                                                                | `8`       |
| `external.redis.existingPasswordSecret`                | Secret resource for the Redis password                                                        | `""`      |
| `external.redis.existingPasswordSecretKey`             | Secret resource name for the Redis password                                                   | `""`      |
| `external.redis.sentinels.endpoints`                   | List of sentinel endpoints to use if Redis connection is in Sentinel mode                     | `[]`      |
| `external.redis.sentinels.masterName`                  | Sentinel master name if Redis connection is in Sentinel mode                                  | `""`      |
| `external.redis.sentinels.existingMasterNameSecret`    | Secret resource for the sentinel master name if Redis connection is in Sentinel mode          | `""`      |
| `external.redis.sentinels.existingMasterNameSecretKey` | Secret resource key name for the sentinel master name if Redis connection is in Sentinel mode | `""`      |
| `external.postgres.host`                               | External PostgreSQL host                                                                      | `""`      |
| `external.postgres.port`                               | External PostgreSQL port                                                                      | `5432`    |
| `external.postgres.username`                           | PostgreSQL username for authentication                                                        | `""`      |
| `external.postgres.password`                           | PostgreSQL password for authentication                                                        | `""`      |
| `external.postgres.database`                           | PostgreSQL database name, defaults to `charted`.                                              | `charted` |
| `external.postgres.schema`                             | PostgreSQL schema to use, defaults to `public`                                                | `""`      |
| `external.postgres.existingAuthSecret`                 | Existing authentication secret name to do authentication on.                                  | `""`      |
| `external.postgres.existingUsernameSecretKey`          | Secret key name for the username for authentication.                                          | `""`      |
| `external.postgres.existingPasswordSecretKey`          | Secret key name for the password for authentciaton                                            | `""`      |
