use anyhow::Context;
use semver::Version;
use std::path::PathBuf;

fn main() -> Result<(), anyhow::Error> {
    rye_is_installed()?;
    let git_root = get_git_root()?;
    install_sysconfigpatcher()?;

    let toolchains = get_python_toolchains()?;
    for toolchain in &toolchains {
        println!("Patching sysconfig for Python {}", toolchain.version);
        toolchain.patch()?;
    }

    // Clean up the build artifacts, if they exist
    let debug_target = git_root.join("target").join("debug");
    if debug_target.exists() {
        println!("Cleaning up the `debug` target directory");
        std::fs::remove_dir_all(&debug_target)?;
    }
    // Recreate the `debug` target directory
    std::fs::create_dir_all(&debug_target)?;

    // Copy the relevant dynamic libraries to the `debug` target directory
    for toolchain in toolchains {
        let truncated_version = format!("{}.{}", toolchain.version.major, toolchain.version.minor);
        let filename = format!("libpython{}.{}", truncated_version, std::env::consts::DLL_EXTENSION);
        let libpython = toolchain.home_directory.join("lib").join(&filename);
        let target = debug_target.join(filename);
        println!("Copying `{}` to `{}`", libpython.display(), target.display());
        // ISSUE: What happens if we have multiple Python toolchains with same major.minor version?
        if let Err(e) = std::fs::copy(&libpython, &target) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                anyhow::bail!("Failed to copy `{}` to `{}`: {}", libpython.display(), target.display(), e);
            }
        }
    }
    Ok(())
}

fn get_git_root() -> Result<PathBuf, anyhow::Error> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .context("Failed to run `git rev-parse --show-toplevel`")?;

    if !output.status.success() {
        panic!(
            "`git rev-parse --show-toplevel` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let git_root = String::from_utf8(output.stdout).context("Invalid UTF-8 in git root path")?;
    Ok(PathBuf::from(git_root.trim()))
}

fn rye_is_installed() -> Result<(), anyhow::Error> {
    let output = std::process::Command::new("rye")
        .arg("--version")
        .output()
        .expect("Failed to run `rye --version`");

    if !output.status.success() {
        anyhow::bail!(
            "`rye --version` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

struct PythonToolchain {
    home_directory: PathBuf,
    version: Version,
}

impl PythonToolchain {
    fn patch(&self) -> Result<(), anyhow::Error> {
        let output = std::process::Command::new("sysconfigpatcher")
            .arg(&self.home_directory)
            .output()
            .with_context(|| {
                format!(
                    "Failed to run `sysconfigpatcher` for {}",
                    self.home_directory.display()
                )
            })?;

        if !output.status.success() {
            anyhow::bail!(
                "`sysconfigpatcher` failed for {}:\n{}",
                self.home_directory.display(),
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }
}

fn get_python_toolchains() -> Result<Vec<PythonToolchain>, anyhow::Error> {
    let output = std::process::Command::new("rye")
        .arg("toolchain")
        .arg("list")
        .arg("--format")
        .arg("json")
        .output()
        .expect("Failed to run `rye list`");

    if !output.status.success() {
        anyhow::bail!(
            "`rye list` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Example payload:
    // ```json
    // {
    //     "name": "cpython@3.12.3",
    //     "path": "/Users/luca/.rye/py/cpython@3.12.3/bin/python3"
    // }
    // ```
    #[derive(serde::Deserialize)]
    struct RawToolchain {
        // Name of the Python toolchain
        name: String,
        // Path to the Python executable
        path: PathBuf,
    }

    let toolchains: Vec<RawToolchain> = serde_json::from_slice(&output.stdout)?;
    let mut parsed_toolchains = vec![];
    for toolchain in toolchains {
        let version = toolchain
            .name
            .split_once('@')
            .context("Missing version in toolchain name")?
            .1;
        let version = Version::parse(version).context("Invalid Python version")?;
        let directory = toolchain
            .path
            // The `bin/` directory is the parent of the Python executable
            .parent()
            .context("No parent directory for Python executable")?
            // The parent of the `bin/` directory is the Python folder
            .parent()
            .context("No parent directory for `bin/` in the Python folder")?
            .to_owned();
        parsed_toolchains.push(PythonToolchain {
            home_directory: directory,
            version,
        });
    }

    Ok(parsed_toolchains)
}

fn install_sysconfigpatcher() -> Result<(), anyhow::Error> {
    // Check if it's already installed first
    let output = std::process::Command::new("rye")
        .arg("tools")
        .arg("list")
        .output()
        .expect("Failed to run `rye tools list`");

    if !output.status.success() {
        anyhow::bail!(
            "`rye tools list` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let tools = String::from_utf8(output.stdout)?;
    if tools
        .split("\n")
        .any(|tool| tool.trim() == "sysconfigpatcher")
    {
        return Ok(());
    }

    // Install it if it's not
    let output = std::process::Command::new("rye")
        .arg("install")
        .arg("--git")
        .arg("https://github.com/bluss/sysconfigpatcher")
        .arg("sysconfigpatcher")
        .output()
        .expect("Failed to run `rye tools install sysconfigpatcher`");

    if !output.status.success() {
        anyhow::bail!(
            "Failed to install sysconfigpatcher:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
