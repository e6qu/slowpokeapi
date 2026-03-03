# SlowPokeAPI Helm Chart

Currency exchange rate API with distributed sync for Kubernetes.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+

## Installing the Chart

```bash
helm install slowpokeapi ./deploy/helm/slowpokeapi
```

## Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of replicas | `3` |
| `image.repository` | Image repository | `ghcr.io/e6qu/slowpokeapi` |
| `image.tag` | Image tag | `""` (uses appVersion) |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `service.type` | Service type | `ClusterIP` |
| `service.port` | HTTP port | `8080` |
| `service.syncPort` | Sync port | `8081` |
| `ingress.enabled` | Enable ingress | `false` |
| `autoscaling.enabled` | Enable HPA | `false` |
| `persistence.enabled` | Enable persistence | `false` |
| `statefulSet.enabled` | Use StatefulSet instead of Deployment | `false` |
| `serviceMonitor.enabled` | Enable Prometheus ServiceMonitor | `false` |
| `sync.enabled` | Enable CRDT sync | `true` |
| `config.logLevel` | Log level | `info` |
| `config.cacheTtlSeconds` | Cache TTL | `3600` |

## Examples

### Development

```bash
helm install slowpokeapi ./deploy/helm/slowpokeapi \
  --set replicaCount=1 \
  --set config.logLevel=debug
```

### Production with Ingress

```bash
helm install slowpokeapi ./deploy/helm/slowpokeapi \
  -f ./deploy/helm/slowpokeapi/values-prod.yaml
```

### StatefulSet with Persistence

```bash
helm install slowpokeapi ./deploy/helm/slowpokeapi \
  --set statefulSet.enabled=true \
  --set persistence.enabled=true \
  --set persistence.storageClass=gp3 \
  --set persistence.size=5Gi
```

## Upgrading

```bash
helm upgrade slowpokeapi ./deploy/helm/slowpokeapi \
  --set image.tag=1.1.0
```

## Uninstalling

```bash
helm uninstall slowpokeapi
```
