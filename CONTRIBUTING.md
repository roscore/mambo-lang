# Contributing / 기여

Issues and pull requests are welcome. Please describe the behavior you want or the bug you found, include a small Mambo source sample, and mention your platform and Rust version when relevant. Keep language changes coherent with four-beat bars and the documented stack model.

For a behavior change, first add a failing focused test and record the red/green steps in `TDD_LOG.md`. Then implement the smallest clear change. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --all-targets` before submitting. Do not add copyrighted audio, lyrics, logos, or imagery.

By contributing, you agree to the [Code of Conduct](CODE_OF_CONDUCT.md). Contributions are licensed under [MIT](LICENSE).
