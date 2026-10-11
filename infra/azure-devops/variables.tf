variable "project_name" {
  description = "Azure DevOps project name"
  type        = string
  default     = "caudal"
}

variable "visibility" {
  description = "public lets fork contributors read CI logs and gets the free open-source parallel jobs; the organization must allow public projects"
  type        = string
  default     = "public"
}

variable "github_repo_id" {
  description = "GitHub repository in owner/repo form"
  type        = string
  default     = "sebalvarado/caudal-sip"
}

variable "github_app_service_connection_name" {
  description = "Name of the service connection the Azure Pipelines GitHub App created; must match exactly"
  type        = string
  default     = "sebalvarado"
}

variable "default_branch" {
  description = "Branch the pipeline definition reads its YAML from"
  type        = string
  default     = "refs/heads/main"
}
