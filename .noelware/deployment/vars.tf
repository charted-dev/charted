# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

variable "kubeconfig" {
  description = "Location to a ~/.kube/config file"
  default     = "~/.kube/config"
  sensitive   = true
  type        = string
}

variable "context" {
  description = "Kubernetes context to live in"
  default     = "default"
  sensitive   = true
  type        = string
}

variable "insecure" {
  description = "whether or not if the connection towards the Kubernetes server should not be secured, this shouldn't be used!"
  default     = false
  type        = bool
}

variable "elastic" {
  description = "whether or not if Elastic products (like Elasticsearch) should be deployed as well"
  default     = true
  type        = bool
}

variable "redpanda" {
  description = "whether or not if a Redpanda cluster should be deployed"
  default     = true
  type        = bool
}