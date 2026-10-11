# The Azure DevOps project and the CI pipeline for this GitHub repository
# (ADR-0019). Credentials come from the environment and never from a file:
#
#   export AZDO_ORG_SERVICE_URL=https://dev.azure.com/<org>
#   export AZDO_PERSONAL_ACCESS_TOKEN=<PAT: Build read & execute, Service
#     Connections read & manage, Project & Team read, write & manage>
#
# First apply, in this order:
#   1. terraform init && terraform apply -target=azuredevops_project.this
#   2. Install the Azure Pipelines GitHub App on the repository
#      (https://github.com/apps/azure-pipelines) and choose this project. That
#      creates the GitHub App service connection.
#   3. terraform import azuredevops_serviceendpoint_github.app <project-id>/<connection-id>
#   4. terraform apply
#   5. On GitHub, require the CI checks on `main`.

terraform {
  required_version = ">= 1.6.0"

  required_providers {
    azuredevops = {
      source  = "microsoft/azuredevops"
      version = "~> 1.13"
    }
  }
}

provider "azuredevops" {}

resource "azuredevops_project" "this" {
  name               = var.project_name
  description        = "CI for ${var.github_repo_id}. Not affiliated with the Banco Central del Ecuador."
  visibility         = var.visibility
  version_control    = "Git"
  work_item_template = "Agile"

  # Code and issues stay on GitHub; this project only runs pipelines.
  features = {
    boards       = "disabled"
    repositories = "disabled"
    pipelines    = "enabled"
    testplans    = "disabled"
    artifacts    = "disabled"
  }
}

# Created by installing the Azure Pipelines GitHub App, which delivers push and
# pull_request events without repository webhooks. The provider cannot write
# its InstallationToken auth scheme, so it is imported and left alone.
resource "azuredevops_serviceendpoint_github" "app" {
  project_id            = azuredevops_project.this.id
  service_endpoint_name = var.github_app_service_connection_name

  lifecycle {
    ignore_changes = [description]
  }
}

resource "azuredevops_build_definition" "ci" {
  project_id = azuredevops_project.this.id
  name       = "CI"

  ci_trigger {
    use_yaml = true
  }

  # Must be declared even with use_yaml, or the GitHub App never subscribes to
  # pull_request events and the check stays queued. CI holds no secrets, so
  # fork pull requests are built.
  pull_request_trigger {
    use_yaml       = true
    initial_branch = var.default_branch
    forks {
      enabled       = true
      share_secrets = false
    }
  }

  repository {
    repo_type             = "GitHub"
    repo_id               = var.github_repo_id
    branch_name           = var.default_branch
    yml_path              = "azure-pipelines-ci.yml"
    service_connection_id = azuredevops_serviceendpoint_github.app.id
  }
}
