# 🐻‍❄️📦 Contributing to charted-server
Thanks for considering to help build and make **charted-server** even better! We full heartily accept contributions from the community — you! We will accept any contributing from any major feature to small, grammatical bugs.

## Bug Reporting
Think you found an issue that you ran into? To submit a bug report, please use the latest release of the server because it might've been fixed by us, but if it hasn't then, you can surf through the [issue board](https://github.com/charted-dev/charted/issues) if the issue was already been reported, it'll be tagged with the `bug` label and be added onto Noelware's internal issue board.

- Be clear and concise with the title and description of the bug, it will help others link their issues & solutions to yours!
- Specify any way to reproduce the bug, so we know what to fix.

To test REST API-related issues, we recommend using `cURL` so it can be easier to reproduce on the latest version.

- Use number-formatted prefixes (i.e, `1. {step}`) to determine a step.
- Prefix the expected result with `#>`, and the actual result with `#?>`

Example:

```shell
# 1. Create a repository
$ curl -XPUT -H "Content-Type: application/json" -d '{"name":"repo1"}' http://localhost:3651/users/@me/repositories
#> {"success": true}

# 2. Invite a member.
$ curl -XPOST -H "Content-Type: application/json" -d '{"user_id":1234}' http://localhost:3651/repositories/1/members/invite
#> {"success": true}

# 3. Once they were invited, check if they can be queried.
$ curl http://localhost:3651/repositories/1/members
#> {"success": true,"data":[{...}]}

# 4. Kick them!
$ curl -XDELETE -H "Content-Type: application/json" http://localhost:3651/repositories/1/members/1234/kick
#> {"success":true}
#?> {"success":false,"errors":[{...}]}
```

## Security Vulnerabilities
If you found any security vulnerabilities when using **charted-server**, please refer to our [Security Policy](https://github.com/charted-dev/charted/blob/master/SECURITY.md).

## Code Contributions
We alweays accept code contributions since your contributions to anything related to the project makes it more powerful and secure than our team can if we just kept this closed sourced, since it is based off community feedback!

This repository is a monorepo, so the codebase might be intimidating, but this guide is here to help you aid in how we structured the project and such.

> **Note**: We do support using [GitHub Codespaces](https://github.com/codespaces) or [Coder](https://coder.com) (with [Noel's Coder templates](https://github.com/auguwu/coder-images) -- it is recommended since it'll run all preinit scripts and the Docker compose project in `.coder/docker-compose.yml`)
>
> Both Codespaces and Coder have the necessary tooling to help you build and run charted-server easily from a remote environment!

### How is the project structured?
The project is a monorepo that is structured into multiple folders:

* `cli/` is the actual CLI source code.
* `crates/` is the different crates that is used through-out the `cli` and `server` folders.
* `distribution/` is related to how **charted-server** is distributed once a release is settled.
* `scripts/` is any script that helps to not write long commands, or anything really.
* `server/` is the actual REST API.
* `tools/` is tools and services that help aid the `server/` folder.
* `web/` is the actual web interface that is packaged with the server!

Originally, **charted-server** was written in Kotlin, which made it impossible to include both the `web/` and `server/` together without magic with Gradle, Rust and Bun helps us build a monorepo that brings in what **charted-server** brings to the table without trying to separate it between repositories and make it harder on the team.

We don't do any specification for Git commit messages, like [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0), but all Git commits that are pushed to `main` should be helpful with an optional body payload.

> [!NOTE]
>
> In a pull request, you can add meaningless Git commit messages since we merge `main` branches with the PR title (#id) with the README from that PR as the optional body.

## FAQ
### :question: Why do I get a `container unhealthy` error when I run `./dev docker up`?
Because Bitnami's PostgreSQL and Redis containers expect the filesystem path of `./.cache/docker/postgresql` and `./.cache/docker/redis` be with uid and gid `1001`.

To fix it, just run the `down` subcommand of the `docker` subcommand of `./dev` and then `chown`:

```shell
$ ./dev docker down
$ sudo chown -R 1001:1001 ./.cache/docker/postgresql ./.cache/docker/redis
```

Once you do that, you can run `./dev docker up` and it should run as usual:

```shell filename="$ docker logs -f charted_redis"
# » docker logs -f charted_redis
redis 01:37:57.63
redis 01:37:57.63 Welcome to the Bitnami redis container
redis 01:37:57.63 Subscribe to project updates by watching https://github.com/bitnami/containers
redis 01:37:57.63 Submit issues and feature requests at https://github.com/bitnami/containers/issues
redis 01:37:57.63
redis 01:37:57.64 INFO  ==> ** Starting Redis setup **
redis 01:37:57.64 WARN  ==> You set the environment variable ALLOW_EMPTY_PASSWORD=yes. For safety reasons, do not use this flag in a production environment.
redis 01:37:57.65 INFO  ==> Initializing Redis
redis 01:37:57.65 INFO  ==> Setting Redis config file
redis 01:37:57.67 INFO  ==> ** Redis setup finished! **
redis 01:37:57.68 INFO  ==> ** Starting Redis **
1:C 17 Oct 2023 01:37:57.687 # oO0OoO0OoO0Oo Redis is starting oO0OoO0OoO0Oo
1:C 17 Oct 2023 01:37:57.687 # Redis version=7.0.11, bits=64, commit=00000000, modified=0, pid=1, just started
1:C 17 Oct 2023 01:37:57.687 # Configuration loaded
1:M 17 Oct 2023 01:37:57.688 * monotonic clock: POSIX clock_gettime
1:M 17 Oct 2023 01:37:57.688 * Running mode=standalone, port=6379.
1:M 17 Oct 2023 01:37:57.688 # Server initialized
1:M 17 Oct 2023 01:37:57.688 # WARNING Memory overcommit must be enabled! Without it, a background save or replication may fail under low memory condition. Being disabled, it can can also cause failures without low memory condition, see https://github.com/jemalloc/jemalloc/issues/1328. To fix this issue add 'vm.overcommit_memory = 1' to /etc/sysctl.conf and then reboot or run the command 'sysctl vm.overcommit_memory=1' for this to take effect.
1:M 17 Oct 2023 01:37:57.691 * Creating AOF base file appendonly.aof.1.base.rdb on server start
1:M 17 Oct 2023 01:37:57.693 * Creating AOF incr file appendonly.aof.1.incr.aof on server start
1:M 17 Oct 2023 01:37:57.693 * Ready to accept connections
```
