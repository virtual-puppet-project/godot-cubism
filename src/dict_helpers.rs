use cubism::{
    core::{ConstantFlags, Drawable, DynamicFlags, Parameter, Part},
    json::model::Motion,
};
use gdnative::prelude::{Dictionary, Vector2};

pub fn create_dict_from_motion(m: &Motion) -> Dictionary {
    let d = Dictionary::new();

    d.insert("file", m.file.to_str().unwrap_or("invalid"));
    d.insert("fade_in_time", m.fade_in_time);
    d.insert("fade_out_time", m.fade_out_time);

    d.into_shared()
}

pub fn create_dict_from_part(part: &Part) -> Dictionary {
    let d = Dictionary::new();

    d.insert("id", part.id);
    d.insert("opacity", part.opacity);

    d.into_shared()
}

pub fn create_dict_from_parameter(parameter: &Parameter) -> Dictionary {
    let d = Dictionary::new();

    d.insert("id", parameter.id);
    d.insert("value", parameter.value);
    d.insert("min_value", parameter.min_value);
    d.insert("max_value", parameter.max_value);
    d.insert("default_value", parameter.default_value);

    d.into_shared()
}

pub fn create_dict_from_drawable(drawable: &Drawable) -> Dictionary {
    let d = Dictionary::new();

    d.insert("index", drawable.index as i32);
    d.insert("render_order", drawable.render_order);
    d.insert("draw_order", drawable.draw_order);
    d.insert("texture_index", drawable.texture_index);
    d.insert::<_, Vec<u16>>("indices", drawable.indices.to_vec());
    d.insert::<_, Vec<Vector2>>(
        "vertex_positions",
        drawable
            .vertex_positions
            .iter()
            .map(|x| Vector2::new(x[0], x[1])) // TODO this seems dangerous
            .collect(),
    );
    d.insert::<_, Vec<Vector2>>(
        "vertex_uvs",
        drawable
            .vertex_uvs
            .iter()
            .map(|x| Vector2::new(x[0], x[1])) // TODO this seems dangerous
            .collect(),
    );
    d.insert("opacity", drawable.opacity);
    d.insert::<_, Vec<i32>>("masks", drawable.masks.to_vec());
    d.insert("constant_flags", drawable.constant_flags.bits());
    d.insert(
        "constant_flags_string",
        format!("{:?}", drawable.constant_flags),
    );
    d.insert(
        "constant_flags_binary",
        format!("{:#b}", drawable.constant_flags),
    );
    d.insert(
        "constant_flags_hex",
        format!("{:#x}", drawable.constant_flags),
    );
    d.insert("dynamic_flags", drawable.dynamic_flags.bits());
    d.insert(
        "dynamic_flags_string",
        format!("{:?}", drawable.dynamic_flags),
    );
    d.insert(
        "dynamic_flags_binary",
        format!("{:#b}", drawable.dynamic_flags),
    );
    d.insert(
        "dynamic_flags_hex",
        format!("{:#x}", drawable.dynamic_flags),
    );

    d.into_shared()
}
