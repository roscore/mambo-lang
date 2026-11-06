# TDD log

1. Wrote integration tests for bar parsing, exact Korean tokens, Unicode positions, loop structure, stack operations, resource limits, and CLI behavior before implementation.
2. Red phase: `cargo test` failed because the library and binary targets did not yet exist.
3. First implementation run exposed two incorrect test fixtures (a Unicode column count and a five-beat test bar); corrected them. Added a test for the missing `MAMBO` example and observed its compile failure before creating the example.
4. Green phase: 16 integration tests pass, including parser errors, arithmetic overflow, loop bounds, file-based CLI, and the `MAMBO` example.
5. Verification: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo doc --no-deps`, `cargo package --allow-dirty`, and built-binary smoke checks for all five CLI commands passed.
