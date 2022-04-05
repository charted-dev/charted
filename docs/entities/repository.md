---
title: Repository API Entity
description: This describes the fields and properties available on using the Repositories API.
createdAt: 2022-04-05T12:00:33.146Z
---

# Repository

A **repository** is a simple "Helm chart" created by a [User](https://charts.noelware.org/docs/server/entities/user) or an [Organization](https://charts.noelware.org/docs/server/entities/organization) in which it can be **private** or **public**.

You would generally use the [Parcel](https://charts.noelware.org/docs/cli) CLI utility to pull and push data to **charted-server** since Helm has no way to grabbing the Helm chart since it is private by the repository owners. You can use the `helm` CLI tool if the package is public.

## JSON Structure

```json
{
  "stargazers_count": 69420,
  "downloads_count": 72927392,
  "description": "Tiny, and simple Elasticsearch microservice to abstract searching objects!",
  "display_name": "翼 Tsubasa",
  "updated_at": "2022-04-05T12:04:05.315Z",
  "created_at": "2022-04-05T12:04:04.986Z",
  "icon_hash": ":hash",
  "icon_url": "https://charts.noelware.org/api/v1/storage/icons/:repo_id/:hash.:ext[jpg,png,gif]",
  "readme": "< markdown content of it; retrieved from storage API. (not stored in db) >",
  "owner": {
      [... User object ...]
  },
  "name": "tsubasa",
  "id": "[repo id]"
}
```

| Field | Type | Description |
| ----- | ---- | ---------- |
| `stargazers_count` | int | How many stars the repository has, can be clicked using the "Star Repo" button. |
| `downloads_count` | int | How many downloads the repository has from `parcel install` or `helm install`. |
| `display_name` | string? | The display name of the repository when displayed on the site. |
| `description` | string? | Short description on what the repository is. |
| `updated_at` | date | Date on when the repository was last updated. |
| `created_at` | date | Date on when the repository was created at. |
| `icon_hash` | string? | The hash of the icon that was stored. |
| `icon_url` | string? | Returns the URL to the icon itself. returns `null` if `icon_hash` is null. |
| `readme` | string? | The readme of the repository, this is retrieved on the storage disk rather than the database. |
| `owner` | [User](https://charts.noelware.org/docs/server/entities/user) or [Organization](https://charts.noelware.org/docs/server/entities/organization) | The owner of the repository, can be a `User` or `Organization`. |
| `name` | string | The short name of the repository. |
| `id` | snowflake | The repository ID. |
