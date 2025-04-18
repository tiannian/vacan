use std::path::Path;

use anyhow::Result;
use tokio::fs::File;
use vacan_b0::objects::{FunctionDecl, FunctionHeader};

pub struct Generator {
    file: File,
}

impl Generator {
    pub async fn new(target_dir: &Path) -> Result<Self> {
        let file = tokio::fs::File::open(target_dir).await?;

        Ok(Self { file })
    }

    pub fn generate(&self, headers: Vec<FunctionHeader>, func: FunctionDecl) -> Result<()> {
        Ok(())
    }
}
