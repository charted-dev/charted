---
title: charted's REST API
description: Describes all the possible ways to interact with charted's REST API
---

## API Reference

charted's REST API is designed to be consumable by developers who want to easily modify, create, fetch, or delete **entities** from the server.

```shell
# REST API :: Production Server
https://charts.noelware.org/api

# REST API :: Staging Server
https://staging-charts.noelware.org/api
```

## API Versioning

**charted**'s REST API is designed in a way to introduce versioning when requesting to the server. Versioning is useful so we don't introduce many breaking changes and can be redirected accordingly.

You can specify the API version to use with the `/v{version}` path BEFORE all other paths described in the documentation:

```shell
https://charts.noelware.org/api/v1 # => redirects to API version v1
https://charts.noelware.org/api    # => redirects to the default, supported API version
```

#### Versions

| Version | Status    | Default? |
| :------ | :-------- | :------- |
| `/v1`   | Avaliable | Yes      |

## Authenticating with the server

Authenticating with the server is very easy to do so. Instances can be configured to enable [Basic Authentication](https://en.wikipedia.org/wiki/Basic_access_authentication), the production server has it disabled as it is not secure, it's only recommended to evaluation and development purposes.

The server also accepts Bearer-based authentication and API key-based authentication, where API Keys are most recommended as it is a static token that can expire within a set period and is tied to a user account.

The format that the server will accept is `Authorization: TYPE TOKEN`, where `TYPE` can be:

-   `Bearer`
-   `Basic` (if enabled by the instance)
-   `ApiKey`
