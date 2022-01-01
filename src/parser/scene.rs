use super::*;

#[derive(Debug)]
pub struct Scene {
    // General scene info
    pub cameras: Vec<Camera>,
    pub image_size: Vector2<u32>,
    // Materials
    pub number_unamed_materials: usize,
    pub materials: HashMap<String, BSDF>,
    pub textures: HashMap<String, Texture>,
    // 3D objects
    pub shapes: Vec<ShapeInfo>,               //< unamed shapes
    pub objects: HashMap<String, ObjectInfo>, //< shapes with objects
    pub instances: Vec<InstanceInfo>,         //< instances on the shapes
    pub lights: Vec<Light>,                   //< list of all light sources
    pub transforms: HashMap<String, Matrix>,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            cameras: Vec::default(),
            image_size: Vector2::new(512, 512),
            // materials
            number_unamed_materials: 0,
            materials: HashMap::default(),
            textures: HashMap::default(),
            // 3d object information
            shapes: Vec::default(),
            objects: HashMap::default(),
            instances: Vec::default(),
            lights: Vec::default(),
            transforms: HashMap::default(),
        }
    }
}

/// State of the parser
#[derive(Debug)]
pub struct State {
    pub named_material: Vec<Option<String>>,
    pub matrix: Vec<Matrix>,
    pub emission: Vec<Option<Spectrum>>,
    pub object: Option<ObjectInfo>,
    pub reverse_orientation: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            named_material: vec![None],
            matrix: vec![Matrix::IDENTITY],
            emission: vec![None],
            object: None,
            reverse_orientation: false,
        }
    }
}

impl State {
    // State
    pub fn save(&mut self) {
        let new_material = self.named_material.last().unwrap().clone();
        self.named_material.push(new_material);
        let new_matrix = *self.matrix.last().unwrap();
        self.matrix.push(new_matrix);
        let new_emission = self.emission.last().unwrap().clone();
        self.emission.push(new_emission);
    }
    pub fn restore(&mut self) {
        self.named_material.pop();
        self.matrix.pop();
        self.emission.pop();
    }

    // Matrix
    pub fn matrix(&self) -> Matrix {
        *self.matrix.last().unwrap()
    }
    pub fn replace_matrix(&mut self, m: Matrix) {
        let curr_mat = self.matrix.last_mut().unwrap();
        curr_mat.clone_from(&m);
    }
    // Named material
    pub fn named_material(&self) -> Option<String> {
        self.named_material.last().unwrap().clone()
    }
    pub fn set_named_matrial(&mut self, s: String) {
        let last_id = self.named_material.len() - 1;
        self.named_material[last_id] = Some(s);
    }
    // Emission
    pub fn emission(&self) -> Option<Spectrum> {
        self.emission.last().unwrap().clone()
    }
    pub fn set_emission(&mut self, e: Spectrum) {
        let last_id = self.emission.len() - 1;
        self.emission[last_id] = Some(e);
    }
    // Object
    pub fn new_object(&mut self, name: String) {
        self.object = Some(ObjectInfo {
            name,
            shapes: Vec::new(),
            matrix: self.matrix(),
        });
    }
    pub fn finish_object(&mut self) -> ObjectInfo {
        std::mem::replace(&mut self.object, None).unwrap()
    }
}
