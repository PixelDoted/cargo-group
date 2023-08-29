use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cargo {
    pub workspace: Workspace,
}

impl Cargo {
    pub fn new(resolver: String) -> Self {
        Self {
            workspace: Workspace {
                resolver,
                members: Vec::new(),
            },
        }
    }

    pub fn read(path: &PathBuf) -> Self {
        let data = std::fs::read(path).expect("No workspace found.");
        let s = std::str::from_utf8(&data).expect("File is binary");
        toml::from_str(s).expect("Not a workspace.")
    }

    pub fn write(self, path: &PathBuf) {
        let data = toml::to_string_pretty(&self).expect("Failed to convert to toml.");
        std::fs::write(path, data).expect("Failed to write workspace.");
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Workspace {
    pub resolver: String,
    pub members: Vec<String>,
}
