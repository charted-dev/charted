# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
# Copyright 2022-2023 noelware <team@noelware.org>
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
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.23.0"
    }

    helm = {
      source  = "hashicorp/helm"
      version = "2.11.0"
    }
  }
}

provider "kubernetes" {
  config_context = var.context != "" ? var.context : null
  config_path    = var.kubeconfig
}

# Since we don't have the official Helm chart released, for now, this will be the actual
# deployment that we might use for the Helm chart.
resource "kubernetes_deployment" "charted" {
  metadata {
    name      = "charted-server"
    namespace = "noelware"
    annotations = {
      "k8s.noelware.cloud/component" = "helm-registry"
      "k8s.noelware.cloud/version"   = "0.4.0-nightly"
    }

    labels = {
      "k8s.noelware.cloud/service" = "charted-server"
      "k8s.noelware.cloud/vendor"  = "noelware"
    }
  }

  spec {
    replicas = 1
    selector {
      match_labels = {
        "k8s.noelware.cloud/service" = "charted-server"
        "k8s.noelware.cloud/vendor"  = "noelware"
      }
    }

    template {
      metadata {
        labels = {
          "k8s.noelware.cloud/service" = "charted-server"
          "k8s.noelware.cloud/vendor"  = "noelware"
        }
      }

      spec {
        volume {
          name = "config"
          config_map {
            default_mode = "0420"
            name         = "charted-config"
          }
        }

        # since im only doing prod testing, this will be a temporary PVC that
        # wont affect prod
        volume {
          name = "storage"
          persistent_volume_claim {
            claim_name = "charted-data"
          }
        }

        container {
          image             = "ghcr.io/charted-dev/charted:0.4.0-nightly-alpine"
          name              = "charted-server"
          image_pull_policy = "IfNotPresent"

          resources {
            limits = {
              cpu    = "1500m"
              memory = "2Gi"
            }

            requests = {
              cpu    = "10m"
              memory = "512Mi"
            }
          }

          env {
            name  = "CHARTED_CONFIG_PATH"
            value = "/app/noelware/charted/config/charted.yaml"
          }

          env {
            name = "WINTERFOX_DEDI_NODE"
            value_from {
              field_ref {
                api_version = "v1"
                field_path  = "spec.nodeName"
              }
            }
          }

          env {
            name  = "CHARTED_JAVA_OPTS"
            value = "-Xmx4096m -Xms1024m"
          }

          port {
            container_port = 3651
            name           = "http"
            protocol       = "TCP"
          }

          liveness_probe {
            http_get {
              path   = "/health"
              port   = 3651
              scheme = "HTTP"
            }

            failure_threshold     = 3
            initial_delay_seconds = 10
            period_seconds        = 30
            success_threshold     = 1
            timeout_seconds       = 1
          }

          readiness_probe {
            http_get {
              path   = "/health"
              port   = 3651
              scheme = "HTTP"
            }

            failure_threshold     = 3
            initial_delay_seconds = 10
            period_seconds        = 30
            success_threshold     = 1
            timeout_seconds       = 1
          }

          volume_mount {
            mount_path = "/app/noelware/charted/config/charted.yaml"
            name       = "config"
            sub_path   = "charted.yaml"
            read_only  = true
          }

          volume_mount {
            mount_path = "/app/noelware/charted/config/logback.properties"
            name       = "config"
            sub_path   = "logback.properties"
            read_only  = true
          }
        }
      }
    }
  }
}

resource "kubernetes_persistent_volume_claim" "charted-data" {
  metadata {
    name      = "charted-data"
    namespace = "noelware"
    annotations = {
      "k8s.noelware.cloud/component" = "helm-registry"
      "k8s.noelware.cloud/version"   = "0.4.0-nightly"
    }

    labels = {
      "k8s.noelware.cloud/service" = "charted-server"
      "k8s.noelware.cloud/vendor"  = "noelware"
    }
  }

  spec {
    access_modes = ["ReadWriteMany"]
    resources {
      requests = {
        "storage" = "1Gi"
      }
    }
  }
}

resource "kubernetes_ingress_v1" "charted-ingress" {
  metadata {
    name      = "charted-server"
    namespace = "noelware"
    annotations = {
      "k8s.noelware.cloud/component" = "helm-registry"
      "k8s.noelware.cloud/version"   = "0.4.0-nightly"
    }

    labels = {
      "k8s.noelware.cloud/service" = "charted-server"
      "k8s.noelware.cloud/vendor"  = "noelware"
    }
  }

  spec {
    rule {
      host = "charts.noelware.org"
      http {
        path {
          path      = "/api"
          path_type = "ImplementationSpecific"
          backend {
            service {
              name = "charted-server"
              port {
                number = 3651
              }
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "charted-service" {
  metadata {
    name      = "charted-server"
    namespace = "noelware"
    annotations = {
      "k8s.noelware.cloud/component" = "helm-registry"
      "k8s.noelware.cloud/version"   = "0.4.0-nightly"
    }

    labels = {
      "k8s.noelware.cloud/service" = "charted-server"
      "k8s.noelware.cloud/vendor"  = "noelware"
    }
  }

  spec {
    type = "ClusterIP"
    selector = {
      "k8s.noelware.cloud/service" = "charted-server"
      "k8s.noelware.cloud/vendor"  = "noelware"
    }

    port {
      target_port = 3651
      name        = "http"
      port        = 3651
    }
  }
}

# https://github.com/bitnami/charts/tree/main/bitnami/postgresql
resource "helm_release" "postgres" {
  repository = "https://charts.bitnami.com/bitnami"
  chart      = "postgresql"
  name       = "postgresql"

  set {
    name  = "metrics.enabled"
    value = "true"
  }
}

# https://github.com/bitnami/charts/tree/main/bitnami/redis
resource "helm_release" "redis" {
  repository = "https://charts.bitnami.com/bitnami"
  chart      = "redis"
  name       = "redis"

  set {
    name  = "sentinel.enabled"
    value = "true"
  }

  set {
    name  = "metrics.enabled"
    value = "true"
  }
}
