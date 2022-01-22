use anyhow::Result;
use clap::{AppSettings::DisableHelpSubcommand, Parser, Subcommand};
use reqwest::Url;
use uuid::Uuid;

use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(DisableHelpSubcommand))]
struct Cli {
    /// API URL to use
    #[clap(long, default_value = "https://paste.misterio.me")]
    api: Url,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists pastes from a given user (or self, if owner is ommited)
    #[clap(alias = "l", alias = "ls")]
    List { owner: Option<String> },
    /// Downloads a given paste
    #[clap(alias = "d", alias = "down")]
    Download { id: Uuid },
    /// Uploads a file and creates a new paste. Requires authentication
    #[clap(alias = "u", alias = "up")]
    Upload {
        /// File to upload. If ommited, reads from stdin
        file: Option<PathBuf>,
        /// Title for your paste. Defaults to file name or "Untitled" (if read from stdin).
        #[clap(short, long)]
        title: Option<String>,
        /// Description for your paste. Optional.
        #[clap(short, long)]
        description: Option<String>,
        /// If specified, the paste will not be listed on your profile and will only be reachable
        /// by its link.
        #[clap(short, long)]
        unlisted: bool,
    },
    /// Deletes a given paste. Requires authentication
    #[clap(alias = "del")]
    Delete { id: Uuid },
}

use pmis::operations;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api = cli.api;

    match cli.command {
        Commands::List { owner } => operations::list(api, owner),
        Commands::Download { id } => operations::download(api, id).await?,
        Commands::Upload {
            file,
            title,
            description,
            unlisted,
        } => operations::upload(api, file, title, description, unlisted),
        Commands::Delete { id } => operations::delete(api, id),
    }

    Ok(())
}
