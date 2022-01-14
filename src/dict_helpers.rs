use cubism::{
    core::{ConstantFlags, Drawable, DynamicFlags, Parameter, Part},
    json::{
        model::Motion,
        motion::{Motion3, Segment, SegmentPoint},
    },
};
use gdnative::prelude::{Dictionary, ToVariant, VariantArray, Vector2};

pub fn create_dict_from_motion(m: &Motion) -> Dictionary {
    let d = Dictionary::new();

    d.insert("file", m.file.to_str().unwrap_or("invalid"));
    d.insert("fade_in_time", m.fade_in_time);
    d.insert("fade_out_time", m.fade_out_time);

    d.into_shared()
}

pub fn create_dict_from_motion3(m: &Motion3) -> Dictionary {
    let d = Dictionary::new();

    d.insert("version", m.version);
    d.insert("meta", {
        let d = Dictionary::new();

        let meta = m.meta;

        d.insert("duration", meta.duration);
        d.insert("fps", meta.fps);
        d.insert("looped", meta.looped);
        d.insert("restricted_beziers", meta.restricted_beziers);
        d.insert("curve_count", meta.curve_count);
        d.insert("total_segment_count", meta.total_segment_count);
        d.insert("total_point_count", meta.total_point_count);
        d.insert("user_data_count", meta.user_data_count);
        d.insert("total_user_data_size", meta.total_user_data_size);

        d.into_shared()
    });
    d.insert::<&str, Vec<Dictionary>>(
        "curves",
        m.curves
            .iter()
            .map(|c| {
                let d = Dictionary::new();

                d.insert("target", c.target.to_string());
                d.insert("id", c.id.to_string());
                d.insert::<&str, Vec<Dictionary>>(
                    "segments",
                    c.segments
                        .iter()
                        .map(|s| {
                            let dict = Dictionary::new();
                            match s {
                                Segment::Linear(sp_a, sp_b) => {
                                    dict.insert("type", "linear");

                                    let d = Dictionary::new();

                                    d.insert(
                                        "segment_point_a",
                                        create_dict_from_segment_point(sp_a),
                                    );
                                    d.insert(
                                        "segment_point_b",
                                        create_dict_from_segment_point(sp_b),
                                    );

                                    dict.insert("value", d.into_shared());
                                }
                                Segment::Bezier(a) => {
                                    dict.insert("type", "bezier");

                                    let va = VariantArray::new();

                                    for sp in a.iter() {
                                        va.push(create_dict_from_segment_point(sp));
                                    }

                                    dict.insert("value", va.into_shared())
                                }
                                Segment::Stepped(sp, f) => {
                                    dict.insert("type", "stepped");

                                    let d = Dictionary::new();

                                    d.insert("segment_point", create_dict_from_segment_point(sp));
                                    d.insert("value", f);

                                    dict.insert("value", d.into_shared());
                                }
                                Segment::InverseStepped(f, sp) => {
                                    dict.insert("type", "inverse_stepped");

                                    let d = Dictionary::new();

                                    d.insert("value", f);
                                    d.insert("segment_point", create_dict_from_segment_point(sp));

                                    dict.insert("value", d.into_shared());
                                }
                            }
                            dict.into_shared()
                        })
                        .collect(),
                );

                d.into_shared()
            })
            .collect(),
    );

    d.into_shared()
}

pub fn create_dict_from_segment_point(sp: &SegmentPoint) -> Dictionary {
    let d = Dictionary::new();

    d.insert("time", sp.time);
    d.insert("value", sp.value);

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
