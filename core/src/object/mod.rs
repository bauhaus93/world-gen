
pub mod object_manager;
pub mod object_prototype;
pub mod object;
pub mod object_error;
mod file_prototype;
mod file_asset;

pub use self::object_manager::ObjectManager;
pub use self::object::Object;
pub use self::object_error::ObjectError;
use self::object_prototype::ObjectPrototype;
use self::file_prototype::FilePrototype;
use self::file_asset::FileAsset;
