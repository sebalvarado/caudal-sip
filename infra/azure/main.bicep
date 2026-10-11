// The public demo: one container running `caudal serve`, scaled to zero when idle.
// Deployed by .github/workflows/deploy.yml with `az deployment group create`.

@description('Name of the container app, also the prefix of the environment and logs.')
param name string = 'caudal-demo'

param location string = resourceGroup().location

@description('Public image, e.g. ghcr.io/<owner>/caudal-demo:<sha>.')
param image string

@description('The only origin allowed by CORS: the site.')
param siteOrigin string = 'https://caudal-sip.pages.dev'

@description('Seconds between resets to the seed.')
param resetSeconds int = 900

resource logs 'Microsoft.OperationalInsights/workspaces@2023-09-01' = {
  name: '${name}-logs'
  location: location
  properties: {
    sku: { name: 'PerGB2018' }
    retentionInDays: 30
  }
}

resource env 'Microsoft.App/managedEnvironments@2024-03-01' = {
  name: '${name}-env'
  location: location
  properties: {
    appLogsConfiguration: {
      destination: 'log-analytics'
      logAnalyticsConfiguration: {
        customerId: logs.properties.customerId
        sharedKey: logs.listKeys().primarySharedKey
      }
    }
  }
}

resource app 'Microsoft.App/containerApps@2024-03-01' = {
  name: name
  location: location
  properties: {
    managedEnvironmentId: env.id
    configuration: {
      ingress: {
        external: true
        targetPort: 8080
        transport: 'http'
        allowInsecure: false
      }
    }
    template: {
      containers: [
        {
          name: 'caudal'
          image: image
          args: ['serve']
          resources: { cpu: json('0.25'), memory: '0.5Gi' }
          env: [
            { name: 'PORT', value: '8080' }
            { name: 'CAUDAL_DEMO_ORIGIN', value: siteOrigin }
            { name: 'CAUDAL_DEMO_RESET_SECS', value: string(resetSeconds) }
          ]
        }
      ]
      scale: {
        minReplicas: 0
        maxReplicas: 1
        rules: [
          { name: 'http', http: { metadata: { concurrentRequests: '50' } } }
        ]
      }
    }
  }
}

output url string = 'https://${app.properties.configuration.ingress.fqdn}'
