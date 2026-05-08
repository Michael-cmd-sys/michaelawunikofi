# GitHub Pages Deployment Design

## Summary

This change will add automated GitHub Pages deployment for the Dioxus 0.7 portfolio site using `DioxusLabs/deploy-action`. Deployments will run from GitHub Actions on pushes to `main` and publish the web build to the repository Pages URL:

`https://michael-cmd-sys.github.io/michaelawunikofi/`

## Goals

- Deploy the Dioxus web app automatically on pushes to `main`.
- Use the Dioxus-maintained deploy action instead of a hand-rolled build script.
- Ensure the deployed app works correctly from the repository subpath `/michaelawunikofi/`.
- Keep the deployment setup small, explicit, and easy to maintain.

## Non-Goals

- No custom production backend hosting.
- No multi-environment deployment matrix.
- No preview deployments for pull requests in this change.
- No domain-name setup or custom CNAME configuration in this change.

## Current State

The repo already contains:

- a Dioxus 0.7 app configured for web in `Cargo.toml`
- Tailwind input/output configured in `Dioxus.toml`
- a working local web build flow via `dx build --platform web`
- a GitHub remote at `git@github.com:Michael-cmd-sys/michaelawunikofi.git`

The repo does not yet contain:

- a GitHub Actions workflow for deployment
- Pages-specific base-path configuration
- deployment verification checks in CI

## Recommended Approach

Use `DioxusLabs/deploy-action` in a GitHub Actions workflow that:

- triggers on pushes to `main`
- builds the Dioxus web target
- publishes the resulting static site to GitHub Pages

This keeps the deployment flow aligned with Dioxus’s build expectations and avoids maintaining separate manual setup steps for the CLI and output handling.

## Architecture

### Workflow

Add a workflow file under `.github/workflows/` for Pages deployment.

Responsibilities:

- check out the repository
- set the required Pages permissions
- invoke the Dioxus deploy action with the web target
- publish the build artifact to GitHub Pages

The workflow should be narrow and single-purpose: one job path for production Pages deployment from `main`.

### Base Path Handling

Because the site will be hosted under a repository path instead of the domain root, the app must generate asset and routing URLs relative to:

`/michaelawunikofi/`

The design requirement is that generated CSS, JavaScript, Wasm, and asset URLs resolve correctly when served from that subpath.

The implementation must make the repository base path explicit during build.

Priority order:

- prefer Dioxus configuration for the web base path
- otherwise use deploy-action or workflow-supported build configuration
- only fall back to minimal application-level changes if configuration cannot express the base path correctly

The implementation should use the first supported option in that order and avoid implicit root-path assumptions.

### Permissions and Security

The workflow should use GitHub’s built-in Pages deployment model and only request the permissions it needs:

- `contents: read`
- `pages: write`
- `id-token: write`

No personal access token should be introduced if the GitHub Pages deployment can use the standard `GITHUB_TOKEN` flow.

## File-Level Plan

### `.github/workflows/deploy.yml`

Add a workflow that:

- runs on push to `main`
- uses the Dioxus deploy action
- targets the web build
- publishes to GitHub Pages
- is explicit about branch trigger and required permissions

### `Dioxus.toml`

Update this file if Dioxus supports repository-subpath configuration here.

If the base path can be expressed in `Dioxus.toml`, that should be the primary configuration point instead of embedding repo-specific path logic in components.

### Application Code

Avoid application-level deployment logic unless the Dioxus/GitHub Pages path handling cannot be expressed in configuration.

If code changes are required, they should be minimal and limited to correct path generation for static deployment.

## Failure Modes and Mitigations

### Risk: assets load locally but break on Pages

Cause:

- output assumes root-relative URLs instead of the repository base path

Mitigation:

- make base-path handling explicit
- verify the built output references the repository subpath correctly

### Risk: workflow builds but does not publish

Cause:

- missing Pages permissions or incorrect action usage

Mitigation:

- use the standard Pages permission set
- follow the Dioxus deploy action’s expected workflow shape closely

### Risk: workflow depends on undocumented defaults

Cause:

- implicit action behavior without pinned expectations

Mitigation:

- keep the workflow explicit about branch, platform, and deploy target
- document any action inputs used for Pages deployment

## Verification Plan

- run local Rust verification before finishing
- run a local `dx build --platform web` check after any config changes
- inspect the GitHub Actions workflow for syntax and trigger correctness
- verify the built public output shape matches what Pages expects
- after merge/push, confirm the first Pages deployment succeeds in GitHub Actions
- confirm the site loads at `https://michael-cmd-sys.github.io/michaelawunikofi/`

## Acceptance Criteria

- pushes to `main` trigger a GitHub Actions deployment workflow
- the workflow builds and publishes the Dioxus web app through GitHub Pages
- the deployed site resolves assets correctly under `/michaelawunikofi/`
- no manual deploy step is required after merging to `main`
- the deployment setup uses the Dioxus deploy action rather than a custom script
