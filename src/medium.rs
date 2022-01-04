use std::sync::Arc;

pub enum Medium {}

pub struct MediumInterface {
    pub inside: Option<Arc<Medium>>,
    pub outside: Option<Arc<Medium>>,
}

impl MediumInterface {
    pub fn new(inside: Option<Arc<Medium>>, outside: Option<Arc<Medium>>) -> Self {
        Self { inside, outside }
    }

    pub fn is_medium_transition(&self) -> bool {
        if let Some(ref inside) = self.inside {
            if let Some(ref outside) = self.outside {
                let pi = &*inside as *const _ as *const usize;
                let po = &*outside as *const _ as *const usize;
                pi != po
            } else {
                true
            }
        } else {
            self.outside.is_some()
        }
    }

    pub fn get_inside(&self) -> Option<Arc<Medium>> {
        self.inside.as_ref().cloned()
    }

    pub fn get_outside(&self) -> Option<Arc<Medium>> {
        self.outside.as_ref().cloned()
    }
}

pub struct HenyeyGreenStein {
    g: f32,
}
