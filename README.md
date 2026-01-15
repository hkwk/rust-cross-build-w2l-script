# yy — Zig helper for cross-compiling from Windows to Linux

`yy` helps you generate simple Zig wrapper scripts (`zig-<target>.cmd` / `.ps1`) and a Cargo `config.toml` snippet so you can cross-compile Rust projects from Windows using Zig as a linker/sysroot.

## Quick start ✅

1. Install Zig and add it to your `PATH`: https://ziglang.org/download
2. Add your target with rustup, e.g.:

```powershell
rustup target add x86_64-unknown-linux-musl
```

3. Generate wrappers in a folder on `PATH` (or into `C:\tools\zig-cross\bin`):

```powershell
# generate cmd wrappers in current dir
cargo run --release -- generate x86_64-unknown-linux-musl --out C:\tools\zig-cross\bin --ps
```

4. Install Cargo config for that target (writes to `%USERPROFILE%\.cargo\config.toml`):

```powershell
cargo run --release -- install-config x86_64-unknown-linux-musl
```

5. Now you can build your project for the target:

```powershell
cargo build --release --target x86_64-unknown-linux-musl
```

## Commands
- `generate <target>`: create `zig-<target>.cmd` and `zig-ar.cmd` (optionally PowerShell `.ps1` wrappers with `--ps`).
- `install-config <target>`: append a `[target.<target>]` section to `%USERPROFILE%\.cargo\config.toml` with proper `linker` and `ar` (and recommended `rustflags` for musl).
- `check-zig`: verify whether `zig` is available on `PATH`.

## Publishing to crates.io 📦
- Update `Cargo.toml` metadata (`name = "yy"`, `description`, `license`, `repository`) and ensure `README.md` is present.
- Create an account on crates.io and `cargo login` with your API token.
- Run `cargo publish --allow-dirty` (or ensure the repo is clean) to publish the crate.

## Troubleshooting & Tips ⚠️
- If wrappers are invoked but `zig.exe` is not found, add Zig's bin folder to `PATH`.
- When targeting `*-musl`, static linking (`rustflags = ["-C", "target-feature=+crt-static"]`) helps avoid needing glibc on the target.

