---
title: Organization API Entity
description: This describes the fields and properties available on using the Organizations API.
createdAt: 2022-04-03T20:58:49.056Z
---

# Organization

**Organizations** are not a core aspect of what charted-server brings as of features. It's just a entity that marks the repository owner as an organization, like companies like Google, Microsoft, and much more.

They are ordinary [**User**](https://charts.noelware.org/docs/server/entities/user) entities, but with extra features to make them unique.

## JSON Structure

> Mock data is from [etcd.io](https://etcd.io) and [CoreOS](https://coreos.com); they do not use charted-server and
> we are not using real data from our servers.

```json
{
  "repositories": [
    {
      "stargazers_count": 69420,
      "downloads_count": 72927392,
      "description": "Distributed reliable key-value store for the most critical data of a distributed system",
      "updated_at": "2022-04-03T21:00:37.808Z",
      "created_at": "2022-04-03T21:00:37.808Z",
      "icon_url": null,
      "owner_id": "<organization id>",
      "name": "etcd",
      "id": "<repository id>"
    }
  ],

  "single_sign_on": false,
  "gravatar_email": null,
  "description": "Key components to secure, simplify and automate your container infrastructure",
  "avatar_url": "https://charts.noelware.org/api/storage/:org_id/avatars/:hash.png",
  "updated_at": "2022-04-03T21:04:13.986Z",
  "created_at": "2022-04-03T21:04:13.986Z",
  "username": "coreos",
  "verified_publisher": true,
  "flags": 0,
  "name": "CoreOS"
}
```

## Properties

| Field                | Type                                                                                                  | Description                                                           |
| -------------------- | ----------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `repositories`       | [Array<Repository>](https://charts.noelware.org/docs/entities/repository)                             | list of repositories owned by the org.                                |
| `single_sign_on`     | bool                                                                                                  | if the org has single sign on enabled.                                |
| `gravatar_email`     | string?                                                                                               | the gravatar email to use; overrides `avatar_url`.                    |
| `description`        | string?                                                                                               | description of the organization.                                      |
| `avatar_url`         | string?                                                                                               | the avatar url that is used to display on the site.                   |
| `updated_at`         | date                                                                                                  | the last timestamp the organization was updated in the database.      |
| `created_at`         | date                                                                                                  | timestamp of when the organization was created                        |
| `username`           | string                                                                                                | the organization's username to use when redirecting.                  |
| `verified_publisher` | bool                                                                                                  | if the organization is marked verified by Noelware                    |
| `flags`              | int of [OrganizationFlags](https://charts.noelware.org/docs/entities/organization#organization-flags) | the flags granted by the administrators to grant special permissions. |
| `name`               | string                                                                                                | the display name of the organization.                                 |

# Organization Flags

This is a enumeration of all the flags that can be granted on an single organization.

```typescript
enum OrganizationFlags {
  Private = 1 << 0,
  InviteOnly = 1 << 1
}
```

## Enumeration Properties

| Field     | Bit              | Description                                                                                                                                       |
| --------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Private` | `1 << 0` (**1**) | Marked if the organization is made private, only the members of the organization can pull, push, and view charts distributed by the organization. |
