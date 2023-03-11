# 🐻‍❄️🛞 charted-server :: Helm Chart

This is the official Helm Chart for bootstrapping a **charted-server** instance on your Kubernetes cluster easy and fast as possible~!
This is an easier choice if you prefer using **Helm** over writing your own manifests.

## TL;DR

```shell
$ helm repo add charted https://charts.noelware.org/~/charted
$ helm install charted-server charted/server
```

## Prerequisites

-   Kubernetes 1.23+
-   Helm 3.2+
-   ReadWriteMany provisioners
