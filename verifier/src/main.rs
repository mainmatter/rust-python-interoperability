use duct::cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the project-specific Python virtual environment.
    println!("Creating the Python virtual environment...");
    cmd!("uv", "sync", "--no-install-workspace").run()?;

    let venv = std::env::current_dir()?.join(".venv");
    cmd!("uv", "sync", "--all-packages")
        // Tell `pyo3` where to find the project-specific
        // virtual environment.
        .env("VIRTUAL_ENV", &venv)
        .run()?;

    println!("Testing the Rust crate...");
    cmd!("cargo", "test")
        // Tell `pyo3` where to find the project-specific
        // virtual environment.
        .env("VIRTUAL_ENV", venv)
        .run()?;
    println!("Testing the Python package...");
    cmd!("uv", "run", "pytest").run()?;

    Ok(())
}
