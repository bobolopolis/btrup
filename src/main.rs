// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]

struct Args {

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List installed firmware
    List {
    },
    /// Check for updated firmware online
    Check {
    },
    /// Delete the installed firmware specified
    Delete {
        /// UUID of the firmware to delete, or "all" to delete all except
        /// the presently running firmware.
        uuid: String,
    },
    /// Upgrade firmware
    Upgrade {
        /// Upgrade file to use. If not specified, the latest is used from
        /// the website.
        file: Option<std::path::PathBuf>,
    },
}

fn main() {
    let arg = Args::parse();

    println!("Hello, world!");
    match &arg.command {
        Commands::Check { } => {
            println!("check selected")
        }
        Commands::Delete { uuid } => {
            println!("delete selected: {:?}", uuid)
        }
        Commands::List { } => {
            println!("list selected")
        }
        Commands::Upgrade { file } => {
            println!("upgrade selected: {:?}", file)
        }
    }
}
