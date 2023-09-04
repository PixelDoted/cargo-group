use std::{path::PathBuf, process::Command};

use clap::Parser;
use cli::Commands;
use workspace::Cargo;

mod cli;
mod git;
mod util;
mod workspace;

fn main() {
    // Cargo Subcommand
    let mut args = std::env::args().skip(1).peekable();
    if let Some(arg) = args.peek() {
        if arg != "group" {
            args.next().unwrap();
        }
    }

    // Parse Arguments
    let command = cli::Commands::parse_from(args);

    // Execute Command
    match command {
        Commands::Init { resolver } => {
            if PathBuf::from("./Cargo.toml").exists() {
                eprintln!("Cargo.toml already exists");
                return;
            }

            let cargo = Cargo::new(resolver);
            cargo.write(&PathBuf::from("./Cargo.toml"));
            git::init("./");
            println!("Created workspace.");
        }

        Commands::Add {
            path,
            lib,
            edition,
            name,
            offline,
        } => {
            let paths = util::find_workspace(&path);
            let mut cargo = Cargo::read(&paths.workspace);
            if cargo.workspace.members.contains(&paths.relative) {
                eprintln!("A member with that path already exsists.");
                return;
            }

            let mut args = vec!["new", path.to_str().unwrap(), "--vcs", "none"];
            if lib {
                args.push("--lib");
            }

            args.push("--edition");
            match edition {
                cli::Edition::Y2015 => args.push("2015"),
                cli::Edition::Y2018 => args.push("2018"),
                cli::Edition::Y2021 => args.push("2021"),
            }

            if let Some(name) = name.as_ref() {
                args.push("--name");
                args.push(name);
            }

            if offline {
                args.push("--offline");
            }

            // Run Cargo new
            Command::new("cargo")
                .args(args)
                .output()
                .expect("Failed to run cargo new.");

            // Update workspace
            cargo.workspace.members.push(paths.relative.clone());
            cargo.write(&paths.workspace);
            println!("added \"{}\".", paths.relative);
        }
        Commands::AddPath { path } => {
            let paths = util::find_workspace(&path);
            let mut cargo = Cargo::read(&paths.workspace);
            if cargo.workspace.members.contains(&paths.relative) {
                eprintln!("A member with that path already exsists.");
                return;
            }

            cargo.workspace.members.push(paths.relative.clone());
            cargo.write(&paths.workspace);
            println!("Added \"{}\" to workspace.", paths.relative);
        }

        Commands::Remove { path } => {
            let path = path.unwrap_or(PathBuf::from("./"));
            util::confirm(&format!(
                "Are you sure you want to delete {:?}?",
                util::file_name(&path)
            ));

            let paths = util::find_workspace(&path);
            let mut cargo = Cargo::read(&paths.workspace);
            if !cargo.workspace.members.contains(&paths.relative) {
                eprintln!("Member does not exist.");
                return;
            }

            for i in 0..cargo.workspace.members.len() {
                if cargo.workspace.members[i] != paths.relative {
                    continue;
                }

                cargo.workspace.members.remove(i);
                std::fs::remove_dir_all(path).unwrap();
                cargo.write(&paths.workspace);
                println!("Removed \"{}\".", paths.relative);
                break;
            }
        }
        Commands::RemovePath { path } => {
            let paths = util::find_workspace(&path);
            let mut cargo = Cargo::read(&paths.workspace);
            if !cargo.workspace.members.contains(&paths.relative) {
                eprintln!("Member does not exist.");
                return;
            }

            for i in 0..cargo.workspace.members.len() {
                if cargo.workspace.members[i] != paths.relative {
                    continue;
                }

                cargo.workspace.members.remove(i);
                println!("Removed \"{}\" from workspace.", paths.relative);
                cargo.write(&paths.workspace);
                break;
            }
        }
    }
}
