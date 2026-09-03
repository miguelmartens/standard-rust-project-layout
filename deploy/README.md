# `deploy/`

Container images, Kubernetes manifests, Helm charts, Terraform, systemd units,
`docker-compose` files — how the built artefact is packaged and shipped.

**Rust has no convention for this directory.** Cargo does not know it exists.
`deploy/`, `deployments/`, `infra/`, `ops/` and `k8s/` are all in common use and
none is more correct than the others. Pick one, and pick it once.

## What belongs here

- `Dockerfile` / `Containerfile` and `.dockerignore`
- Kubernetes manifests, Helm charts, Kustomize overlays
- Terraform, Pulumi, CloudFormation
- systemd unit files, packaging metadata for `deb`/`rpm`

## What does not

- **CI workflows.** Those live where the CI system looks for them, which for
  GitHub Actions is `.github/workflows/`.
- **Build automation.** That is [`../xtask/`](../xtask/).
- **Secrets.** Not here, not anywhere in the repository. Manifests reference
  secrets; they do not contain them.

## Delete this directory if you are not deploying anything

An empty `deploy/` containing only a README is worse than no `deploy/` at all:
it implies a deployment story exists and sends readers looking for one. The same
goes for [`assets/`](../assets/), [`scripts/`](../scripts/) and
[`docs/adr/`](../docs/adr/). Directories in this repository are a menu, not a
checklist.
