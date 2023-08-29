use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub enum Commands {
    /// Initialize a workspace
    Init {
        #[arg(short, long, default_value = "2")]
        /// The resolver this workspace will use
        resolver: String,
    },
    /// Creates a new crate and adds it as a member to this workspace
    Add {
        path: PathBuf,

        #[arg(long)]
        lib: bool,

        #[arg(long, default_value = "2021")]
        edition: Edition,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        offline: bool,
    },
    AddPath {
        path: PathBuf,
    },
    Remove {
        path: Option<PathBuf>,
    },
    RemovePath {
        path: PathBuf,
    },
}

#[derive(ValueEnum, Clone)]
pub enum Edition {
    #[value(alias("2015"))]
    Y2015,

    #[value(alias("2018"))]
    Y2018,

    #[value(alias("2021"))]
    Y2021,
}
