use gdnative::prelude::{godot_init, InitHandle};

mod loader;

fn init(handle: InitHandle) {
    handle.add_class::<loader::CubismLoader>();
}

godot_init!(init);
