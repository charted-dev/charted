# Webhooks Support

This feature implements user-defined callbacks via the REST API. Repositories can point to webhooks that link to events that are stored in [ClickHouse](https://clickhouse.com) and perform anything they want with the data.

## Why?

It'll eliminate some usecases that can be used with the Webhooks API:

-   Polling data from the API server as ratelimits can happen if you retrieve data from the API server too fast
-   Easy as setting up a HTTP server where the API server can connect to and send events to that can be deployed on a Docker image and into a cloud provider
-   Since events are sent to all webhooks, they'll deliver as the event is triggered, i.e, when a repository is creating a release
