//! TODO

use std::path::PathBuf;

use anyhow::{bail, Result};
use structopt::StructOpt;

use crate::annotation::Annotations;
use crate::config::Config;
use crate::context::Context;
use crate::lockfile::{is_cargo_lockfile, write_lockfile, LockfileKind};
use crate::metadata::load_metadata;
use crate::rendering::{write_outputs, Renderer};

/// Command line options for the `generate` subcommand
#[derive(StructOpt, Debug)]
pub struct GenerateOptions {
    /// The path to a Cargo binary to use for gathering metadata
    #[structopt(long, env = "CARGO")]
    pub cargo: Option<PathBuf>,

    /// The path to a rustc binary for use with Cargo
    #[structopt(long, env = "RUSTC")]
    pub rustc: Option<PathBuf>,

    /// The config file with information about the Bazel and Cargo workspace
    #[structopt(long)]
    pub config: PathBuf,

    /// The path to either a Cargo or Bazel lockfile
    #[structopt(long)]
    pub lockfile: PathBuf,

    /// The type of lockfile
    #[structopt(long)]
    pub lockfile_kind: LockfileKind,

    /// The directory of the current repository rule
    #[structopt(long)]
    pub repository_dir: PathBuf,

    /// A [Cargo config](https://doc.rust-lang.org/cargo/reference/config.html#configuration)
    /// file to use when gathering metadata
    #[structopt(long)]
    pub cargo_config: Option<PathBuf>,

    /// Whether or not to ignore the provided lockfile and re-generate one
    #[structopt(long)]
    pub repin: bool,

    /// The path to a Cargo metadata `json` file.
    #[structopt(long)]
    pub metadata: Option<PathBuf>,

    /// If true, outputs will be printed instead of written to disk.
    #[structopt(long)]
    pub dry_run: bool,
}

pub fn generate(opt: GenerateOptions) -> Result<()> {
    // Load the config
    let config = Config::try_from_path(&opt.config)?;

    // Determine if the dependencies need to be repinned.
    let mut should_repin = opt.repin;

    // Cargo lockfiles must always be repinned.
    if is_cargo_lockfile(&opt.lockfile, &opt.lockfile_kind) {
        should_repin = true;
    }

    // Go straight to rendering if there is no need to repin
    if !should_repin {
        let context = Context::try_from_path(opt.lockfile)?;

        // Render build files
        let outputs = Renderer::new(config.rendering).render(&context)?;

        // Write the outputs to disk
        write_outputs(outputs, &opt.repository_dir, opt.dry_run)?;

        return Ok(());
    }

    // Ensure Cargo and Rustc are available for use during generation.
    let cargo_bin = match &opt.cargo {
        Some(bin) => bin,
        None => bail!("The `--cargo` argument is required when generating unpinned content"),
    };
    let rustc_bin = match &opt.rustc {
        Some(bin) => bin,
        None => bail!("The `--rustc` argument is required when generating unpinned content"),
    };

    // Ensure a path to a metadata file was provided
    let metadata_path = match &opt.metadata {
        Some(path) => path,
        None => bail!("The `--metadata` argument is required when generating unpinned content"),
    };

    // Load Metadata and Lockfile
    let (cargo_metadata, cargo_lockfile) = load_metadata(
        metadata_path,
        if is_cargo_lockfile(&opt.lockfile, &opt.lockfile_kind) {
            Some(&opt.lockfile)
        } else {
            None
        },
    )?;

    // Copy the rendering config for later use
    let render_config = config.rendering.clone();

    // Annotate metadata
    let annotations = Annotations::new(cargo_metadata, cargo_lockfile, config)?;

    // Generate renderable contexts for earch package
    let context = Context::new(annotations, cargo_bin, rustc_bin)?;

    // Render build files
    let outputs = Renderer::new(render_config).render(&context)?;

    // Write outputs
    write_outputs(outputs, &opt.repository_dir, opt.dry_run)?;

    // Ensure Bazel lockfiles are written to disk so future generations can be short-circuted.
    if matches!(opt.lockfile_kind, LockfileKind::Bazel) {
        write_lockfile(context, &opt.lockfile, opt.dry_run)?;
    }

    Ok(())
}
