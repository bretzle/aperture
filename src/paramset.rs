use std::{cell::Cell, fmt::Debug};

#[derive(Debug, Default, Clone)]
pub struct ParamSet {
    floats: Vec<ParamSetItem<f32>>,
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

impl ParamSet {
    find_one!(find_one_float, floats, f32);
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
