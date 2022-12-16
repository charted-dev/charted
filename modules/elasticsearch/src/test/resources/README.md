# Elasticsearch Tests :: SSL
For SSL tests, you will need to generate a **csr-bundle.zip** file, which will make sure tests run. In CI, this will automatically be mapped
as a GitHub secret.

```shell
$ docker pull docker.elastic.co/elasticsearch/elasticsearch:[LATEST TAG]
$ docker run --rm -it -v $(pwd):/certs docker.elastic.co/elasticsearch/elasticsearch:[LATEST TAG] /bin/bash
# $ ./bin/elasticsearch-certutil csr --name=whatever
# ENTER
# $ cp csr-bundle.zip /certs/csr-bundle.zip
# CTRL+C
```

You will need to sign the generated certificate with your certificate authority. Once you're done with that, you might have to add the certificate
authority to your system, refer to the system's documentation how to do so
