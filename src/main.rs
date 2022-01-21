use clap::{AppSettings::DisableHelpSubcommand, Parser, Subcommand};
use email_address::EmailAddress;
use reqwest::Url;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(DisableHelpSubcommand))]
struct Cli {
    /// API URL to use, defaults to https://paste.misterio.me
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
    /// Uploads a file and creates a new paste
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
    /// Downloads a given paste
    #[clap(alias = "d", alias = "down")]
    Download { id: Uuid },
    /// Deletes a given paste
    #[clap(alias = "del")]
    Delete { id: Uuid },
    /// Registers a new account. Password is read from STDIN
    Register {
        username: String,
        email: EmailAddress,
    },
    /// Logs into your account. Password is read from STDIN
    Login { username: String },
    /// Logs out of your account
    Logout {
        /// If specified, will revoke all other sessions as well.
        #[clap(short, long)]
        all: bool,
    },
    /// Shows info on a given user (or self, if owner is ommited)
    User { username: Option<String> },
}

use pmis::operations;

fn main() {
    let cli = Cli::parse();

    let api = cli.api;

    match cli.command {
        Commands::List { owner } => operations::list(api, owner),
        Commands::Upload {
            file,
            title,
            description,
            unlisted,
        } => operations::upload(api, file, title, description, unlisted),
        Commands::Download { id } => operations::download(api, id),
        Commands::Delete { id } => operations::delete(api, id),
        Commands::Register { username, email } => operations::register(api, username, email),
        Commands::Login { username } => operations::login(api, username),
        Commands::Logout { all } => operations::logout(api, all),
        Commands::User { username } => operations::user(api, username),
    }
}
