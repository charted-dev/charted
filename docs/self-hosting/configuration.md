---
title: Configuration
description: Reference for the `charted.hcl` file
---

**charted-server** uses the [HashiCorp Configuration Language](https://github.com/hashicorp/hcl) built by [HashiCorp](https://hashicorp.com). It doesn't have functions or variables, so it's just static configuration.

**charted-server** also supports environment variables that can be overwritten when the configuration loader is being ran. The priority is **Environment Variables > Configuration File**.

<pre>
<a href="#charted_jwt_secret_key">jwt_secret_key</a> = "{random characters}"
<a href="#charted_registrations">registrations</a>  = true
<a href="#charted_single_user">single_user</a>    = false
<a href="#charted_single_org">single_org</a>     = false
<a href="#charted_sentry_dsn">sentry_dsn</a>     = null
<a href="#charted_base_url">base_url</a>       = null

<a href="#charted_logging">logging</a> {
    <a href="#charted_logging_level">level</a> = "info"
    <a href="#charted_logging_json">json</a>  = false
}

<a href="#charted_sessions">sessions</a> {
    <a href="#charted_sessions_enable_basic_auth">enable_basic_auth</a> = false

    backend = <a href="#charted_sessions_backend_local">"local"</a>

    backend <a href="#charted_sessions_backend_ldap">"ldap"</a> {}
}

<a href="#charted_server">server</a> {
    <a href="#charted_server_host">host</a> = "0.0.0.0"
    <a href="#charted_server_port">port</a> = 3651

    <a href="#charted_server_ssl">ssl</a> {
        <a href="#charted_server_ssl_cert">cert</a>     = "{path to ssl certificate}"
        <a href="#charted_server_ssl_cert_key">cert_key</a> = "{path to ssl certificate key}"
    }
}

database <a href="#charted_database_sqlite">"sqlite"</a> {
    <a href="#charted_database_sqlite_max_connections">max_connections</a> = 10
    <a href="#charted_database_sqlite_run_migrations">run_migrations</a>  = false
    <a href="#charted_database_sqlite_db_path">db_path</a>         = "./data/charted.db"
}

database <a href="#charted_database_postgresql">"postgresql"</a> {
    <a href="#charted_database_postgresql_max_connections">max_connections</a> = 10
    <a href="#charted_database_postgresql_run_migrations">run_migrations</a>  = false
    <a href="#charted_database_postgresql_password">password</a>        = null
    <a href="#charted_database_postgresql_username">username</a>        = null
    <a href="#charted_database_postgresql_database">database</a>        = "charted"
    <a href="#charted_database_postgresql_schema">schema</a>          = null
    <a href="#charted_database_postgresql_host">host</a>            = "localhost"
    <a href="#charted_database_postgresql_port">port</a>            = 5432
}

storage <a href="#charted_storage_filesystem">"filesystem"</a> {}

storage <a href="#charted_storage_s3">"s3"</a> {}

storage <a href="#charted_storage_azure">"azure"</a> {}
</pre>

| Name                                                                                 | Description                                                                                                                                                       | Type                                                                                                                                 | Required? | Default Value                                                                                                         |
| :----------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :-------- | :-------------------------------------------------------------------------------------------------------------------- |
| <a id="#charted_jwt_secret_key"></a> `jwt_secret_key` (`CHARTED_JWT_SECRET_KEY`)     | Secret key that is used to sign JWT tokens                                                                                                                        | `string`                                                                                                                             | No.       | `{random characters}`                                                                                                 |
| <a id="#charted_registrations"></a> `registrations` (`CHARTED_ENABLE_REGISTRATIONS`) | Allows user registrations via `PUT /users` REST endpoint                                                                                                          | boolean (`true`, `false`)                                                                                                            | No.       | `false`                                                                                                               |
| <a id="#charted_single_user"></a> `single_user` (`CHARTED_SINGLE_USER`)              | Enables the "Single User" option, which will disable most features as it is orientated to a single user instance.                                                 | boolean (`true`, `false`)                                                                                                            | No.       | `false`                                                                                                               |
| <a id="#charted_single_org"></a> `single_org` (`CHARTED_SINGLE_ORG`)                 | Enables the "Single Organization" option, which will disable some features as this registry is orientated for a single organization that can have multiple users. | boolean (`true`, `false`)                                                                                                            | No.       | `false`                                                                                                               |
| <a id="#charted_sentry_dsn"></a> `sentry_dsn` (`CHARTED_SENTRY_DSN`)                 | Whether or not to opt-in to <a href="https://sentry.io" target="_blank">Sentry</a> to have error reporting and tracing features be sent to a Sentry server.       | `string`, formatted as <a href="https://docs.sentry.io/concepts/key-terms/dsn-explainer/" target="_blank">Data Source Name</a> (DSN) | No.       | `null`                                                                                                                |
| <a id="#charted_base_url"></a> `base_url` (`CHARTED_BASE_URL`)                       | URI that will redirect all API requests and Helm chart downloads towards.                                                                                         | `string`                                                                                                                             | No.       | <code>http://<a href="#charted_server_host">{server.host}</a>:<a href="#charted_server_port">{server.port}</a></code> |

<!-- prettier-ignore-start -->

<a id="#charted_logging"></a>
## block `logging {}`
| Name                                                              | Description                                                                | Type                                                | Required? | Default Value |
| :---------------------------------------------------------------- | :------------------------------------------------------------------------- | :-------------------------------------------------- | :-------- | :------------ |
| <a id="#charted_logging_level"></a> `level` (`CHARTED_LOG_LEVEL`) | The log level that all console / JSON logs will be sent as.                | `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"` | No.       | `"info"`      |
| <a id="#charted_logging_json"></a> `json` (`CHARTED_LOG_JSON`)    | whether if all console logs should be printed as a JSON-formatted payload. | `boolean` (`true`, `false`)                         | No.       | `false`       |

<a id="#charted_server"></a>
## block `server {}`
| Name                                                                     | Description                                                        | Type                | Required? | Default Value |
| :----------------------------------------------------------------------- | :----------------------------------------------------------------- | :------------------ | :-------- | :------------ |
| <a id="#charted_server_host"></a> `host` (`CHARTED_SERVER_HOST`, `HOST`) | Host address that the server will accept incoming requests from.   | `string`            | No.       | `0.0.0.0`     |
| <a id="#charted_server_port"></a> `port` (`CHARTED_SERVER_PORT`, `PORT`) | The port that the host address will accept incoming requests from. | `uint16` (1..65535] | No.       | `3651`        |

<a id="#charted_server_ssl"></a>
### block `ssl {}` (`CHARTED_SERVER_SSL`)
| Name                                                                                 | Description                                                       | Type                               | Required? | Default Value |
| :----------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :--------------------------------- | :-------- | :------------ |
| <a id="#charted_server_ssl_cert"></a> `cert` (`CHARTED_SERVER_SSL_CERT`)             | Path to a SSL certificate that is used to enable TLS connections. | Path (either relative or absolute) | Yes       | `null`        |
| <a id="#charted_server_ssl_cert_key"></a> `cert_key` (`CHARTED_SERVER_SSL_CERT_KEY`) | Path to a SSL certificate key.                                    | Path (either relative or absolute) | Yes       | `null`        |

<a id="#charted_sessions"></a>
## block `sessions {}`

<a id="#charted_database_sqlite"></a>
## block `database "sqlite" {}` (`CHARTED_DATABASE_DRIVER` = `"sqlite"`)

<a id="#charted_database_postgresql"></a>
## block `database "postgresql" {}` (`CHARTED_DATABASE_DRIVER` = `"postgresql"`)

<a id="#charted_storage_filesystem"></a>
## block `storage "filesystem" {}` (`CHARTED_STORAGE_SERVICE` = `"filesystem"`)

<a id="#charted_storage_s3"></a>
## block `storage "s3" {}` (`CHARTED_STORAGE_SERVICE` = `"s3"`)

<a id="#charted_storage_azure"></a>
## block `storage "azure" {}` (`CHARTED_STORAGE_SERVICE` = `"azure"`)

<!-- prettier-ignore-end -->
