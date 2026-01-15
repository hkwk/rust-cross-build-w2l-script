//! yy — Library helpers for Zig wrapper generation and Cargo config installation.
//!
//! This crate exposes small, testable functions used by the `yy` binary.

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Locate `zig` on PATH. Returns the absolute path to the executable if found.
pub fn find_zig() -> Option<PathBuf> {
    which::which("zig").ok()
}

/// Generate wrapper scripts for `target` into `out` (or current dir when `None`).
///
/// Returns the output directory on success.
pub fn generate_wrappers(target: &str, out: Option<&Path>, ps: bool) -> Result<PathBuf> {
    let out = out.map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&out).with_context(|| format!("Failed to create output dir: {}", out.display()))?;

    if find_zig().is_none() {
        eprintln!("Warning: `zig` executable not found on PATH. Generated wrappers will call zig.exe and will fail until zig is available.");
    }

    write_cmd_wrapper(&out, target)?;
    write_ar_wrapper(&out)?;
    if ps {
        write_ps_wrapper(&out, target)?;
        write_ps_ar_wrapper(&out)?;
    }

    Ok(out)
}

/// Append a target section to `%USERPROFILE%\.cargo\config.toml` (creates file if needed).
pub fn install_cargo_config(target: &str, set_rustflags: bool) -> Result<()> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    let cargo_dir = home.join(".cargo");
    fs::create_dir_all(&cargo_dir).with_context(|| format!("create dir {}", cargo_dir.display()))?;
    let config = cargo_dir.join("config.toml");

    let mut contents = String::new();
    if config.exists() {
        contents = fs::read_to_string(&config).with_context(|| format!("reading {}", config.display()))?;
        if contents.contains(&format!("[target.{}]", target)) {
            println!("Config already contains a section for target {}", target);
            return Ok(());
        }
    }

    let mut snippet = format!("[target.{}]\nlinker = \"zig-{}\"\nar = \"zig-ar\"\n", target, target);
    if set_rustflags {
        snippet.push_str("rustflags = [\"-C\", \"target-feature=+crt-static\"]\n");
    }

    contents.push('\n');
    contents.push_str(&snippet);
    fs::write(&config, contents).with_context(|| format!("writing {}", config.display()))?;
    Ok(())
}

// Internal helpers -----------------------------------------------------------

fn write_cmd_wrapper(out: &Path, target: &str) -> Result<()> {
    let name = format!("zig-{}.cmd", target.replace(':', "-"));
    let path = out.join(name);
    let content = format!("@echo off\r\nzig.exe cc -target {t} -static %*\r\n", t = target);
    atomic_write(&path, content.as_bytes())?;
    Ok(())
}

fn write_ar_wrapper(out: &Path) -> Result<()> {
    let path = out.join("zig-ar.cmd");
    let content = "@echo off\r\nzig.exe ar %*\r\n";
    atomic_write(&path, content.as_bytes())?;
    Ok(())
}

fn write_ps_wrapper(out: &Path, target: &str) -> Result<()> {
    let name = format!("zig-{}.ps1", target.replace(':', "-"));
    let path = out.join(name);
    let content = format!("& \"zig.exe\" cc -target {t} -static $args\r\n", t = target);
    atomic_write(&path, content.as_bytes())?;
    Ok(())
}

fn write_ps_ar_wrapper(out: &Path) -> Result<()> {
    let path = out.join("zig-ar.ps1");
    let content = "& \"zig.exe\" ar $args\r\n";
    atomic_write(&path, content.as_bytes())?;
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension("tmp");
    let mut f = fs::File::create(&tmp).with_context(|| format!("creating {}", tmp.display()))?;
    f.write_all(bytes).with_context(|| format!("writing {}", tmp.display()))?;
    f.flush()?;
    fs::rename(&tmp, path).with_context(|| format!("renaming to {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn wrapper_generation_and_content() {
        let t = "x86_64-unknown-linux-musl";
        let mut tmpdir = std::env::temp_dir();
        tmpdir.push("yy-test-wrappers-lib");
        let _ = fs::remove_dir_all(&tmpdir);
        fs::create_dir_all(&tmpdir).unwrap();

        let out = generate_wrappers(t, Some(&tmpdir), false).unwrap();
        assert_eq!(out, tmpdir);

        let p = tmpdir.join(format!("zig-{}.cmd", t));
        let s = fs::read_to_string(&p).unwrap();
        assert!(s.contains(&format!("-target {t}")));

        let _ = fs::remove_dir_all(&tmpdir);
    }
}