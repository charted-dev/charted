---
title: Guide on installing charted-server on Kubernetes
description: Guide on how to easily install charted-server on Kubernetes with the Helm package manager or with Noelware's Kubernetes Operator
---

## Helm

To install **charted-server** with the Helm package manager, you can add charted's Helm repository with

```shell
$ helm repo add charted https://charts.noelware.org/~/charted
```

```shell
$ helm install charted charted/server --namespace charted-system --create-namespace
```

## Kubernetes Operator

To install **charted-server** with Noelware's Kubernetes Operator, you will need to install the Kubernetes
operator on your system with:

```shell
$ kubectl apply -f https://artifacts.noelware.cloud/kubernetes/operator/master/noelware-on-k8s.crds.yaml
```

And now, you can just install it with charted-server's Kubernetes CRD that was installed with Noelware's Kubernetes Operator:

```yaml
apiVersion: k8s.noelware.cloud/charted/v1alpha
kind: ChartedServer
metadata:
  name: charted-server
spec:
  image: cr.noelware.cloud/charted/server
  persistence:
    volumeClaim:
      name: charted-data
    filesystem:
      directory: /var/lib/noelware/charted/data
```
