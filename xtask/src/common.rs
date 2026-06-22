pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const RESET: &str = "\x1b[0m";

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::error::{AppError, AppResult};

pub(crate) fn read_file(path: &Path) -> AppResult<String> {
    let content = fs::read_to_string(path).map_err(|source| AppError::FileRead {
        path: path.display().to_string(),
        source,
    })?;
    Ok(content)
}

pub(crate) fn find_files_by_ext(dir: &Path, ext: &'static str) -> AppResult<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();
    let entries = fs::read_dir(&dir).map_err(AppError::IssuesDirRead)?;
    for entry in entries {
        let entry = entry.map_err(AppError::IssuesDirRead)?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|e| e == ext) {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}
