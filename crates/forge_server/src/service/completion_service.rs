use std::path::PathBuf;

use forge_walker::Walker;
use serde::Serialize;

use crate::Result;

#[derive(Serialize)]
pub struct File {
    pub path: String,
    pub is_dir: bool,
}

#[async_trait::async_trait]
pub trait CompletionService: Send + Sync {
    async fn list(&self) -> Result<Vec<File>>;
}

pub struct LiveCompletionService {
    path: String,
}

impl LiveCompletionService {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

#[async_trait::async_trait]
impl CompletionService for LiveCompletionService {
    async fn list(&self) -> Result<Vec<File>> {
        let cwd = PathBuf::from(self.path.clone()); // Use the current working directory
        let walker = Walker::new(cwd);

        let files = walker.get().await?;
        Ok(files
            .into_iter()
            .map(|file| File { path: file.path, is_dir: file.is_dir })
            .collect())
    }
}
