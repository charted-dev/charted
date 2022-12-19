{{/*
📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
Copyright 2022-2023 Noelware <team@noelware.org>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/}}

{{/*
Expand the name of the chart.
*/}}
{{- define "charted.name" -}}
{{- default .Chart.Name .Values.global.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "charted.fullname" -}}
{{- if .Values.global.fullNameOverride -}}
{{- .Values.global.fullNameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- $name := default .Chart.Name .Values.global.nameOverride -}}
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
helm.sh/chart: {{ include "charted.chart" . }}
{{ include "charted.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "charted.selectorLabels" -}}
app.kubernetes.io/name: {{ include "charted.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
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
k8s.noelware.cloud/component: api-server
k8s.noelware.cloud/product: charted-server
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

{{- define "charted.postgres.fullname" -}}
{{- include "common.names.dependency.fullname" (dict "chartName" "postgresql" "chartValues" .Values.global.postgres "context" $) -}}
{{- end -}}

{{- define "charted.postgres.host" -}}
  {{- if .Values.global.postgres.enabled }}
    {{- if eq .Values.global.postgres.architecture "replication" -}}
      {{- printf "%s-%s" (include "charted.postgres.fullname" .) "primary" | trunc 64 | trimSuffix "-" -}}
    {{- else -}}
      {{- print (include "charted.postgres.fullname" .) -}}
    {{- end -}}
  {{- else }}
    {{- print .Values.external.postgres.host -}}
  {{- end }}
{{- end }}

{{- define "charted.postgres.port" -}}
  {{- if .Values.global.postgres.enabled -}}
    {{- print .Values.global.postgres.primary.service.ports.postgresql -}}
  {{- else -}}
    {{- print .Values.external.postgres.port -}}
  {{- end -}}
{{- end -}}

{{- define "charted.postgres.database" -}}
  {{- if .Values.global.postgres.enabled -}}
    {{- print "charted" -}}
  {{- else -}}
    {{- print .Values.external.postgres.database -}}
  {{- end -}}
{{- end -}}

{{- define "charted.postgres.username" -}}
  {{- if .Values.global.postgres.enabled -}}
    {{- print .Values.global.postgres.auth.username -}}
  {{- else -}}
    {{- print .Values.external.postgres.username -}}
  {{- end -}}
{{- end -}}

{{- define "charted.postgres.password" -}}
  {{- if .Values.global.postgres.enabled -}}
    {{- print .Values.global.postgres.auth.password -}}
  {{- else -}}
    {{- print .Values.external.postgres.password -}}
  {{- end -}}
{{- end -}}

{{- define "charted.redis.fullname" -}}
{{- include "common.names.dependency.fullname" (dict "chartName" "redis" "chartValues" .Values.global.redis "context" $) -}}
{{- end -}}

{{- define "charted.redis.host" -}}
  {{- if .Values.global.redis.enabled }}
    {{- printf "%s-master" (include "charted.redis.fullname" .) | trunc 63 | trimSuffix "-" -}}
  {{- else }}
    {{- print .Values.external.redis.host -}}
  {{- end }}
{{- end }}

{{- define "charted.redis.port" -}}
  {{- if .Values.global.redis.enabled -}}
    {{- print .Values.global.redis.master.service.ports.redis -}}
  {{- else -}}
    {{- print .Values.external.redis.port -}}
  {{- end -}}
{{- end -}}

{{- define "charted.redis.db" -}}
  {{- if .Values.global.redis.enabled -}}
    {{- print 8 -}}
  {{- else -}}
    {{- print .Values.external.redis.database -}}
  {{- end -}}
{{- end -}}

{{- define "charted.clickhouse.fullname" -}}
{{- include "common.names.dependency.fullname" (dict "chartName" "clickhouse" "chartValues" .Values.global.clickhouse "context" $) -}}
{{- end -}}

{{- define "charted.clickhouse.hosts" -}}
  {{- if .Values.global.clickhouse.enabled -}}
    {{ printf "%s,%s,%s" (printf "%s-%s" (include "charted.clickhouse.fullname") 0) (printf "%s-%s" (include "charted.clickhouse.fullname") 1) (printf "%s-%s" (include "charted.clickhouse.fullname") 2) }}
  {{- end -}}
{{- end -}}
