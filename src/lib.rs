use gdnative::prelude::{godot_init, InitHandle};

mod dict_helpers;
mod loader;

fn init(handle: InitHandle) {
    handle.add_class::<loader::CubismModel>();
    handle.add_class::<loader::CubismModelFactory>();
}

godot_init!(init);
