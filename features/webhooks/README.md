# HTTP Webhooks

Allows transmitting events of data via HTTP webhooks. **charted-server** builds upon the [Standard Webhooks Specification v1.0](https://github.com/standard-webhooks/standard-webhooks/blob/main/spec/standard-webhooks.md).

All webhook events are stored in ClickHouse while the webhook metadata itself (i.e, HTTP endpoint) are stored in the main database.
