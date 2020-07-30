use std::collections::BTreeMap;
use std::ops::Fn;
use std::rc::Rc;

use super::{FilePrototype, Object, ObjectError, ObjectPrototype};
use crate::file::read_file;
use crate::graphics::GraphicsError;
use crate::traits::{RenderInfo, Renderable};
use crate::Point3f;

pub struct ObjectManager {
    //object_shader: ShaderProgram,
    prototype_map: BTreeMap<String, Rc<ObjectPrototype>>,
    active_list: Vec<Object>,
    next_id: u32,
}

impl ObjectManager {
    pub fn from_yaml(file_path: &str) -> Result<ObjectManager, ObjectError> {
        info!("Creating object manager by yaml, path = '{}'", file_path);
        let file = read_file(file_path)?;
        let parsed_file: FilePrototype = serde_yaml::from_str(file.as_str())?;

        let mut obj_manager = ObjectManager::default();

        for (name, lod0_path, lod1_path) in parsed_file.into_iter() {
            info!(
                "Loading prototype '{}', lod0 = '{}', lod1 = '{}'",
                name, lod0_path, lod1_path
            );
            obj_manager.add_prototype(&name, &lod0_path, &lod1_path)?;
        }

        Ok(obj_manager)
    }

    pub fn count_active_objects(&self) -> usize {
        self.active_list.len()
    }

    pub fn add_prototype(
        &mut self,
        name: &str,
        lod0_path: &str,
        lod1_path: &str,
    ) -> Result<(), ObjectError> {
        debug_assert!(!self.prototype_map.contains_key(name));
        let prototype = ObjectPrototype::from_obj(lod0_path, lod1_path)?;
        self.prototype_map
            .insert(name.to_string(), Rc::new(prototype));
        Ok(())
    }

    pub fn create_object(&mut self, prototype_name: &str, persistent: bool) -> Result<u32, ObjectError> {
        match self.prototype_map.get(prototype_name) {
            Some(proto) => {
                let mut obj = Object::new(self.next_id, proto.clone());
                if persistent {
                    obj.set_persitent();
                }
                self.active_list
                    .push(obj);
                self.next_id += 1;
                Ok(self.next_id - 1)
            }
            None => Err(ObjectError::PrototypeNotExisting(prototype_name.to_string()).into()),
        }
    }

    pub fn mod_object<F: Fn(&mut Object)>(&mut self, id: u32, func: F) -> bool {
        match self.get_object_mut(id) {
            Some(obj) => {
                func(obj);
                true
            }
            None => false,
        }
    }

    fn get_object_mut(&mut self, id: u32) -> Option<&mut Object> {
        for e in &mut self.active_list {
            if e.get_id() == id {
                return Some(e);
            }
        }
        None
    }

    fn sort_by_id(&mut self) {
        self.active_list.sort_by_key(|k| k.get_id());
    }

    pub fn unload_by_list(&mut self, ids: &[u32]) {
        self.sort_by_id();
        for id in ids.iter() {
            match self.active_list.binary_search_by_key(id, |k| k.get_id()) {
                Ok(index) => {
                    self.active_list.remove(index);
                }
                Err(_e) => {}
            }
        }
    }

    pub fn unload_distant(&mut self, center: Point3f, distant: f32) -> usize {
        let begin_len = self.active_list.len();
        self.active_list
            .retain(|o| o.get_distance(center) < distant || o.is_persistent());
        begin_len - self.active_list.len()
    }
}

impl Renderable for ObjectManager {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        //info.push_shader(self.surface_shader_program.clone());

        for obj in &self.active_list {
            obj.render(info)?;
        }

        //info.pop_shader();

        Ok(())
    }
}

impl Default for ObjectManager {
    fn default() -> ObjectManager {
        ObjectManager {
            prototype_map: BTreeMap::new(),
            active_list: Vec::new(),
            next_id: 0,
        }
    }
}
