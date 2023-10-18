# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

terraform {
  required_providers {
    helm = {
      source  = "hashicorp/helm"
      version = "2.11.0"
    }
  }
}

provider "helm" {
  kubernetes {
    config_context = var.context
    config_path    = var.kubeconfig
  }
}

resource "kubernetes_namespace" "charted" {
  metadata {
    name = "charted"
    annotations = {
      "k8s.noelware.cloud/managed-by" = "Terraform"
      "k8s.noelware.cloud/product"    = "charted"
    }
  }
}

resource "helm_release" "postgresql" {
  repository = "oci://registry-1.docker.io/bitnamicharts"
  namespace  = "charted"
  depends_on = [kubernetes_namespace.charted]
  version    = "11.9.2"
  values     = ["./charts/postgresql.yaml"]
  atomic     = true
  chart      = "postgresql-ha"
  name       = "postgresql-ha"
  wait       = true
}

resource "helm_release" "redis" {
  repository = "oci://registry-1.docker.io/bitnamicharts"
  depends_on = [kubernetes_namespace.charted]
  namespace  = "charted"
  version    = "18.0.1"
  values     = ["./charts/redis.yaml"]
  atomic     = true
  chart      = "redis"
  name       = "redis"
  wait       = true
}

resource "helm_release" "logstash" {
  repository = "https://charts.noelware.org/~/noelware"
  depends_on = [kubernetes_namespace.charted]
  namespace  = "charted"
  values     = ["./charts/logstash.yaml"]
  version    = "8.9.1"
  atomic     = true
  chart      = "logstash"
  name       = "logstash"
  wait       = true
}

# Petal is Noelware's load balancing service to throttle logs to Logstash
# via Redpanda clusters, so in a structure of:
#
#   charted ~> petal ~> redpanda ~> logstash
resource "helm_release" "petal" {
  repository = "https://charts.noelware.org/~/noelware"
  depends_on = [helm_release.logstash, helm_release.redpanda, kubernetes_namespace.charted]
  namespace  = "charted"
  version    = "0.1.0-beta"
  values     = ["./charts/petal.yaml"]
  atomic     = true
  chart      = "petal"
  name       = "petal"
  wait       = true
}

resource "helm_release" "redpanda" {
  repository = "https://charts.noelware.org/~/noelware"
  depends_on = [kubernetes_namespace.charted]
  namespace  = "charted"
  version    = "23.2.8"
  values     = ["./charts/redpanda.yaml"]
  atomic     = true
  chart      = "redpanda"
  name       = "redpanda"
  wait       = true
}

resource "helm_release" "charted-emails" {
  repository = "https://charts.noelware.org/~/charted"
  depends_on = [kubernetes_namespace.charted]
  namespace  = "charted"
  version    = "0.2.0-beta"
  atomic     = true
  values     = ["./charts/emails.yaml"]
  chart      = "emails"
  name       = "emails"
  wait       = true
}

resource "helm_release" "charted-server" {
  repository = "https://charts.noelware.org/~/charted"
  depends_on = [kubernetes_namespace.charted, helm_release.logstash, helm_release.helm_release.petal, helm_release.postgresql-ha, helm_release.redis-sentinel, helm_release.redpanda]
  namespace  = "charted"
  values     = ["./charts/charted.yaml"]
  version    = "0.1.0-beta"
  atomic     = true
  chart      = "server"
  wait       = true
  name       = "charted"
}
