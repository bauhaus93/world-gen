use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileAsset {
    name: String,
    lod0: String,
    lod1: String
}

impl FileAsset {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_lod0_path(&self) -> &str {
        &self.lod0
    }
    pub fn get_lod1_path(&self) -> &str {
        &self.lod1
    }
}