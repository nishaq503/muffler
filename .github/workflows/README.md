# GitHub Actions Workflows

This directory contains the CI/CD workflows for the project.

## Workflows

### 🔄 CI (`ci.yml`)
**Triggers:** Push/PR to main/master branches

The main continuous integration workflow that runs on every push and pull request:

- **Pre-commit checks**: Runs all pre-commit hooks including:
  - Code formatting (ruff, cargo fmt, isort)
  - Linting (ruff, clippy, mypy)
  - Security checks (bandit)
  - File checks (YAML, TOML, etc.)

- **Rust tests**: Runs the full Rust test suite using `cargo nextest`

- **Python tests**: Runs Python tests for all packages in the workspace

- **Build check**: Ensures all code compiles successfully

### 🚀 Release (`release.yml`)
**Triggers:** Git tags starting with `v*`

Handles release builds and publishing:

- Runs full test suite before building
- Builds release binaries for multiple platforms (Linux, Windows, macOS)
- Creates GitHub releases with downloadable artifacts
- Generates release notes automatically

### 🔒 Security (`security.yml`)
**Triggers:**
- Weekly schedule (Mondays at 9 AM UTC)
- Manual dispatch
- Changes to dependency files

Security and dependency management:

- **Rust security audit**: Uses `cargo audit` to check for known vulnerabilities
- **Python security check**: Uses `pip-audit` for Python dependency vulnerabilities
- **Dependency review**: Reviews dependency changes in PRs

### 📚 Documentation (`docs.yml`)
**Triggers:** Changes to documentation-related files

Documentation building and validation:

- **Rust docs**: Builds Rust documentation and checks for broken links
- **Python docs**: Validates Python docstrings
- **LaTeX docs**: Builds the research paper PDF from LaTeX sources

### 🧪 Test Workflows (`test-workflows.yml`)
**Triggers:** Changes to workflow files

Meta-workflow for testing the workflows themselves:

- Validates YAML syntax of all workflow files
- Tests basic setup and dependency installation
- Ensures workflows would run successfully

## Status Badges

Add these to your main README.md:

```markdown
[![CI](https://github.com/USERNAME/muffler/workflows/CI/badge.svg)](https://github.com/USERNAME/muffler/actions/workflows/ci.yml)
[![Security](https://github.com/USERNAME/muffler/workflows/Security%20and%20Dependencies/badge.svg)](https://github.com/USERNAME/muffler/actions/workflows/security.yml)
[![Documentation](https://github.com/USERNAME/muffler/workflows/Documentation/badge.svg)](https://github.com/USERNAME/muffler/actions/workflows/docs.yml)
```

## Setup Instructions

1. **Push the workflows**: Commit and push the `.github/workflows/` directory to your repository

2. **Enable GitHub Actions**: Ensure GitHub Actions are enabled in your repository settings

3. **Set up branch protection** (optional but recommended):
   - Go to Settings → Branches
   - Add rule for `main`/`master`
   - Require status checks to pass before merging
   - Select the CI workflow jobs as required checks

4. **Configure secrets** (if needed):
   - `GITHUB_TOKEN` is automatically provided
   - Add any additional secrets in Settings → Secrets and variables → Actions

## Local Development

All checks that run in CI can be run locally:

```bash
# Run all pre-commit checks
pre-commit run --all-files

# Run formatting
mask fmt

# Run linting
mask lint

# Run tests
mask test

# Build everything
mask build
```

## Caching

The workflows use caching to speed up builds:

- **Rust**: Uses `Swatinem/rust-cache` for Cargo dependencies and build artifacts
- **Python**: Uses uv's built-in caching via `astral-sh/setup-uv`
- **Actions**: Caches are automatically managed and shared between workflow runs

## Maintenance

- **Dependencies**: The security workflow runs weekly to check for vulnerabilities
- **Workflow updates**: The test-workflows workflow validates changes to workflow files
- **Manual triggers**: Most workflows can be triggered manually from the Actions tab
