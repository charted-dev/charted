# Garbage Collection

This feature allows the API server to spawn a Tokio task to remove unnecessary objects in the datastore via constraints. The constraints lexer is implemented with [Logos](https://docs.rs/logos) and parsed with a custom parser.

## Why?

As the server grows and you use it more, there is unnecessary Helm charts, users, organizations, etc that might keep using more disk. The way is to manually remove them yourself, but that can be a tedious task!

This implements:

-   REST handler to run the garbage collector and grab metrics about previous iterations (i.e, total disk saved, etc.)
-   Command line interface (`charted gc`) to run and grab metrics.

## Configuration

This can be configured with the `./config/gc.yaml` file or in `./config/charted.yaml` with the `gc` key:

```yaml filename=./config/charted.yaml
gc:
    cron: '0 0 * * *' # runs at 00:00 for each constraint that doesn't have a `gc.constraints[].cron` value set.
    constraints:
        - $object: Repository
          constraint: updated_at >= 30d
          description: Delete repositories that haven't been updated in 30 days
          actions:
              - delete:db
              - email:deletion
```

The garbage collector will:

-   Run a cron job (specified in `gc.cron` or `gc.constraints[].cron`) to check if the `constraint` is true, then will run the following actions:
    -   Deletes the repository from the database
    -   Sends a email that a repository was deleted (if the emails service is enabled, this will be nop if not enabled)
