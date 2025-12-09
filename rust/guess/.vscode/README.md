Usage notes for debug/run in Cursor

Quick start
- "Debug workspace binary (lldb)": builds (preLaunchTask) and runs `target/debug/${workspaceFolderBasename}` (e.g. `target/debug/guess`). Use this for the crate's primary binary.
- "Debug workspace binary (current file alias)": runs `target/debug/${fileBasenameNoExtension}` after executing the `cargo build current file` task. This builds the binary named after the current file (good when you have multiple bin targets named after files).

If you prefer zero config
- rust-analyzer provides Run | Debug code lenses above `fn main()` that will infer the right binary. Click Debug to launch using your installed debug adapter.

Enabling the debugger
- Install/enable the CodeLLDB extension (it provides the `type: "lldb"` debug adapter used by these configurations).
- If your editor flags the debug type as unknown, enable CodeLLDB in Cursor's Extensions view or install it.

Notes and caveats
- `cargo` names binaries after the package/bin target, not necessarily the source file basename. The "current file" task builds `cargo build --bin <fileBasenameNoExtension>`; that works only if a binary with that name exists in `Cargo.toml` or `src/bin/<name>.rs`.
- If a binary name differs from the file name, either use the workspace binary config or add a `[[bin]]` entry to `Cargo.toml`.

Useful terminal commands
```bash
# build & run the main crate binary
cargo run

# fast iteration (install cargo-watch first):
cargo install cargo-watch
cargo watch -x run
```

If you'd like me to, I can:
- Add a `[[bin]]` example in `Cargo.toml` and a sample `src/bin/` file.
- Remove the alias launch entry or make a more intelligent launch that queries Cargo metadata (more complex).

