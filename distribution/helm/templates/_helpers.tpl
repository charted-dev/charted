{{/*
~ 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
~ Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
~
~ Licensed under the Apache License, Version 2.0 (the "License");
~ you may not use this file except in compliance with the License.
~ You may obtain a copy of the License at
~
~    http://www.apache.org/licenses/LICENSE-2.0
~
~ Unless required by applicable law or agreed to in writing, software
~ distributed under the License is distributed on an "AS IS" BASIS,
~ WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
~ See the License for the specific language governing permissions and
~ limitations under the License.
*/}}

{{/*
Expand the name of the chart.
*/}}
{{- define "charted.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "charted.fullname" -}}
{{- if .Values.fullNameOverride -}}
{{- .Values.fullNameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- $name := default .Chart.Name .Values.nameOverride -}}
{{- if contains $name .Release.Name -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}
{{- end -}}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "charted.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "charted.labels" -}}
k8s.noelware.cloud/chart: {{ include "charted.chart" . }}
{{ include "charted.selectorLabels" . }}
{{- if .Chart.AppVersion }}
k8s.noelware.cloud/version: {{ .Chart.AppVersion | quote }}
{{- end }}
k8s.noelware.cloud/managed-by: {{ .Release.Name }}
{{- end }}

{{- define "charted.selectorLabels" -}}
k8s.noelware.cloud/name: {{ include "charted.name" . }}
k8s.noelware.cloud/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "charted.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "charted.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Default annotations
*/}}
{{- define "charted.defaultAnnotations" -}}
k8s.noelware.cloud/component: helm-registry
k8s.noelware.cloud/product: charted-server

{{- range $key, $val := .Values.annotations }}
    {{ $key }}: {{ $val | quote }}
{{- end }}
{{- end -}}

{{/*
Default Pod security context object
*/}}
{{- define "charted.defaultPodSecurityContext" -}}
enabled: true
fsGroup: 1001
seccompProfile:
  type: "RuntimeDefault"
{{- end -}}

{{/*
Default container security context object
*/}}
{{- define "charted.defaultContainerSecurityContext" -}}
enabled: true
runAsUser: 1001
runAsNonRoot: true
readOnlyRootFilesystem: false
allowPrivilegeEscalation: false
capabilities:
  drop: ["ALL"]
{{- end -}}

{{/*
Default server configuration
*/}}
{{- define "charted.defaultServerConfiguration" -}}
# refer to https://charts.noelware.org/docs/server/latest/self-hosting/configuration to see
# all the configuration options!
jwt_secret_key: ${JWT_SECRET_KEY}
{{- end -}}

{{/*
Default Logback configuration
*/}}
{{- define "charted.defaultLogbackConfiguration" -}}
# refer to https://charts.noelware.org/docs/server/latest/self-hosting/configuration to see
# all the configuration options!
charted.console.json=yes
charted.log.level=INFO
{{- end -}}
