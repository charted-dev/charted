# 📦 Contributing to charted-server
**charted-server** is a free and open source Helm Chart registry made in [Kotlin](https://kotlinlang.org) developed by
[JetBrains](https://www.jetbrains.com). We full heartily accept contributions from the community — you! We accept
any contribution from major features that might be cool to implement to small, grammatical bugs.

## Bug reports
If you think you found any bugs when running **charted-server**, please test it on the latest installation of [charted-server](https://github.com/charted-dev/charted/releases)
because it might be fixed! If it wasn't fixed (and it was present in future releases), you can surf through the [issue board](https://github.com/charted-dev/charted/issues)
and search for the issue.

If the issue is not present in the issue board, you can submit a **Bug Reports** issue. Please make sure to do the following:

- Label the issue with the `bug` label.
- Be clear and concise with the title, it will help others link their issues and solutions to yours.
- Specify the ways to reproduce the bug, so we know how to fix it.

We recommend using the terminal (if possible) to check for issues, i.e (simple example to show how it can be reproduced):

```shell
$ curl -XDELETE -H "Authorization: ApiKey <api key here>" http://localhost:3651/repositories/1/members/2
{"success": true}

$ curl -XGET http://localhost:3651/repositories/1/members
{"success":true,"data":[{"display_name":"Noel","joined_at":"2022-07-30T16:09:30.440Z","updated_at":"2022-07-30T16:09:30.440Z","user":{}}]}
```

## Security Vulnerabilities
If you found any security vulnerabilities when using **charted-server**, please refer to our [Security Policy](https://github.com/charted-dev/charted/blob/master/SECURITY.md).

<!-- If you found any security vulnerabilities when using **charted-server**, please refer to our [Security Policy](https://noelware.org/security/policy). -->

## Code Contributions
We always accept code contributions since your contributions make **charted-server** more powerful than our team can, since it's based off of community feedback and how we should make our end product better.

Since **charted-server** holds the repository for both the [web UI](https://github.com/charted-dev/charted/tree/main/web) and the server, here's the general prerequisites:

- PostgreSQL 12 or higher
- Node.js 18 or higher
- Java 17 or higher
- Redis 6 or higher
- 2 to 8GB of system RAM

We support building the **web UI** platform in **Visual Studio Code** and the server in **IntelliJ IDEA**. Any other platforms that is not Linux, macOS, or Windows are not supported to be used as a development platform, and we only support building **charted-server** in x86_64 or ARM64 architectures! The ClickHouse migrations project (located in [databases/clickhouse/migrations](https://github.com/charted-dev/charted/tree/main/databases/clickhouse/migrations)) can be compiled with **Go**.

### Server
The **server** and the rest of the subprojects (excluding `:web`) can be imported in **IntelliJ IDEA**. It's also important to run the `gitHooks` Gradle task to apply pre-commit hooks if contributing more than once, our GitHub bot will notify you if any errors occur in PRs if linting or running unit/integration tests had failed.

To apply the pre-commit hook, you can just run:

```shell
# Unix (Linux, macOS)
$ ./gradlew gitHooks

# Windows
$ gradlew.bat gitHooks
```

Now that you got that set-up, you can start cloning the Git repository and getting it setup with **IntelliJ IDEA**:

```shell
$ git clone https://github.com/charted-dev/charted
```

It's also best recommended to fork the repository and setting up a upstream remote, you can start forking it by [clicking me!](https://github.com/charted-dev/charted/fork)

```shell
# omit $USERNAME with your user that cloned this repository!
$ git clone https://github.com/$USERNAME/charted && cd charted

# only run this if you plan to contribute many times
$ git remote add upstream https://github.com/charted-dev/charted
```

Now that you cloned the repository, I should go over the **Makefile** present in the root directory of **charted-server**. The **Makefile** contains recipes that makes compiling **charted-server** easier, I (Noel) use **Make** to develop **charted-server**:

Using the **Makefile** on Windows is not recommended, you can opt using the Gradle wrapper script instead, since the **Makefile** was only used in a **Unix** environment.

| Command           | Configured Gradle Task                           | Description                                                                                                                                                                                                                                      |
|:------------------|:-------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **make spotless** | `./gradlew spotlessApply`                        | This will run **Spotless**. **charted-server** uses Spotless as our formatter with **ktlint** by Pinterest.                                                                                                                                      |
| **make build**    | `./gradlew :cli:installDist`                     | This will run the **:server:installDist** Gradle task. It will create a distribution in `server/build/install/charted-server` that you can run using **make run**.                                                                               |
| **make clean**    | `./gradlew clean`                                | This will remove any `build/` directories in all subprojects, including the root project. Generally recommended to run.                                                                                                                          |
| **make test**     | `./gradlew test`                                 | Runs all unit and integration tests with **JUnit 5**. Since integration tests make use of Docker, you are generally recommended to use Docker anyway. But, it will disable the integration tests if **Docker** 20.10 or higher is not installed. |
| **make**          | `./gradlew clean spotlessApply :cli:installDist` | The default target. This will run the `build`, `spotless`, and `clean` tasks and run the server, which can be accessible with `localhost:3651`, if PostgreSQL and Redis are properly configured.                                                 |

### Web UI
The **Web UI** can be located in the [web/](https://github.com/charted-dev/charted/tree/main/web) directory. It's generally best recommended to use **Visual Studio Code**, but using **WebStorm** is also completely ok.

If you're just building the web UI, you will need to have everything installed, including **Node.js**. We include tasks in the `:web` subproject that will manage the Node.js installation in `$ROOT/build/nodejs/{version}`, if `./gradlew :web:extractNodeInstallation` was used.

It will also look for a Node.js binary to use if you want to use your local Node.js installation methods. The folder also includes a `.node-version` file to use asdf-vm, nvm, and other tools.

To open the project, you can run `code ./web` (or `code-insiders ./web` if using Insiders) if in the directory that you cloned **charted-server**! The web UI uses React and Vite as the stack.

You can run `yarn` (or `./gradlew :web:installDeps`) to install the dependencies in the infamous `node_modules` directory in the **web** project. You can start up the Vite development server by using `yarn dev` (or `./gradlew :web:dev`), you should have **charted-server** running in a seperate terminal.

You can configure the web UI with a **config.yml** file in the `web` directory, it can look like this:

```yaml
# The Sentry DSN to use for reporting errors. This is automatically linked with the React Error Boundary component.
# This is optional and should be only used in production environments.
sentry_dsn: string

# If tracing should be enabled or not. Tracing happens early on when starting up the development or production server,
# you can configure Elastic APM, we do want to support OpenTelemetry later.
tracing:
  # Configures using Elastic APM to collect tracing metadata.
  apm:
    # The server URL to connect to APM Server. If you're using Elastic Agent, please expose :8200 on your
    # machine on any agent that has the APM integration enabled.
    server_url: url
    
# This contains the proxy configuration. The web UI can proxy the API server by using `true` to link it
# to `/api/*` towards the charted-server instance the web UI is using. You can also use an configuration
# object to configure the base prefix.
proxy: boolean or configuration object
#  prefix: string? (default /api/*)

# The connection towards charted-server configuration. This will automatically set up a liveness probe on the server if `charted.liveness.enabled`
# is set to true.
charted:
  host: ip/url
  port: number between 1024~65535
  ssl: boolean or object
  #  ca: file?
  #  key: file?
  liveness:
    enabled: boolean (default true)
    interval: time value, number (default `15s`)

# The actual server configuration. By default, the web UI will start at `localhost:2134`, but you can configure that yourself.
server:
  host: ip/url
  port: number between 1024 ~ 65535, cannot be the same as `charted.port`
  headers: map[string, string]
```

- any type ending with `?` is considered optional.
- `map[string, string]` refers to a YAML map, i.e: `headers: { key: "string" }`.
- `time value` refers to a time value (i.e, `15s`, `13 minutes`)
