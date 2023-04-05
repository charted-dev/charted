This directory is meant for our actual infrastructure for deploying [charts.noelware.org](https://charts.noelware.org). It is managed by
Terraform.

This is only ran on the Release (Stable) CI pipeline when the repository is `charted-dev/charted` so forks can't run our Terraform
deployment if they don't want to.

You are required to have Terraform installed to deploy & handle linting with Spotless.

## How to deploy

Create a file in this directory called `context.tfvars` with the following content:

```tfvars
kubeconfig = "some path to kubeconfig"
context = "some context to use, omit for default"
```
