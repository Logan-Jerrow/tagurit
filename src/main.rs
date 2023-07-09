#![warn(
    // clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use crate::mp3::Mp3;
use clap::Parser;
use std::path::{Path, PathBuf};

mod mp3;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    path: Option<PathBuf>,
}

#[must_use]
fn get_mp3_files(path: &Path) -> Box<[PathBuf]> {
    path.read_dir()
        .expect("read directory failed")
        .flatten()
        .map(|dir| dir.path())
        .filter(|p| Mp3::try_from(p.as_path()).is_ok())
        .collect()
}

fn main() {
    let cli = Cli::parse();
    let path = get_mp3_files(
        &cli.path
            .as_deref()
            .and_then(|o| o.canonicalize().ok())
            .expect("invalid path"),
    );

    dbg!(path);
}
