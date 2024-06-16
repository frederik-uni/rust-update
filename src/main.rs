use std::{
    collections::HashMap,
    env,
    path::Path,
    process::{exit, Command},
};

use clap::Parser as _;
use cli::Cli;
use self_update::cargo_crate_version;
use version::Version;

mod cli;
mod version;

fn main() {
    let cli = Cli::parse();
    // check if migrating
    match cli.migrate {
        Some(version) => match Version::try_from(version.as_str()) {
            Ok(ver) => {
                // get migration scripts & runs them in order
                let migrations = migration_scripts();
                let mut versions = migrations
                    .into_iter()
                    .filter(|(v, _)| *v > ver)
                    .collect::<Vec<_>>();
                versions.sort_by(|(ver_a, _), (ver_b, _)| ver_a.cmp(ver_b));
                for (_, script) in versions {
                    let _ = Command::new(script).output().unwrap();
                }
            }
            Err(_) => {
                panic!("Failed to parse version")
            }
        },
        None => {
            // downloads new version and runs new binary
            let need_restart = update("frederik-uni", "rust-update", "rust-update").unwrap();
            if need_restart {
                drop(cli);
                Command::new(env::current_exe().unwrap()).spawn().unwrap();
                exit(0);
            }
        }
    }
    println!("Hello World")
}

fn migration_scripts() -> HashMap<Version, &'static Path> {
    let mut migrations = HashMap::new();
    migrations.insert(Version::new(0, 0, 1), Path::new("./scripts/migration1"));
    migrations
}

fn update(owner: &str, repo: &str, bin: &str) -> Result<bool, Box<dyn ::std::error::Error>> {
    let mut status_builder = self_update::backends::github::Update::configure();
    let status = status_builder
        .repo_owner(owner)
        .repo_name(repo)
        .bin_name(bin)
        .show_output(false)
        .show_download_progress(true)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    Ok(status.updated())
}
