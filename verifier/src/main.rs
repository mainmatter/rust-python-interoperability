use std::{path::Path, process::Output};

use duct::cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let venv = std::env::current_dir()?
        .canonicalize()
        .expect("Failed to determine the canonical form of the current directory path")
        .join(".venv");

    if venv.try_exists().ok() != Some(true) {
        // Create the project-specific Python virtual environment.
        println!("\n=== Creating the Python virtual environment ===\n");
        cmd!("uv", "sync", "--no-install-workspace", "--managed-python")
            .stderr_to_stdout()
            .run()?;
    }

    println!("\n=== Syncing Python packages ===\n");
    run_in_venv(
        cmd!("uv", "sync", "--all-packages", "--managed-python"),
        &venv,
    )?;

    println!("\n=== Testing the Rust crate ===\n");
    run_in_venv(cmd!("cargo", "test"), &venv)?;

    println!("\n=== Testing the Python package ===\n");
    run_in_venv(cmd!("uv", "run", "--managed-python", "pytest"), &venv)?;

    Ok(())
}

/// Execute a command in an environment that's equivalent to what you'd get
/// after activating a Python virtual environment in a shell session via
/// `source .venv/bin/activate`.
fn run_in_venv(command: duct::Expression, venv: &Path) -> std::io::Result<Output> {
    let path = {
        let old = std::env::var("PATH").unwrap_or_default();
        format!("{}:{old}", venv.join("bin").display())
    };

    command
        .stderr_to_stdout()
        .env("VIRTUAL_ENV", &venv)
        .env("PATH", &path)
        .env_remove("PYTHON_HOME")
        .run()
}
