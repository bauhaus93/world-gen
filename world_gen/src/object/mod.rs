
pub mod object_manager;
pub mod object_prototype;
pub mod object;
pub mod object_error;

pub use self::object_manager::ObjectManager;
pub use self::object::Object;
use self::object_prototype::ObjectPrototype;
use self::object_error::ObjectError;