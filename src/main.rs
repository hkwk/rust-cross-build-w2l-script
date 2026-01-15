use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// yy — Zig helper for cross-compiling from Windows to Linux
#[derive(Parser)]
#[command(author, version, about = "Create Zig wrapper scripts and Cargo config for cross-compilation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate wrapper scripts for a target (creates cmd and optional ps1 wrappers)
    Generate {
        /// target triple, e.g. x86_64-unknown-linux-musl
        target: String,
        /// output directory for wrapper scripts (default: current dir)
        #[arg(short, long, value_name = "DIR")]
        out: Option<PathBuf>,
        /// also write PowerShell (.ps1) wrappers
        #[arg(long)]
        ps: bool,
    },

    /// Install a Cargo config snippet into %USERPROFILE%\\.cargo\\config.toml
    InstallConfig {
        /// target triple to add
        target: String,
        /// set rustflags for static musl linking
        #[arg(long, default_value_t = true)]
        set_rustflags: bool,
    },

    /// Check if `zig` is available on PATH
    CheckZig,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { target, out, ps } => {
            let out_ref = out.as_deref();
            let wrote = yy::generate_wrappers(&target, out_ref, ps)?;
            println!("Wrappers written to {}", wrote.display());
        }
        Commands::InstallConfig { target, set_rustflags } => {
            yy::install_cargo_config(&target, set_rustflags)?;
            println!("Cargo config updated for target {target}");
        }
        Commands::CheckZig => {
            if let Some(p) = yy::find_zig() {
                println!("Found zig: {}", p.display());
            } else {
                println!("`zig` not found on PATH.");
            }
        }
    }

    Ok(())
}

