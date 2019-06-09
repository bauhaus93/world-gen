use std::collections::BTreeMap;
use std::sync::Arc;

use super::{ Object, ObjectPrototype, ObjectError };

pub struct ObjectManager {
    prototype_map: BTreeMap<String, Arc<ObjectPrototype>>
}


impl ObjectManager {

    pub fn add_prototype(&mut self, name: &str, lod0_path: &str, lod1_path: &str) -> Result<(), ObjectError> {
        debug_assert!(!self.prototype_map.contains_key(name));
        let prototype = ObjectPrototype::from_obj(lod0_path, lod1_path)?;
        self.prototype_map.insert(name.to_string(), Arc::new(prototype));
        Ok(())
    }

    pub fn create_object(&self, prototype_name: &str) -> Result<Object, ObjectError> {
        match self.prototype_map.get(prototype_name) {
            Some(proto) => {
                let obj = Object::new(proto.clone());
                Ok(obj)
            },
            None => {
                Err(ObjectError::PrototypeNotExisting(prototype_name.to_string()).into())
            }
        }
    }

}

impl Default for ObjectManager {
    fn default() -> ObjectManager {
        ObjectManager {
            prototype_map: BTreeMap::new()
        }
    }
}

