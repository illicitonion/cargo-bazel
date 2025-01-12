//! Tools for gathering various kinds of metadata (Cargo.lock, Cargo metadata, Crate Index info).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use cargo_lock::Lockfile as CargoLockfile;
use cargo_metadata::{Metadata as CargoMetadata, MetadataCommand};

// TODO: This should also return a set of [crate-index::IndexConfig]s for packages in metadata.packages
pub trait MetadataGenerator {
    fn generate<T: AsRef<Path>>(&self, manifest_path: T) -> Result<(CargoMetadata, CargoLockfile)>;
}

pub struct Generator {
    cargo_bin: PathBuf,
    rustc_bin: PathBuf,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            cargo_bin: PathBuf::from(env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())),
            rustc_bin: PathBuf::from(env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string())),
        }
    }

    pub fn with_cargo(mut self, cargo_bin: PathBuf) -> Self {
        self.cargo_bin = cargo_bin;
        self
    }

    pub fn with_rustc(mut self, rustc_bin: PathBuf) -> Self {
        self.rustc_bin = rustc_bin;
        self
    }
}

impl MetadataGenerator for Generator {
    fn generate<T: AsRef<Path>>(&self, manifest_path: T) -> Result<(CargoMetadata, CargoLockfile)> {
        let lockfile = {
            let manifest_dir = manifest_path
                .as_ref()
                .parent()
                .expect("The manifest should have a parent directory");
            let lock_path = manifest_dir.join("Cargo.lock");
            if !lock_path.exists() {
                bail!("No `Cargo.lock` file was found with the given manifest")
            }
            cargo_lock::Lockfile::load(lock_path)?
        };

        let metadata = MetadataCommand::new()
            .cargo_path(&self.cargo_bin)
            .manifest_path(manifest_path.as_ref())
            .other_options(["--locked".to_owned()])
            .exec()?;

        Ok((metadata, lockfile))
    }
}

pub struct LockGenerator {
    cargo_bin: PathBuf,
    rustc_bin: PathBuf,
}

impl LockGenerator {
    pub fn new(cargo_bin: PathBuf, rustc_bin: PathBuf) -> Self {
        Self {
            cargo_bin,
            rustc_bin,
        }
    }

    pub fn generate(
        &self,
        manifest_path: &Path,
        existing_lock: &Option<PathBuf>,
    ) -> Result<cargo_lock::Lockfile> {
        let manifest_dir = manifest_path.parent().unwrap();
        let generated_lockfile_path = manifest_dir.join("Cargo.lock");

        let output = if let Some(lock) = existing_lock {
            if !lock.exists() {
                bail!(
                    "An existing lockfile path was provided but a file at '{}' does not exist",
                    lock.display()
                )
            }

            // Install the file into the target location
            if generated_lockfile_path.exists() {
                fs::remove_file(&generated_lockfile_path)?;
            }
            fs::copy(&lock, &generated_lockfile_path)?;

            // Ensure the Cargo cache is up to date to simulate the behavior
            // of having just generated a new one
            Command::new(&self.cargo_bin)
                // Cargo detects config files based on `pwd` when running so
                // to ensure user provided Cargo config files are used, it's
                // critical to set the working directory to the manifest dir.
                .current_dir(manifest_dir)
                .arg("fetch")
                .arg("--locked")
                .arg("--manifest-path")
                .arg(manifest_path)
                .env("RUSTC", &self.rustc_bin)
                .output()
                .context(format!(
                    "Error running cargo to fetch crates '{}'",
                    manifest_path.display()
                ))?
        } else {
            // Simply invoke `cargo generate-lockfile`
            Command::new(&self.cargo_bin)
                // Cargo detects config files based on `pwd` when running so
                // to ensure user provided Cargo config files are used, it's
                // critical to set the working directory to the manifest dir.
                .current_dir(manifest_dir)
                .arg("generate-lockfile")
                .arg("--manifest-path")
                .arg(manifest_path)
                .env("RUSTC", &self.rustc_bin)
                .output()
                .context(format!(
                    "Error running cargo to generate lockfile '{}'",
                    manifest_path.display()
                ))?
        };

        if !output.status.success() {
            bail!(format!("Failed to generate lockfile: {:?}", output))
        }

        cargo_lock::Lockfile::load(&generated_lockfile_path).context(format!(
            "Failed to load lockfile: {}",
            generated_lockfile_path.display()
        ))
    }
}

pub fn write_metadata(path: &Path, metadata: &cargo_metadata::Metadata) -> Result<()> {
    let content =
        serde_json::to_string_pretty(metadata).context("Failed to serialize Cargo Metadata")?;

    fs::write(path, content).context("Failed to write metadata to disk")
}

pub fn load_metadata(
    metadata_path: &Path,
    lockfile_path: Option<&Path>,
) -> Result<(cargo_metadata::Metadata, cargo_lock::Lockfile)> {
    let content = fs::read_to_string(metadata_path)
        .with_context(|| format!("Failed to load Cargo Metadata: {}", metadata_path.display()))?;

    let metadata =
        serde_json::from_str(&content).context("Unable to deserialize Cargo metadata")?;

    let lockfile_path = lockfile_path
        .map(PathBuf::from)
        .unwrap_or_else(|| metadata_path.parent().unwrap().join("Cargo.lock"));

    let lockfile = cargo_lock::Lockfile::load(&lockfile_path)
        .with_context(|| format!("Failed to load lockfile: {}", lockfile_path.display()))?;

    Ok((metadata, lockfile))
}
