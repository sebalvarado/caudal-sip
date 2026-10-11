output "project_id" {
  description = "Azure DevOps project ID, needed to import the GitHub App service connection"
  value       = azuredevops_project.this.id
}

output "pipeline_ci_id" {
  description = "CI pipeline definition ID"
  value       = azuredevops_build_definition.ci.id
}
