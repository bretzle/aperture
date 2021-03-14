use crate::{geometry::*, spectrum::Spectrum, utils::resolve_filename};
use std::{cell::Cell, fmt::Debug};

#[derive(Debug, Default, Clone)]
pub struct ParamSet {
    bools: Vec<ParamSetItem<bool>>,
    ints: Vec<ParamSetItem<i32>>,
    floats: Vec<ParamSetItem<f32>>,
    strings: Vec<ParamSetItem<String>>,
    spectra: Vec<ParamSetItem<Spectrum>>,
    point2fs: Vec<ParamSetItem<Point2f>>,
    point3fs: Vec<ParamSetItem<Point3f>>,
    vector3fs: Vec<ParamSetItem<Vector3f>>,
    normal3fs: Vec<ParamSetItem<Normal3f>>,
    textures: Vec<ParamSetItem<String>>,
}

macro_rules! find_one {
    ($x:ident, $y:ident, $t:ty) => {
        pub fn $x(&self, name: &str, d: $t) -> $t {
            let res = self.$y.iter().find(|ref e| e.name == name);

            if let Some(e) = res.as_ref() {
                e.looked_up.set(true);
            }

            res.map(|e| e.values[0].clone()).unwrap_or(d)
        }
    };
}

macro_rules! find(
    ($x:ident, $y:ident, $t:ty) => (
        pub fn $x(&self, name: &str) -> Option<Vec<$t>> {
            let res = self.$y.iter().find(|ref e| e.name == name);

            if let Some(e) = res.as_ref() {
                e.looked_up.set(true);
            }

            res.map(|e| e.values.clone())
        }
    );
);

impl ParamSet {
    find!(find_bool, bools, bool);
    find!(find_int, ints, i32);
    find!(find_float, floats, f32);
    find!(find_string, strings, String);
    find!(find_spectrum, spectra, Spectrum);
    find!(find_point2f, point2fs, Point2f);
    find!(find_point3f, point3fs, Point3f);
    find!(find_vector3f, vector3fs, Vector3f);
    find!(find_normal3f, normal3fs, Normal3f);
    find_one!(find_one_bool, bools, bool);
    find_one!(find_one_int, ints, i32);
    find_one!(find_one_float, floats, f32);
    find_one!(find_one_string, strings, String);
    find_one!(find_one_spectrum, spectra, Spectrum);
    find_one!(find_one_point2f, point2fs, Point2f);
    find_one!(find_one_point3f, point3fs, Point3f);
    find_one!(find_one_vector3f, vector3fs, Vector3f);
    find_one!(find_one_normal3f, normal3fs, Normal3f);

    find_one!(find_texture, textures, String);

    pub fn find_one_filename(&self, name: &str, d: String) -> String {
        let filename = self.find_one_string(name, "".to_owned());
        if filename == "" {
            d
        } else {
            resolve_filename(&filename)
        }
    }
}

#[derive(Debug, Clone)]
struct ParamSetItem<T: Debug> {
    name: String,
    values: Vec<T>,
    looked_up: Cell<bool>,
}

impl<T: Debug> Default for ParamSetItem<T> {
    fn default() -> Self {
        Self {
            name: String::new(),
            values: Vec::new(),
            looked_up: Cell::new(false),
        }
    }
}
