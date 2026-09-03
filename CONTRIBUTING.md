# Contributing to Valence UF App

Thank you for improving this project.

## Development setup

1. Clone [unified-field-dev/valence-uf-app](https://github.com/unified-field-dev/valence-uf-app)
2. Install Rust **nightly** (matches CI; Leptos workspace deps use the `nightly` feature)
3. From the repository root:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-valence-uf-app
cargo fmt -p valence-backend -p valence-app -p valence-uf-app-e2e -p protected-valence-host -- --check
cargo check --workspace
```

Full gates (Layer 1 + Layer 2 e2e): [`docs/VERIFICATION.md`](docs/VERIFICATION.md).

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md).

## Pull requests

- Prefer small, focused PRs.
- Update [`README.md`](README.md) when public API or host wiring steps change.
- Run the Verify commands in README or VERIFICATION before opening a PR.
