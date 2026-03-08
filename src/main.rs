// src/main.rs

use urtext::build::pipeline;
use urtext::content::archetypes;
use urtext::deploy::sync;
use urtext::server::routes;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "urtext")]
#[command(about = "Static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site
    Build {
        /// Include draft posts
        #[arg(long)]
        drafts: bool,
    },

    /// Serve the site locally
    Serve {
        /// Port to serve on
        #[arg(long, default_value_t = 1111)]
        port: u16,
    },

    /// Deploy the site
    Deploy {
        /// Deployment target
        #[arg(long)]
        target: Option<String>,
    },

    /// Create new content
    Create {
        /// Content kind
        #[arg(long)]
        kind: String,

        /// Content title
        #[arg(long)]
        title: String,
    },

    /// Make a new site
    New {
        /// Site name
        #[arg(long)]
        sitename: String,
    },
}

impl Commands {
    pub async fn run(self) -> Result<()> {
        match self {
            Commands::Build { drafts } => pipeline::build(drafts),
            Commands::Serve { port } => routes::serve(port),
            Commands::Deploy { target } => sync::transfer(target),
            Commands::Create { kind, title } => archetypes::new_post(kind, title),
            Commands::New { sitename } => archetypes::new_site(sitename),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.run().await
}
