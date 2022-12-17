# 🐻‍❄️🛞 charted-server :: Helm Chart

This is the official Helm Chart for bootstrapping a **charted-server** instance on your Kubernetes cluster easy and fast as possible~!

## TL;DR

```shell
$ helm repo add charted https://charts.noelware.org/~/charted
$ helm install charted-server charted/server
```

## Prerequisites

- Kubernetes 1.23+
- Helm 3.2+
- ReadWriteMany provisioners

## Parameters

### Global Parameters

| Name                      | Description                                                                                                                                                                                                                                    | Value                |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| `global.fullNameOverride` | String to fully override the Helm installation name for all objects                                                                                                                                                                            | `""`                 |
| `global.nameOverride`     | String to override the Helm installation name for all objects                                                                                                                                                                                  | `""`                 |
| `global.replicas`         | How many replicas for the StatefulSet or Deployment. As of right now, charted-server does not implement High Avability.                                                                                                                        | `1`                  |
| `global.jvmOptions`       | The JVM options to use when running the server                                                                                                                                                                                                 | `-Xmx4096 -Xms1024m` |
| `global.debug`            | If the debug flag should be enabled on the server                                                                                                                                                                                              | `false`              |
| `global.nodeSelector`     | Constrainted node labels for the charted-server pod                                                                                                                                                                                            | `{}`                 |
| `global.tolerations`      | Tolerations for charted-server pod                                                                                                                                                                                                             | `[]`                 |
| `global.affinity`         | Affinity for the charted-server pod                                                                                                                                                                                                            | `{}`                 |
| `global.extraEnvVars`     | List of environment variables to add                                                                                                                                                                                                           | `[]`                 |
| `global.initContainers`   | List of init containers to run                                                                                                                                                                                                                 | `[]`                 |
| `global.meilisearch`      | Whether if Meilisearch should be installed alongside charted-server. This will not actually                                                                                                                                                    | `false`              |
| `global.clickhouse`       | Whether if ClickHouse should be installed alongside charted-server. This will install Bitnami's ClickHouse distribution from their Helm Charts.                                                                                                | `false`              |
| `global.prometheus`       | Whether if Prometheus should be installed alongside charted-server. This will install Bitnami's Prometheus distribution from their Helm Charts. This will not affect metrics from the server configuration side, it will just not be ingested. | `false`              |
| `global.postgres`         | Whether if PostgreSQL should be installed alongside charted-server. This will install Bitnami's PostgreSQL distribution from their Helm Charts. This can be set to `false` if you already have PostgreSQL running.                             | `true`               |
| `global.redis`            | Whether if Redis should be installed alongside charted-server. This will install Bitnami's Redis distribution from their Helm Charts. This can be set to `false` if you already have PostgreSQL running.                                       | `true`               |

### Server Configuration

| Name                                 | Description                                                                          | Value                                                    |
| ------------------------------------ | ------------------------------------------------------------------------------------ | -------------------------------------------------------- |
| `config.mapName`                     | The ConfigMap name for the server configuration                                      | `config`                                                 |
| `config.path`                        | Absolute path to load the server configuration file from                             | `/app/noelware/charted/server/config/charted.yaml`       |
| `config.config`                      | The actual configuration for the server                                              | `""`                                                     |
| `config.jwtSecretKey.generate`       | Generates a JWT secret key if this is true as a secret.                              | `true`                                                   |
| `config.jwtSecretKey.existingSecret` | The existing secret name for the JWT secret key. It must have only one key: `secret` | `""`                                                     |
| `config.logback.mapName`             | The ConfigMap name for the Logback properties file                                   | `logback-config`                                         |
| `config.logback.path`                | Absolute path to load the logback.properties file from                               | `/app/noelware/charted/server/config/logback.properties` |
| `config.logback.config`              | The actual configuration for Logback                                                 | `""`                                                     |

### Docker Image Parameters

| Name               | Description                                                                 | Value                         |
| ------------------ | --------------------------------------------------------------------------- | ----------------------------- |
| `image.repository` | Full repository URL to the charted-server Docker image                      | `ghcr.io/charted-dev/charted` |
| `image.pullPolicy` | Pull policy for the image. If the `image.tag` is latest, then use `Always`. | `IfNotPresent`                |
| `image.tag`        | The tag of the Docker image to run                                          | `0.3.0-nightly`               |

### Service Account Parameters

| Name                         | Description                                                                                                            | Value  |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ------ |
| `serviceAccount.create`      | Specifies whether a service account should be created                                                                  | `true` |
| `serviceAccount.annotations` | Annotations to add to the service account                                                                              | `{}`   |
| `serviceAccount.name`        | The name of the service account to use. If not set and create is true, a name is generated using the fullname template | `""`   |

### Elasticsearch

| Name                                     | Description                                                                  | Value                                |
| ---------------------------------------- | ---------------------------------------------------------------------------- | ------------------------------------ |
| `elasticsearch.enabled`                  | If the Helm installation should install a single node Elasticsearch cluster. | `false`                              |
| `elasticsearch.metrics.enabled`          | If the server should install Metricbeat as a sidecar container.              | `false`                              |
| `elasticsearch.metrics.image.repository` | The abstract repository image without the tag                                | `docker.elastic.co/beats/metricbeat` |
| `elasticsearch.metrics.image.pullPolicy` | Pull policy for the image. If the `image.tag` is latest, then use `Always`.  | `IfNotPresent`                       |
| `elasticsearch.metrics.image.tag`        | Repository tag to use                                                        | `8.5.3`                              |

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

### Storage Parameters

| Name                                          | Description                                                                                                                                  | Value                            |
| --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- |
| `storage.driver`                              | The driver to use when configuring storage. (allowed: filesystem, s3)                                                                        | `filesystem`                     |
| `storage.filesystem.directory`                | The container directory path to mount it to                                                                                                  | `/var/lib/noelware/charted/data` |
| `storage.filesystem.persistence.enabled`      | If persistence is enabled or not                                                                                                             | `true`                           |
| `storage.filesystem.persistence.claimName`    | The name to the PVC name                                                                                                                     | `storage`                        |
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
| `storage.s3.secrets.keys.secretAccessKey`     | Key value for the secret access key                                                                                                          | `aws-secret-access-key`          |
| `storage.s3.secrets.keys.accessKeyId`         | Key value for the access key ID                                                                                                              | `aws-access-key-id`              |
