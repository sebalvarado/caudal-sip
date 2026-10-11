# infra/azure — the public demo host

One Azure Container App running `caudal serve`, scaled to zero when idle, deployed on every merge to `main` by [`deploy.yml`](../../.github/workflows/deploy.yml). Decided in [ADR-0020](../../docs/architecture/adr/0020-azure-container-apps-demo-host.md); the API it serves is bounded by [ADR-0021](../../docs/architecture/adr/0021-public-demo-api.md). Nothing outside this directory and that workflow knows about Azure.

## One-time setup

No secret is stored anywhere. The workflow signs in with OIDC.

1. Create a resource group, e.g. `caudal-demo` in `eastus2`.
2. Create an app registration (or user-assigned identity) and give it **Contributor** on that resource group.
3. Add a federated credential to it: issuer `https://token.actions.githubusercontent.com`, subject `repo:<owner>/caudal-sip:ref:refs/heads/main`, audience `api://AzureADTokenExchange`.
4. After the first `image` job runs, make the `caudal-demo` package on GHCR **public**, so the container app pulls it without registry credentials.
5. Set these repository **variables** (not secrets — they are identifiers):

| Variable | Value |
|---|---|
| `AZURE_CLIENT_ID` | The app registration's client id |
| `AZURE_TENANT_ID` | The tenant id |
| `AZURE_SUBSCRIPTION_ID` | The subscription id |
| `AZURE_RESOURCE_GROUP` | The resource group |
| `AZURE_APP_NAME` | Optional, defaults to `caudal-demo` |
| `SITE_URL` | Optional, the site origin allowed by CORS |

Until `AZURE_CLIENT_ID` is set, the deploy job is skipped and nothing fails.

6. Put the URL from the job summary in the site's `PUBLIC_DEMO_API` build variable, so the demo page knows where to call.

## By hand

```bash
az deployment group create -g caudal-demo -f infra/azure/main.bicep -p image=ghcr.io/<owner>/caudal-demo:latest
```
