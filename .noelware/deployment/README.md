This directory is meant for our actual infrastructure for deploying [charts.noelware.org](https://charts.noelware.org). It is managed by
Terraform.

This is only ran on the Release (Stable) CI pipeline when the repository is `charted-dev/charted` so forks can't run our Terraform
deployment if they don't want to.

You are required to have Terraform installed to deploy & handle linting with Spotless.
