use serde::Deserialize;

use super::FileAsset;

#[derive(Deserialize)]
pub struct FilePrototype {
    root_dir: String,
    asset_list: Vec<FileAsset>,

}

pub struct PrototypeIterator {
    prototype_file: FilePrototype,
    curr_index: usize
}

impl FilePrototype {
    pub fn get_root_dir(&self) -> &str {
        &self.root_dir
    }
    pub fn get_asset(&self, index: usize) -> Option<&FileAsset> {
        self.asset_list.get(index)
    }
}

impl IntoIterator for FilePrototype {
    type Item = (String, String, String);
    type IntoIter = PrototypeIterator;

    fn into_iter(self) -> Self::IntoIter {
        PrototypeIterator {
            prototype_file: self,
            curr_index: 0
        }
    }
}

impl Iterator for PrototypeIterator {
    type Item = (String, String, String);
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.prototype_file.get_asset(self.curr_index) {
            Some(asset) => {
                let result = (asset.get_name().to_owned(),
                              self.prototype_file.get_root_dir().to_owned() + "/" + asset.get_lod0_path(),
                              self.prototype_file.get_root_dir().to_owned() + "/" + asset.get_lod1_path());
                self.curr_index += 1;
                Some(result)
            },
            _ => None
        }
    }
}
