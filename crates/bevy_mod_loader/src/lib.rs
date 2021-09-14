use std::path::Path;
use bevy_app::{Plugin, AppBuilder};
use bevy_log::prelude::*;
use std::env;
use libloading::{Library, Symbol};
use std::any::Any;

extern crate libloading;
// Based on https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html

pub trait Mod: Any + Send + Sync {
    fn build(&self, app: &mut AppBuilder) {}
    fn teardown(&self) {}
}

#[macro_export]
macro_rules! declare_mod {
    ($mod_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _mod_create() -> *mut $crate::Mod{
            // make sure the constructor is the correct type.
            let constructor: fn() -> $mod_type= $constructor;

            let object = constructor();
            let boxed: Box<$crate::Mod> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}



pub struct ModLoaderPlugin {
}

impl ModLoaderPlugin {
    pub fn new() -> Self {
        ModLoaderPlugin {}
    }
}

impl Plugin for ModLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        unsafe {
            let mut mods: Vec<Box<Mod>> = Vec::new();
            let mut loaded_libraries: Vec<Library> = Vec::new();

            type ModCreate = unsafe fn() -> *mut Mod;

            let lib = unsafe { Library::new("mods/simple_mod/libsimple_mod.dylib") }.unwrap();

            // We need to keep the library around otherwise our plugin's vtable will
            // point to garbage. We do this little dance to make sure the library
            // doesn't end up getting moved.
            loaded_libraries.push(lib);

            let lib = loaded_libraries.last().unwrap();

            let constructor: Symbol<ModCreate> = unsafe { lib.get(b"_mod_create") }.unwrap();
            let boxed_raw = constructor();

            let plugin = unsafe { Box::from_raw(boxed_raw) };
            plugin.build(app);
            mods.push(plugin);
        }
    }
}