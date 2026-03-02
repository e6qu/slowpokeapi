# Helm Chart Specification

## Overview

Helm chart for deploying SlowPokeAPI to Kubernetes with support for both Deployment and StatefulSet modes.

## Chart Structure

```
deploy/helm/slowpokeapi/
├── Chart.yaml
├── values.yaml
├── values-dev.yaml
├── values-prod.yaml
├── templates/
│   ├── _helpers.tpl
│   ├── deployment.yaml
│   ├── statefulset.yaml
│   ├── service.yaml
│   ├── serviceaccount.yaml
│   ├── configmap.yaml
│   ├── secret.yaml
│   ├── ingress.yaml
│   ├── hpa.yaml
│   ├── pdb.yaml
│   └── servicemonitor.yaml
└── README.md
```

## Chart.yaml

```yaml
apiVersion: v2
name: slowpokeapi
description: Currency exchange rate API with distributed sync
type: application
version: 1.0.0
appVersion: "1.0.0"
keywords:
  - currency
  - exchange-rates
  - api
home: https://github.com/e6qu/slowpokeapi
sources:
  - https://github.com/e6qu/slowpokeapi
maintainers:
  - name: e6qu
    url: https://github.com/e6qu
```

## values.yaml

```yaml
replicaCount: 3

image:
  repository: ghcr.io/e6qu/slowpokeapi
  pullPolicy: IfNotPresent
  tag: ""

imagePullSecrets: []
nameOverride: ""
fullnameOverride: ""

serviceAccount:
  create: true
  annotations: {}
  name: ""

podAnnotations:
  prometheus.io/scrape: "true"
  prometheus.io/port: "8080"
  prometheus.io/path: "/metrics"

podSecurityContext:
  runAsNonRoot: true
  runAsUser: 1000
  fsGroup: 1000

securityContext:
  allowPrivilegeEscalation: false
  capabilities:
    drop:
      - ALL
  readOnlyRootFilesystem: true

service:
  type: ClusterIP
  port: 8080
  syncPort: 8081

ingress:
  enabled: false
  className: ""
  annotations: {}
  hosts:
    - host: api.slowpokeapi.local
      paths:
        - path: /
          pathType: Prefix
  tls: []

resources:
  limits:
    cpu: 500m
    memory: 256Mi
  requests:
    cpu: 100m
    memory: 128Mi

autoscaling:
  enabled: false
  minReplicas: 2
  maxReplicas: 10
  targetCPUUtilizationPercentage: 80
  targetMemoryUtilizationPercentage: 80

nodeSelector: {}

tolerations: []

affinity: {}

pdb:
  enabled: false
  minAvailable: 1
  maxUnavailable: ""

persistence:
  enabled: false
  storageClass: ""
  accessMode: ReadWriteOnce
  size: 1Gi

config:
  logLevel: info
  cacheTtlSeconds: 3600
  upstreamTimeoutSeconds: 10

sync:
  enabled: true
  gossipIntervalSeconds: 5
  fanout: 3
  discovery:
    method: dns
    dnsName: ""
    staticPeers: []

upstream:
  frankfurter:
    enabled: true
    baseUrl: https://api.frankfurter.app
  fawaz:
    enabled: true
    baseUrl: https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1
  coingecko:
    enabled: true
    baseUrl: https://api.coingecko.com/api/v3

auth:
  enabled: false
  existingSecret: ""

serviceMonitor:
  enabled: false
  namespace: ""
  interval: 30s
  scrapeTimeout: 10s
  labels: {}

statefulSet:
  enabled: false
```

## Templates

### deployment.yaml

```yaml
{{- if not .Values.statefulSet.enabled }}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ include "slowpokeapi.fullname" . }}
  labels:
    {{- include "slowpokeapi.labels" . | nindent 4 }}
spec:
  {{- if not .Values.autoscaling.enabled }}
  replicas: {{ .Values.replicaCount }}
  {{- end }}
  selector:
    matchLabels:
      {{- include "slowpokeapi.selectorLabels" . | nindent 6 }}
  template:
    metadata:
      annotations:
        checksum/config: {{ include (print $.Template.BasePath "/configmap.yaml") . | sha256sum }}
        {{- with .Values.podAnnotations }}
        {{- toYaml . | nindent 8 }}
        {{- end }}
      labels:
        {{- include "slowpokeapi.selectorLabels" . | nindent 8 }}
    spec:
      {{- with .Values.imagePullSecrets }}
      imagePullSecrets:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      serviceAccountName: {{ include "slowpokeapi.serviceAccountName" . }}
      securityContext:
        {{- toYaml .Values.podSecurityContext | nindent 8 }}
      containers:
        - name: {{ .Chart.Name }}
          securityContext:
            {{- toYaml .Values.securityContext | nindent 12 }}
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag | default .Chart.AppVersion }}"
          imagePullPolicy: {{ .Values.image.pullPolicy }}
          ports:
            - name: http
              containerPort: 8080
              protocol: TCP
            - name: sync
              containerPort: 8081
              protocol: TCP
          env:
            - name: SLOWPOKEAPI_LOG_LEVEL
              value: {{ .Values.config.logLevel }}
            - name: SLOWPOKEAPI_CACHE_TTL_SECONDS
              value: {{ .Values.config.cacheTtlSeconds | quote }}
            - name: SLOWPOKEAPI_UPSTREAM_TIMEOUT_SECONDS
              value: {{ .Values.config.upstreamTimeoutSeconds | quote }}
            - name: SLOWPOKEAPI_SYNC_ENABLED
              value: {{ .Values.sync.enabled | quote }}
            {{- if .Values.sync.enabled }}
            - name: SLOWPOKEAPI_SYNC_DISCOVERY
              value: {{ .Values.sync.discovery.method }}
            - name: SLOWPOKEAPI_SYNC_DNS_NAME
              value: {{ include "slowpokeapi.fullname" . }}-headless.{{ .Release.Namespace }}.svc.cluster.local
            {{- end }}
            {{- if .Values.persistence.enabled }}
            - name: SLOWPOKEAPI_STORAGE_PATH
              value: /data/slowpokeapi.db
            {{- end }}
          envFrom:
            - configMapRef:
                name: {{ include "slowpokeapi.fullname" . }}
          livenessProbe:
            httpGet:
              path: /healthz
              port: http
            initialDelaySeconds: 5
            periodSeconds: 10
          readinessProbe:
            httpGet:
              path: /readyz
              port: http
            initialDelaySeconds: 10
            periodSeconds: 5
          startupProbe:
            httpGet:
              path: /livez
              port: http
            initialDelaySeconds: 0
            periodSeconds: 1
            failureThreshold: 30
          resources:
            {{- toYaml .Values.resources | nindent 12 }}
          {{- if .Values.persistence.enabled }}
          volumeMounts:
            - name: data
              mountPath: /data
          {{- end }}
      {{- if .Values.persistence.enabled }}
      volumes:
        - name: data
          emptyDir: {}
      {{- end }}
      {{- with .Values.nodeSelector }}
      nodeSelector:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with .Values.affinity }}
      affinity:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      {{- with .Values.tolerations }}
      tolerations:
        {{- toYaml . | nindent 8 }}
      {{- end }}
{{- end }}
```

### statefulset.yaml

```yaml
{{- if .Values.statefulSet.enabled }}
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: {{ include "slowpokeapi.fullname" . }}
  labels:
    {{- include "slowpokeapi.labels" . | nindent 4 }}
spec:
  serviceName: {{ include "slowpokeapi.fullname" . }}-headless
  {{- if not .Values.autoscaling.enabled }}
  replicas: {{ .Values.replicaCount }}
  {{- end }}
  selector:
    matchLabels:
      {{- include "slowpokeapi.selectorLabels" . | nindent 6 }}
  template:
    metadata:
      annotations:
        checksum/config: {{ include (print $.Template.BasePath "/configmap.yaml") . | sha256sum }}
        {{- with .Values.podAnnotations }}
        {{- toYaml . | nindent 8 }}
        {{- end }}
      labels:
        {{- include "slowpokeapi.selectorLabels" . | nindent 8 }}
    spec:
      {{- with .Values.imagePullSecrets }}
      imagePullSecrets:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      serviceAccountName: {{ include "slowpokeapi.serviceAccountName" . }}
      securityContext:
        {{- toYaml .Values.podSecurityContext | nindent 8 }}
      containers:
        - name: {{ .Chart.Name }}
          securityContext:
            {{- toYaml .Values.securityContext | nindent 12 }}
          image: "{{ .Values.image.repository }}:{{ .Values.image.tag | default .Chart.AppVersion }}"
          imagePullPolicy: {{ .Values.image.pullPolicy }}
          ports:
            - name: http
              containerPort: 8080
              protocol: TCP
            - name: sync
              containerPort: 8081
              protocol: TCP
          env:
            - name: POD_NAME
              valueFrom:
                fieldRef:
                  fieldPath: metadata.name
            - name: SLOWPOKEAPI_STORAGE_PATH
              value: /data/slowpokeapi.db
            - name: SLOWPOKEAPI_SYNC_DISCOVERY
              value: dns
            - name: SLOWPOKEAPI_SYNC_DNS_NAME
              value: {{ include "slowpokeapi.fullname" . }}-headless.{{ .Release.Namespace }}.svc.cluster.local
          envFrom:
            - configMapRef:
                name: {{ include "slowpokeapi.fullname" . }}
          livenessProbe:
            httpGet:
              path: /healthz
              port: http
          readinessProbe:
            httpGet:
              path: /readyz
              port: http
          resources:
            {{- toYaml .Values.resources | nindent 12 }}
          volumeMounts:
            - name: data
              mountPath: /data
  volumeClaimTemplates:
    - metadata:
        name: data
      spec:
        accessModes:
          - {{ .Values.persistence.accessMode }}
        {{- if .Values.persistence.storageClass }}
        storageClassName: {{ .Values.persistence.storageClass }}
        {{- end }}
        resources:
          requests:
            storage: {{ .Values.persistence.size }}
{{- end }}
```

### service.yaml

```yaml
apiVersion: v1
kind: Service
metadata:
  name: {{ include "slowpokeapi.fullname" . }}
  labels:
    {{- include "slowpokeapi.labels" . | nindent 4 }}
spec:
  type: {{ .Values.service.type }}
  ports:
    - port: {{ .Values.service.port }}
      targetPort: http
      protocol: TCP
      name: http
  selector:
    {{- include "slowpokeapi.selectorLabels" . | nindent 4 }}
---
apiVersion: v1
kind: Service
metadata:
  name: {{ include "slowpokeapi.fullname" . }}-headless
  labels:
    {{- include "slowpokeapi.labels" . | nindent 4 }}
spec:
  type: ClusterIP
  clusterIP: None
  ports:
    - port: {{ .Values.service.port }}
      targetPort: http
      protocol: TCP
      name: http
    - port: {{ .Values.service.syncPort }}
      targetPort: sync
      protocol: TCP
      name: sync
  selector:
    {{- include "slowpokeapi.selectorLabels" . | nindent 4 }}
```

### servicemonitor.yaml

```yaml
{{- if .Values.serviceMonitor.enabled }}
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: {{ include "slowpokeapi.fullname" . }}
  labels:
    {{- include "slowpokeapi.labels" . | nindent 4 }}
    {{- with .Values.serviceMonitor.labels }}
    {{- toYaml . | nindent 4 }}
    {{- end }}
spec:
  selector:
    matchLabels:
      {{- include "slowpokeapi.selectorLabels" . | nindent 6 }}
  endpoints:
    - port: http
      path: /metrics
      interval: {{ .Values.serviceMonitor.interval }}
      scrapeTimeout: {{ .Values.serviceMonitor.scrapeTimeout }}
{{- end }}
```

## Usage Examples

### Install with Defaults

```bash
helm install slowpokeapi ./deploy/helm/slowpokeapi
```

### Install with StatefulSet and Persistence

```bash
helm install slowpokeapi ./deploy/helm/slowpokeapi \
    --set statefulSet.enabled=true \
    --set persistence.enabled=true \
    --set persistence.storageClass=gp3 \
    --set persistence.size=5Gi
```

### Install for Production

```bash
helm install slowpokeapi ./deploy/helm/slowpokeapi \
    -f ./deploy/helm/slowpokeapi/values-prod.yaml \
    --set replicaCount=5 \
    --set autoscaling.enabled=true \
    --set serviceMonitor.enabled=true
```

### Upgrade

```bash
helm upgrade slowpokeapi ./deploy/helm/slowpokeapi \
    --set image.tag=1.1.0
```

## values-prod.yaml

```yaml
replicaCount: 3

resources:
  limits:
    cpu: 1000m
    memory: 512Mi
  requests:
    cpu: 200m
    memory: 256Mi

autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10

pdb:
  enabled: true
  minAvailable: 2

ingress:
  enabled: true
  className: nginx
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
  hosts:
    - host: api.slowpokeapi.io
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: slowpokeapi-tls
      hosts:
        - api.slowpokeapi.io

serviceMonitor:
  enabled: true
  interval: 15s

config:
  logLevel: warn
  cacheTtlSeconds: 1800
```
