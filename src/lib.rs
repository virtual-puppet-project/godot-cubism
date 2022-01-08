use gdnative::prelude::{godot_init, InitHandle};

mod dict_helpers;
mod loader;

fn init(handle: InitHandle) {
    handle.add_class::<loader::CubismLoader>();
    handle.add_class::<loader::CubismLoaderFactory>();
}

godot_init!(init);
