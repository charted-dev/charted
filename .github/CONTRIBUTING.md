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
If you found any security vulnerabilities within **charted-server**, you can report it via email: team@noelware.org

## Code Contributions
If you wish to contribute to the source code of **charted-server**, this is the guide for you.

At the moment, we only support [IntelliJ IDEA](https://jetbrains.com/idea) at the moment for developing **charted-server**.

### Prerequisites 
- PostgreSQL
- Java 17
- Redis
- IntelliJ IDEA
- 2-8GB of system RAM
