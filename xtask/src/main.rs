use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::error::{AppError, AppResult, ErrorCode};
use crate::issues::lint_issues;

mod common;
mod error;
mod issues;

pub struct WorkFiles {
    pub frontmatter: PathBuf,
    pub taxonomy_db: PathBuf,
    pub issues_dir: PathBuf,
}

#[derive(Parser)]
#[command(name = "xtask", about = "Task runner for multi-widget")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint .issues/ files against frontmatter.json rules
    LintIssues,
}

fn find_work_files() -> AppResult<WorkFiles> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("Cargo.toml").exists() && dir.join(".issues").is_dir() {
            return Ok(WorkFiles {
                frontmatter: dir.join(".frontmatter/config/frontmatter.json"),
                taxonomy_db: dir.join(".frontmatter/database/taxonomyDb.json"),
                issues_dir: dir.join(".issues"),
            });
        }
        if !dir.pop() {
            return Err(AppError::WorkspaceRootNotFound);
        }
    }
}

fn main() {
    let _ = enable_ansi_support::enable_ansi_support();
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("error: {e}");
        std::process::exit(e.exit_code());
    }
}

fn run(cli: Cli) -> AppResult<()> {
    let files = find_work_files()?;
    match cli.command {
        Commands::LintIssues => lint_issues(files),
    }
}
