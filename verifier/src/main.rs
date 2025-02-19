use duct::cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let venv = std::env::current_dir()?.join(".venv");

    if venv.try_exists().ok() != Some(true) {
        // Create the project-specific Python virtual environment.
        println!("Creating the Python virtual environment...");
        cmd!("uv", "sync", "--no-install-workspace").run()?;
    }

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
