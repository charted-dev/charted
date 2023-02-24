# 📦 Contributing to charted-server

**charted-server** is a free and open source Helm Chart registry made in [Kotlin](https://kotlinlang.org) developed by
[JetBrains](https://www.jetbrains.com). We full heartily accept contributions from the community — you! We accept
any contribution from major features that might be cool to implement to small, grammatical bugs.

## Bug reports

If you think you found any bugs when running **charted-server**, please test it on the latest installation of [charted-server](https://github.com/charted-dev/charted/releases)
because it might be fixed! If it wasn't fixed (and it was present in future releases), you can surf through the [issue board](https://github.com/charted-dev/charted/issues)
and search for the issue.

If the issue is not present in the issue board, you can submit a **Bug Reports** issue. Please make sure to do the following:

-   Label the issue with the `bug` label.
-   Be clear and concise with the title, it will help others link their issues and solutions to yours.
-   Specify the ways to reproduce the bug, so we know how to fix it.

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

We always accept code contributions since your contributions make **charted-server** more powerful than our team can,
since it's based off of community feedback and how we should make our product better.

This repository only holds the backend server code for the **charted** project, the web interface's repository is located in
[charted-dev/web](https://github.com/charted-dev/web).

**charted-server** has GitHub Codespaces support that allows you to use [GitHub's Codespaces](https://github.com/codespaces) to develop in
a remote environment, and **charted-server** has support for Coder located in the [coder-templates](https://github.com/charted-dev/coder-templates).
Both Coder and GitHub Codespaces have all the necessary tools that you would need to run and develop **charted-server**.

### Coder

You can develop **charted-server** on your own managed hardware with [Coder](https://coder.com), which what **Noel** uses to
develop his own projects outside his home.

**Noel** has own template that he would recommend to develop **charted-server** on since the default image that is
used can be run on **ARM64** or **x86_64** environments.

First, you will need to clone the [coder-images](https://github.com/auguwu/coder-images) repository in your
current directory with `git clone https://github.com/auguwu/coder-images`, now you can
change the directory to **coder-images**:

![](https://noel-is.gay/images/5245ea46.png)

Now, we need to switch to the **template** directory and create the template into the Coder instance you want in. Well first, we need
to log in to import the template, or we will get an error, but if you're already logged in, you should be set. Run the command
`coder login <url>` and you should be prompted to log in.

Now, we can import the Coder template with the `coder template create` command as shown below:

### GitHub Codespaces

> **Note** - The screenshots might say "Codespaces usage is paid for by auguwu" in the screenshots, please do
> note that **Noel** is not paying for your usage of Codespaces, it's tied to your own account, and in the screenshots,
> **Noel** is logged in as his user on GitHub.

To develop on GitHub Codespaces, you will need access to it which can view the [documentation on how to enable it](). Once it is enabled or
you already had it enabled, you can click the "<> Code" button that is in the repository home page:

![repo homepage](https://noel-is.gay/images/9b44aca1.png)

Next, you will need to click the three dots on the above screenshot and click "+ New with options"

![+ new with options](https://noel-is.gay/images/989813a0.png)

Once you have clicked the button, you will be greeted to this page, but you will need
to edit the options that are suited for the development of **charted-server**, which
is shown below:

![devcontainer config](https://noel-is.gay/images/5e83afe6.png)

Now that you clicked the **Create codespace** button, you should be in Visual Studio Code, but we do not
recommend developing **charted-server** on Visual Studio Code, but if you do wish, we do include a list of
extensions that might be helpful to do so.

You should be in this section (the README might change depending on what day you're reading this)
in Visual Studio Code:

![](https://noel-is.gay/images/a39b8694.png)

Now, you can connect with JetBrains Gateway with the **GitHub Codespaces** plugin in the JetBrains Marketplace. When you do
open it up, and you don't have the plugin installed, it will be under the "Install More Providers" section:

![install more providers section](https://noel-is.gay/images/6b18beda.png)

After that, you can click on using the Codespaces plugin and do the usual stuff that you would need to do
if you haven't installed it with JetBrains Gateway yet. Now, you can click on the Codespace with **IntelliJ IDEA**
as the remote IDE to run on and click the **Connect** button to connect to the codespace.

![](https://noel-is.gay/images/8d5a155e.png)

Once finished, you might get a warning like this:

![](https://noel-is.gay/images/0f4e8887.png)

It is recommended to update the heap size since **charted-server** does take a chunk when developing
on Codespaces, that's why we chose the 16GB one!

Now you're ready to develop **charted-server** in GitHub codespaces! You have access to `psql` and `redis-cli` since
the development container includes those utilities while Postgres and Redis are accessible.

It is recommended to add a `config.yml` file in the root directory and put the contents below in that file:

```yaml
jwt_secret_key: <THIS IS WHERE YOU GENERATE A RANDOM KEY>
database:
    host: postgres
redis:
    host: redis
```
