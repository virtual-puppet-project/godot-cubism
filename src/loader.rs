use cubism::{
    json::model::{
        Expression, FileReferences, Group, GroupTarget, HitArea, Layout, Model3, Motion, Motions,
    },
    model::UserModel,
};
use gdnative::prelude::*;
use std::{fs::File, path::PathBuf};

#[derive(NativeClass, Copy, Clone, Default)]
#[user_data(MutexData<CubismLoaderFactory>)]
#[inherit(Reference)]
pub struct CubismLoaderFactory;

#[methods]
impl CubismLoaderFactory {
    fn new(_owner: &Reference) -> Self {
        Self
    }

    #[export]
    pub fn cubism_loader(
        &self,
        _owner: &Reference,
        path: String,
    ) -> Instance<CubismLoader, Unique> {
        let res_path = PathBuf::from(path);

        let json = Model3::from_reader(File::open(&res_path).expect("Unable to open file"))
            .expect("Unable to read model3 json");

        let model = UserModel::from_model3(&res_path, &json).expect("Unable to load model");

        CubismLoader {
            model,
            json,
            res_path,
        }
        .emplace()
    }
}

// #[derive(NativeClass)]
// #[inherit(Reference)]
// #[user_data(user_data::MutexData<CubismLoader>)]
// pub struct CubismLoader {
//     model: Option<UserModel>,
//     json: Option<Model3>,
//     res_path: Option<PathBuf>, // This might be a relative path?
// }
#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constructor]
#[user_data(user_data::MutexData<CubismLoader>)]
pub struct CubismLoader {
    model: UserModel,
    json: Model3,
    res_path: PathBuf, // This might be a relative path?
}

unsafe impl Sync for CubismLoader {}
unsafe impl Send for CubismLoader {}

#[methods]
impl CubismLoader {
    // fn new(_owner: &Reference, path: String) -> Self {
    //     let res_path = PathBuf::from(path);

    //     let json = Model3::from_reader(File::open(&res_path).expect("Unable to open file"))
    //         .expect("Unable to read model3 json");

    //     let model = UserModel::from_model3(&res_path, &json).expect("Unable to load model");

    //     CubismLoader {
    //         model,
    //         json,
    //         res_path,
    //     }
    // }

    /// Loads in a given model from a model3 json file
    // #[export]
    // pub fn load_model(&mut self, _owner: &Reference, path: String) {
    //     let res_path = PathBuf::from(path);

    //     let json = Model3::from_reader(File::open(&res_path).expect("Unable to open file"))
    //         .expect("Unable to read model3 json");

    //     let model = UserModel::from_model3(&res_path, &json).expect("Unable to load model");

    //     self.json = Some(json);
    //     self.model = Some(model);
    //     self.res_path = Some(res_path);
    // }

    #[export]
    pub fn get_json(&self, _owner: &Reference) -> Dictionary {
        let d = Dictionary::new();

        // let json = self.json.as_ref().expect("Unable to get json");
        let json = &self.json;

        d.insert("version", json.version as i32);
        d.insert("file_references", {
            let data = &json.file_references;
            let d = Dictionary::new();

            d.insert(
                "moc",
                data.moc
                    .as_ref()
                    .unwrap_or(&PathBuf::from("invalid"))
                    .to_str()
                    .unwrap_or("invalid"),
            );
            d.insert::<_, Vec<&str>>(
                "textures",
                data.textures
                    .iter()
                    .map(|x| x.to_str().unwrap_or("invalid"))
                    .collect(),
            );
            d.insert(
                "pose",
                data.pose
                    .as_ref()
                    .unwrap_or(&PathBuf::from("invalid"))
                    .to_str()
                    .unwrap_or("invalid"),
            );
            d.insert(
                "physics",
                data.physics
                    .as_ref()
                    .unwrap_or(&PathBuf::from("invalid"))
                    .to_str()
                    .unwrap_or("invalid"),
            );
            d.insert::<_, Vec<Dictionary>>(
                "expressions",
                data.expressions
                    .iter()
                    .map(|x| {
                        let d = Dictionary::new();

                        d.insert("name", x.name.clone());
                        d.insert("file", x.file.to_str().unwrap_or("invalid"));

                        d.into_shared()
                    })
                    .collect(),
            );
            d.insert("motions", {
                let data = &data.motions;
                let d = Dictionary::new();

                d.insert::<_, Vec<Dictionary>>(
                    "idle",
                    data.idle
                        .iter()
                        .map(|x| create_dict_from_motion(&x))
                        .collect(),
                );
                d.insert::<_, Vec<Dictionary>>(
                    "tap_body",
                    data.tap_body
                        .iter()
                        .map(|x| create_dict_from_motion(&x))
                        .collect(),
                );
                d.insert::<_, Vec<Dictionary>>(
                    "pinch_in",
                    data.pinch_in
                        .iter()
                        .map(|x| create_dict_from_motion(&x))
                        .collect(),
                );
                d.insert::<_, Vec<Dictionary>>(
                    "pinch_out",
                    data.pinch_out
                        .iter()
                        .map(|x| create_dict_from_motion(&x))
                        .collect(),
                );
                d.insert::<_, Vec<Dictionary>>(
                    "shake",
                    data.shake
                        .iter()
                        .map(|x| create_dict_from_motion(&x))
                        .collect(),
                );
                d.insert::<_, Vec<Dictionary>>(
                    "flick_head",
                    data.flick_head
                        .iter()
                        .map(|x| create_dict_from_motion(&x))
                        .collect(),
                );

                d.into_shared()
            });
            d.insert(
                "user_data",
                data.user_data
                    .as_ref()
                    .unwrap_or(&PathBuf::from("invalid"))
                    .to_str()
                    .unwrap_or("invalid"),
            );

            d.into_shared()
        });
        d.insert::<_, Vec<Dictionary>>(
            "groups",
            json.groups
                .iter()
                .map(|x| {
                    let d = Dictionary::new();

                    d.insert(
                        "target",
                        match x.target {
                            GroupTarget::Parameter => "Parameter",
                            GroupTarget::Part => "Part",
                        },
                    );
                    d.insert("name", x.name.clone());
                    d.insert::<_, Vec<String>>("ids", x.ids.iter().map(|id| id.clone()).collect());

                    d.into_shared()
                })
                .collect(),
        );
        d.insert::<_, Vec<Dictionary>>(
            "hit_area",
            json.hit_areas
                .iter()
                .map(|ha| {
                    let d = Dictionary::new();

                    d.insert("name", ha.name.clone());
                    d.insert("id", ha.id.clone());

                    d.into_shared()
                })
                .collect(),
        );
        if let Some(data) = json.layout {
            d.insert("layout", {
                let d = Dictionary::new();

                d.insert("center_x", data.center_x);
                d.insert("center_y", data.center_y);
                d.insert("x", data.x);
                d.insert("y", data.y);
                d.insert("width", data.width);
                d.insert("height", data.height);

                d.into_shared()
            })
        }

        d.into_shared()
    }

    #[export]
    pub fn parameter(&self, _owner: &Reference, param_name: String) -> Dictionary {
        let d = Dictionary::new();

        // if let Some(param) = self.model.as_ref().unwrap().parameter(&param_name) {
        if let Some(param) = self.model.parameter(&param_name) {
            d.insert("id", param.id);
            d.insert("value", param.value);
            d.insert("min_value", param.min_value);
            d.insert("max_value", param.max_value);
            d.insert("default_value", param.default_value);
        }

        d.into_shared()
    }

    #[export]
    pub fn part(&self, _owner: &Reference, part_name: String) -> Dictionary {
        let d = Dictionary::new();

        // if let Some(part) = self.model.as_ref().unwrap().part(&part_name) {
        if let Some(part) = self.model.part(&part_name) {
            d.insert("id", part.id);
            d.insert("opacity", part.opacity);
        }

        d.into_shared()
    }

    #[export]
    pub fn drawable(&self, _owner: &Reference, drawable_name: String) -> Dictionary {
        let d = Dictionary::new();

        // if let Some(drawable) = self.model.as_ref().unwrap().drawable(&drawable_name) {
        if let Some(drawable) = self.model.drawable(&drawable_name) {
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
            d.insert("dynamic_flags", drawable.dynamic_flags.bits());
        }

        d.into_shared()
    }

    pub fn update(&mut self, _owner: &Reference, delta: f32) {
        self.model.update(delta);
    }
}

fn create_dict_from_motion(m: &Motion) -> Dictionary {
    let d = Dictionary::new();

    d.insert("file", m.file.to_str().unwrap_or("invalid"));
    d.insert("fade_in_time", m.fade_in_time);
    d.insert("fade_out_time", m.fade_out_time);

    d.into_shared()
}
