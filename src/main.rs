// src/main.rs

use urtext::{build::pipeline, content::archetypes, deploy::sync, server::routes};

// use axum::Router;
use clap::{Parser, Subcommand};

// use urtext::server::routes;

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

        /// Minify generated HTML
        #[arg(long)]
        minify: bool,
    },

    /// Serve the site locally
    Serve {
        /// Port to serve on
        #[arg(long, default_value_t = 1111)]
        port: u16,

        /// Watch for file changes
        #[arg(long)]
        watch: bool,
    },

    /// Deploy the site
    Deploy {
        /// Deployment target
        #[arg(long)]
        target: Option<String>,
    },

    /// Create new content
    Create {
        /// Content type
        #[arg(long)]
        kind: String,

        /// Content title
        #[arg(long)]
        title: String,
    },

    /// Make new site
    New {
        /// Site name
        #[arg(long)]
        sitename: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { drafts, minify } => pipeline::build(drafts, minify),
        Commands::Serve { port, watch } => routes::serve(port, watch),
        Commands::Deploy { target } => sync::transfer(target),
        Commands::Create { kind, title } => archetypes::new_post(kind, title),
        Commands::New { sitename } => archetypes::new_site(sitename),
    }
}
