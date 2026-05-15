# Build verification

After any code change, run `cargo clippy --all-targets -- -D warnings`. If it fails, fix before reporting the task complete.

After any code change, run `cargo fmt --check`. If it fails, run `cargo fmt`.

If the change affects behavior covered by tests, also run `cargo test`.

Do not run the app (`cargo run`) unless the user asks.
