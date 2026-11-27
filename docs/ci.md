# CI and development checks

The workflow runs for pushes and pull requests targeting `main` or `develop`, and manual dispatch. `Format` runs on Ubuntu 24.04 x86_64. `Build and test` uses Ubuntu 24.04 x86_64, macOS 15 ARM64 and Windows 2025 x86_64, with fail-fast disabled. The only current feature configuration is the default; all targets are checked. Hosted execution has not yet been verified.

Run locally:

```sh
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
```

Rust 1.100.0 is pinned in rust-toolchain.toml, including rustfmt and Clippy. Rustup installs the pinned version and components on demand; a non-rustup installation must provide matching tools. Review and update the pin deliberately with its checks. Cargo.lock is committed for the executable.

CI uses pinned checkout and cache action commits, no persisted checkout credentials, `contents: read`, timeouts and cancellation of superseded runs per workflow and ref/PR. Cargo registry, git and target caches are keyed by OS, architecture, toolchain file and lockfile; never cache credentials. The hosted runner must have rustup and access to the pinned toolchain. Weekly Dependabot updates cover Cargo and Actions, with five PRs each, targeting the verified remote `main`. If the owner later creates remote `develop`, reconsider the target.

After actual CI runs, the owner can configure rulesets for `main` and `develop`: require PRs and the generated Format and all three Build and test checks, block force pushes and deletion, and require up-to-date branches where appropriate. Select the actual emitted check names after successful runs. Avoid mandatory reviewer counts that block a sole maintainer. Enable Actions and Dependabot as required in repository settings; no settings are changed here.

Future release automation must validate revision, tag, version and required checks, isolate release permissions, exclude untrusted PR code, and associate each artifact with its exact corresponding source, license, attribution, notices, build information and SHA-256 checksum. Planned targets include macOS ARM64/x86_64, Linux ARM64/x86_64 and Windows x86_64. No release workflow exists yet.
