# 🐻‍❄️🔎 search-indexer
> *Search indexer for [charted-server](https://github.com/charted-dev/charted), made in Go*

This microservice hopes to aim to simply on how to index objects from Elasticsearch or Meilisearch for charted-server to not add more overhead to the API server.

## How does this work?
The microservice connects to a PostgreSQL database that contains all the entities from the API server's database and indexes new entities and removes indices from old entities that are no longer present in the database.

This will also listen to create, update, and delete notifications from the PostgreSQL database and index it from those event payloads.
