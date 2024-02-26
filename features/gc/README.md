# Garbage Collection

This feature allows the API server to spawn a Tokio task to remove unnecessary objects in the datastore via constraints. The constraints lexer is implemented with [Logos](https://docs.rs/logos) and parsed with a custom parser.

## Why?

As the server grows and you use it more, there is unnecessary Helm charts, users, organizations, etc that might keep using more disk. The way is to manually remove them yourself, but that can be a tedious task!

This implements:

-   REST handler to run the garbage collector and grab metrics about previous iterations (i.e, total disk saved, etc.)
-   Command line interface (`charted gc`) to run and grab metrics.

## Configuration

This can be configured in `./config/charted.toml` with the `[features.gc]` TOML table:

```toml filename=./config/charted.toml
[features.gc]
cron = "@daily" # runs at 00:00 - this is the base cron schedule, it'll be the default if none were specified.

# Specify a constraint that the garbage collector will use to determine
# how a entity should be garbage collected.
[[features.constraint]]
entity = "Repository"
constraint = "updated_at >= 30d"
description = "Delete all repositories that haven't been updated in 30 days"
actions = [
    # delete it from the database
    "delete",

    # send the email to the owner and the team members
    "email"
]
```

The garbage collector will:

-   Run a cron job (specified in `gc.cron` or `gc.constraint[].cron`) to check if the `constraint` is true, then will run the following actions:
    -   Deletes the repository from the database
    -   Sends a email that a repository was deleted (if the emails service is enabled, this will be nop if not enabled)
