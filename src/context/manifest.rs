use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub format: String,
    pub version: String,
    pub created_at: String,
    pub generated_by: String,
    pub context_files: Vec<String>,
}

impl Manifest {
    pub fn new(files: &[String]) -> Self {
        Manifest {
            format: "openspore-context".to_string(),
            version: "0.1".to_string(),
            created_at: Utc::now().to_rfc3339(),
            generated_by: "openspore".to_string(),
            context_files: files.iter().map(|f| format!("context/{f}")).collect(),
        }
    }
}
