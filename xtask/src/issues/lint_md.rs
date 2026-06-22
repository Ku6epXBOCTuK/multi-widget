use std::process::Command;

use crate::common::{GREEN, RED, RESET, find_files_by_ext};
use crate::error::{AppError, AppResult};
use crate::issues::parse_frontmatter_json::ValidationRules;
use crate::issues::validate_rules::validate_file;
use crate::{WorkFiles, common::read_file};

pub(super) struct FileContent {
    pub(super) filename: String,
    pub(super) content: String,
}

pub fn lint_issues(work_files: WorkFiles) -> AppResult<()> {
    let rules = ValidationRules::load(&work_files.frontmatter, &work_files.taxonomy_db)?;

    let files = find_files_by_ext(&work_files.issues_dir, "md")?;

    if files.is_empty() {
        println!("No .md files found in {}", work_files.issues_dir.display());
        return Ok(());
    }

    let mut total_errors: usize = 0;

    for path in &files {
        let content = read_file(path)?;
        let filename = path.display().to_string();
        let file = FileContent { filename, content };
        if let Err(e) = validate_file(file, &rules) {
            eprintln!("  {RED}✗ {e}{RESET}");
            total_errors += 1;
        }
    }

    println!("\nRunning markdown lint...");
    match Command::new("cmd")
        .args(["/C", "npm", "run", "lint:issues:md"])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                total_errors += 1;
            }
        }
        Err(e) => {
            eprintln!("  {RED}⚠ skipping issues markdown lint: {e}{RESET}");
        }
    }

    println!("\n── Summary ──");
    println!("Files checked: {}", files.len());
    if total_errors == 0 {
        println!("{GREEN}Errors: 0 — all good!{RESET}");
        Ok(())
    } else {
        println!("{RED}Errors: {total_errors}{RESET}");
        Err(AppError::LintIssuesFailed {
            count: total_errors,
            files: files.len(),
        })
    }
}
