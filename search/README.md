# Subproject :search
This contains the modules for using different search backends. We currently support **Meilisearch** and **Elasticsearch**!

## Meilisearch
This uses the Rust-based search database, [Meilisearch](https://meilisearch.com) which is getting popular.

To configure the server to use **Meilisearch**, you will need to use the `search.meili` configuration object:

```yml
search:
  meili:
    endpoint: "http://127.0.0.1:9000"
    master_key: "?"
```

## Elasticsearch
This uses the most widely popular search database, [Elasticsearch](https://elastic.co/elasticsearch)! The official registry uses
Elasticsearch as its search backend, if you didn't know!

Note that using ES will require a bit of mildly annoying configuration and a bit of extra RAM! If you're using **charted-server**
at scale, it is recommended to Elasticsearch as the search backend since ES is distributed by heart.

```yml
search:
  elasticsearch:
    nodes:
      - 127.0.0.1:9200

    auth:
      username: ...
      password: ...
```
