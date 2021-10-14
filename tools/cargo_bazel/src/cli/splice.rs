//! TODO

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::cli::Result;
use crate::splicing::{Splicer, SplicingManifest};

/// Command line options for the `splice` subcommand
#[derive(StructOpt, Debug)]
pub struct SpliceOptions {
    /// A generated manifest of splicing inputs
    #[structopt(long)]
    pub splicing_manifest: PathBuf,

    /// A Cargo lockfile (Cargo.lock).
    #[structopt(long)]
    pub cargo_lockfile: Option<PathBuf>,

    /// The directory in which to build the workspace. A `Cargo.toml` file
    /// should always be produced within this directory.
    #[structopt(long)]
    pub workspace_dir: PathBuf,

    /// If true, outputs will be printed instead of written to disk.
    #[structopt(long)]
    pub dry_run: bool,

    /// The path to a Cargo binary to use for gathering metadata
    #[structopt(long, env = "CARGO")]
    pub cargo: PathBuf,

    /// The path to a rustc binary for use with Cargo
    #[structopt(long, env = "RUSTC")]
    pub rustc: PathBuf,
}

/// Combine a set of disjoint manifests into a single workspace.
pub fn splice(opt: SpliceOptions) -> Result<()> {
    // Load the "Splicing manifest"
    let splicing_manifest = {
        let content = fs::read_to_string(opt.splicing_manifest)?;
        SplicingManifest::from_str(&content)?
    };

    // Generate a splicer for creating a Cargo workspace manifest
    let splicer = Splicer::new(
        opt.workspace_dir,
        splicing_manifest,
        opt.cargo_lockfile,
        opt.cargo,
        opt.rustc,
    )?;

    // Splice together the manifest
    splicer.splice_workspace()?;

    Ok(())
}
