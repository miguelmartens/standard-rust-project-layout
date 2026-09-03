# `.github/`

GitHub-specific configuration. Nothing here is a Rust or Cargo concern; the
directory exists because GitHub looks for these paths.

- [`workflows/ci.yaml`](workflows/ci.yaml) — the CI pipeline: `check` (fmt,
  clippy, tests, doctests, rustdoc), `msrv`, and `deny`.
- [`dependabot.yml`](dependabot.yml) — weekly dependency and action updates.

Also conventionally found here, and deliberately absent from this example
repository: `CODEOWNERS`, `PULL_REQUEST_TEMPLATE.md`, `ISSUE_TEMPLATE/`,
`SECURITY.md`, `FUNDING.yml`.

## On file extensions

`ci.yaml` uses the four-letter extension. `dependabot.yml` does not, because
GitHub documents that exact filename and a misnamed Dependabot configuration
fails *silently* — no error, no updates, no signal. Consistency loses to a
platform requirement.

## If you are not on GitHub

Delete this directory and put the equivalent where your CI system looks:
`.gitlab-ci.yml`, `azure-pipelines.yaml`, `.circleci/config.yml`,
`.woodpecker.yaml`. The three jobs are what matter, not the syntax — and
`cargo xtask ci` means most of the pipeline is one line whichever system you use.
