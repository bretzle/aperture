//! Defines the scene struct which contains the various objects defining the scene.
//! This includes the geometry, instances of the geometry, the camera and so on.
//!
//! # Scene JSON Files
//! The scene file format has four required sections: a camera, an integrator,
//! a list of materials and a list of objects and lights. The root object in the
//! JSON file should contain one of each of these.
//!
//! ```json
//! {
//!     "camera": {...},
//!     "integrator": {...},
//!     "materials": [...],
//!     "objects": [...]
//! }
//! ```
//!
//! For more information on each object see the corresponding modules:
//!
//! - Camera: See film/camera
//! - Integrator: See integrator
//! - Materials: See materials
//! - Objects: See geometry
//!

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use image;
use serde_json::{self, Value};

use crate::film::{filter, AnimatedColor, Camera, ColorKeyframe, Colorf, FrameInfo, RenderTarget};
use crate::geometry::{
    BoundableGeom, Disk, Instance, Intersection, Mesh, Rectangle, SampleableGeom, Sphere, BVH,
};
use crate::integrator::{self, Integrator};
use crate::linalg::{AnimatedTransform, Keyframe, Point, Ray, Transform, Vector};
use crate::material::{Glass, Material, Matte, Merl, Metal, Plastic, RoughGlass, SpecularMetal};
use crate::texture::{self, Texture};

/// This lets me enforce only certain types of textures are valid,
/// and to look up the right type of texture result for a given
/// input. But it's a bit of a pain to deal with, if I want to add
/// more texture-able types and such.
struct LoadedTextures {
    textures: HashMap<String, Arc<dyn Texture + Send + Sync>>,
}
impl LoadedTextures {
    pub fn none() -> LoadedTextures {
        LoadedTextures {
            textures: HashMap::new(),
        }
    }
    /// Get a Color texture, if it's in the map by loading from the element.
    /// If the element is a string the teture name will be looked up, if
    /// not a constant texture will be created and returned
    pub fn find_color(&self, e: &Value) -> Option<Arc<dyn Texture + Send + Sync>> {
        match *e {
            Value::String(ref s) => match self.textures.get(s) {
                Some(t) => Some(t.clone()),
                None => None,
            },
            Value::Array(_) => match load_color(e) {
                Some(c) => Some(Arc::new(texture::ConstantColor::new(c))),
                None => None,
            },
            _ => panic!("Invalid JSON type for colorf texture"),
        }
    }
    /// Get a scalar texture, if it's in the map by loading from the element.
    /// If the element is a string the teture name will be looked up, if
    /// not a constant texture will be created and returned
    pub fn find_scalar(&self, e: &Value) -> Option<Arc<dyn Texture + Send + Sync>> {
        match *e {
            Value::String(ref s) => match self.textures.get(s) {
                Some(t) => Some(t.clone()),
                None => None,
            },
            Value::Number(ref n) => Some(Arc::new(texture::ConstantScalar::new(
                n.as_f64().unwrap() as f32,
            ))),
            _ => panic!("Invalid JSON type for scalar texture"),
        }
    }
}

/// The scene containing the objects and camera configuration we'd like to render,
/// shared immutably among the ray tracing threads
pub struct Scene {
    pub cameras: Vec<Camera>,
    active_camera: Option<usize>,
    pub bvh: BVH<Instance>,
    pub integrator: Box<dyn Integrator + Send + Sync>,
}

impl Scene {
    pub fn load_file(file: &str) -> (Scene, RenderTarget, usize, FrameInfo) {
        let mut f = match File::open(file) {
            Ok(f) => f,
            Err(e) => panic!("Failed to open scene file: {}", e),
        };
        let mut content = String::new();
        if let Err(e) = f.read_to_string(&mut content) {
            panic!("Failed to read scene file: {}", e);
        }
        // Why not use expect here?
        let data: Value = match serde_json::from_str(&content[..]) {
            Ok(d) => d,
            Err(e) => panic!("JSON parsing error: {}", e),
        };
        assert!(
            data.is_object(),
            "Expected a root JSON object. See example scenes"
        );
        let path = match Path::new(file).parent() {
            Some(p) => p,
            None => Path::new(file),
        };

        let (rt, spp, frame_info) = load_film(
            data.get("film")
                .expect("The scene must specify a film to write to"),
        );
        let cameras = load_cameras(&data, rt.dimensions());
        let integrator = load_integrator(
            data.get("integrator")
                .expect("The scene must specify the integrator to render with"),
        );
        let textures = match data.get("textures") {
            Some(e) => load_textures(path, e),
            None => LoadedTextures::none(),
        };
        let materials = load_materials(
            path,
            data.get("materials")
                .expect("An array of materials is required"),
            &textures,
        );
        // mesh cache is a map of file_name -> (map of mesh name -> mesh)
        let mut mesh_cache = HashMap::new();
        let instances = load_objects(
            path,
            &materials,
            &mut mesh_cache,
            data.get("objects")
                .expect("The scene must specify a list of objects"),
        );

        assert!(
            !instances.is_empty(),
            "Aborting: the scene does not have any objects!"
        );
        let scene = Scene {
            cameras,
            active_camera: None,
            // TODO: Read time parameters from the scene file, update BVH every few frames
            bvh: BVH::new(4, instances, 0.0, frame_info.time),
            integrator,
        };
        (scene, rt, spp, frame_info)
    }
    /// Test the ray for intersections against the objects in the scene.
    /// Returns Some(Intersection) if an intersection was found and None if not.
    pub fn intersect(&self, ray: &mut Ray) -> Option<Intersection> {
        self.bvh.intersect(ray, |r, i| i.intersect(r))
    }
    /// Advance the time the scene is currently displaying to the time range passed
    pub fn update_frame(&mut self, frame: usize, start: f32, end: f32) {
        let cam = match self.active_camera {
            Some(c) => {
                if c != self.cameras.len() - 1 && self.cameras[c + 1].active_at == frame {
                    println!("Changing to camera {}", c + 1);
                    c + 1
                } else {
                    c
                }
            }
            None => {
                // If there's no active camera we need to find the right one to start with
                // based on what frame we're beginning the rendering at. e.g. if you have a
                // camera become active at frame 5 and pass --start-frame 5, you should render
                // from that camera.
                let c = self
                    .cameras
                    .iter()
                    .take_while(|x| x.active_at <= frame)
                    .count()
                    - 1;
                println!("Selecting starting camera {}", c);
                c
            }
        };
        self.active_camera = Some(cam);
        self.cameras[cam].update_frame(start, end);
        // TODO: How often to re-build the BVH?
        let shutter_time = self.cameras[cam].shutter_time();
        println!(
            "Frame {}: re-building bvh for {} to {}",
            frame, shutter_time.0, shutter_time.1
        );
        self.bvh.rebuild(shutter_time.0, shutter_time.1);
    }
    /// Get the active camera for the current frame
    pub fn active_camera(&self) -> &Camera {
        &self.cameras[self
            .active_camera
            .expect("Update frame must be called before active_camera")]
    }
}

/// Load the film described by the JSON value passed. Returns the render target
/// along with the image dimensions and samples per pixel
fn load_film(elem: &Value) -> (RenderTarget, usize, FrameInfo) {
    let width = elem
        .get("width")
        .expect("The film must specify the image width")
        .as_u64()
        .expect("Image width must be a number") as usize;
    let height = elem
        .get("height")
        .expect("The film must specify the image height")
        .as_u64()
        .expect("Image height must be a number") as usize;
    let spp = elem
        .get("samples")
        .expect("The film must specify the number of samples per pixel")
        .as_u64()
        .expect("Samples per pixel must be a number") as usize;
    let start_frame = elem
        .get("start_frame")
        .expect("The film must specify the starting frame")
        .as_u64()
        .expect("Start frame must be a number") as usize;
    let end_frame = elem
        .get("end_frame")
        .expect("The film must specify the frame to end on")
        .as_u64()
        .expect("End frame must be a number") as usize;
    if end_frame < start_frame {
        panic!("End frame must be greater or equal to the starting frame");
    }
    let frames = elem
        .get("frames")
        .expect("The film must specify the total number of frames")
        .as_u64()
        .expect("Frames must be a number") as usize;
    let scene_time = elem
        .get("scene_time")
        .expect("The film must specify the overall scene time")
        .as_f64()
        .expect("Scene time must be a number") as f32;
    let frame_info = FrameInfo::new(frames, scene_time, start_frame, end_frame);
    let filter = load_filter(
        elem.get("filter")
            .expect("The film must specify a reconstruction filter"),
    );
    (
        RenderTarget::new((width, height), (2, 2), filter),
        spp,
        frame_info,
    )
}
/// Load the reconstruction filter described by the JSON value passed
fn load_filter(elem: &Value) -> Box<dyn filter::Filter + Send + Sync> {
    let width = elem
        .get("width")
        .expect("The filter must specify the filter width")
        .as_f64()
        .expect("Filter width must be a number") as f32;
    let height = elem
        .get("height")
        .expect("The filter must specify the filter height")
        .as_f64()
        .expect("Filter height must be a number") as f32;
    let ty = elem
        .get("type")
        .expect("A type is required for the filter")
        .as_str()
        .expect("Filter type must be a string");
    if ty == "mitchell_netravali" {
        let b = elem
            .get("b")
            .expect("A b parameter is required for the Mitchell-Netravali filter")
            .as_f64()
            .expect("b must be a number") as f32;
        let c = elem
            .get("c")
            .expect("A c parameter is required for the Mitchell-Netravali filter")
            .as_f64()
            .expect("c must be a number") as f32;
        Box::new(filter::MitchellNetravali::new(width, height, b, c))
            as Box<dyn filter::Filter + Send + Sync>
    } else if ty == "gaussian" {
        let alpha = elem
            .get("alpha")
            .expect("An alpha parameter is required for the Gaussian filter")
            .as_f64()
            .expect("alpha must be a number") as f32;
        Box::new(filter::Gaussian::new(width, height, alpha))
            as Box<dyn filter::Filter + Send + Sync>
    } else {
        panic!("Unrecognized filter type {}!", ty);
    }
}

/// Load the cameras or single camera specified for this scene
fn load_cameras(elem: &Value, dim: (usize, usize)) -> Vec<Camera> {
    match elem.get("cameras") {
        Some(c) => {
            let cameras_json = match c.as_array() {
                Some(ca) => ca,
                None => panic!("cameras listing must be an array of cameras"),
            };
            let mut cameras = Vec::new();
            for cam in cameras_json {
                cameras.push(load_camera(cam, dim));
            }
            cameras.sort_by(|a, b| a.active_at.cmp(&b.active_at));
            cameras
        }
        None => vec![load_camera(
            elem.get("camera").expect("Error: A camera is required!"),
            dim,
        )],
    }
}
/// Load the camera described by the JSON value passed.
/// Returns the camera along with the number of samples to take per pixel
/// and the scene dimensions. Panics if the camera is incorrectly specified
fn load_camera(elem: &Value, dim: (usize, usize)) -> Camera {
    let shutter_size = match elem.get("shutter_size") {
        Some(s) => s
            .as_f64()
            .expect("Shutter size should be a float from 0 to 1") as f32,
        None => 0.5,
    };
    let active_at = match elem.get("active_at") {
        Some(s) => s
            .as_u64()
            .expect("The camera activation frame 'active_at' must be an unsigned int")
            as usize,
        None => 0,
    };
    let transform = match elem.get("keyframes") {
        Some(t) => load_keyframes(t).expect("Invalid keyframes specified"),
        None => {
            let t = match elem.get("transform") {
                Some(t) => load_transform(t).expect("Invalid transform specified"),
                None => {
                    println!("Warning! Specifying transforms with pos, target and up vectors is deprecated!");
                    let pos = load_point(
                        elem.get("position")
                            .expect("The camera must specify a position"),
                    )
                    .expect("position must be an array of 3 floats");
                    let target = load_point(
                        elem.get("target")
                            .expect("The camera must specify a target"),
                    )
                    .expect("target must be an array of 3 floats");
                    let up = load_vector(
                        elem.get("up")
                            .expect("The camera must specify an up vector"),
                    )
                    .expect("up must be an array of 3 floats");
                    Transform::look_at(&pos, &target, &up)
                }
            };
            AnimatedTransform::unanimated(&t)
        }
    };
    let fov_elem = elem
        .get("fov")
        .expect("The camera must specify a field of view");
    if fov_elem.is_array() {
        let fovs_elems = fov_elem.as_array().expect("List of FOVs must be an array");
        let fov_knot_elems = elem
            .get("fov_knots")
            .expect("Animated field of view must specify spline knots")
            .as_array()
            .expect("Fov spline knots must be an array");
        let fov_spline_degree =
            elem.get("fov_spline_degree")
                .expect("Animated fov spline must have degree")
                .as_u64()
                .expect("Animated fov spline degree must be a u64") as usize;
        let fovs = fovs_elems
            .iter()
            .map(|x| x.as_f64().expect("fovs must be a number") as f32)
            .collect();
        let fov_knots = fov_knot_elems
            .iter()
            .map(|x| x.as_f64().expect("fov knots must be a number") as f32)
            .collect();
        Camera::animated_fov(
            transform,
            fovs,
            fov_knots,
            fov_spline_degree,
            dim,
            shutter_size,
            active_at,
        )
    } else {
        let fov = fov_elem.as_f64().expect("Camera fov must be a number") as f32;
        Camera::new(transform, fov, dim, shutter_size, active_at)
    }
}

/// Load the integrator described by the JSON value passed.
/// Return the integrator or panics if it's incorrectly specified
fn load_integrator(elem: &Value) -> Box<dyn Integrator + Send + Sync> {
    let ty = elem
        .get("type")
        .expect("Integrator must specify a type")
        .as_str()
        .expect("Integrator type must be a string");
    if ty == "pathtracer" {
        let min_depth = elem
            .get("min_depth")
            .expect("The integrator must specify the minimum ray depth")
            .as_u64()
            .expect("min_depth must be a number") as u32;
        let max_depth = elem
            .get("max_depth")
            .expect("The integrator must specify the maximum ray depth")
            .as_u64()
            .expect("max_depth must be a number") as u32;
        Box::new(integrator::Path::new(min_depth, max_depth))
    } else if ty == "whitted" {
        let min_depth = elem
            .get("min_depth")
            .expect("The integrator must specify the minimum ray depth")
            .as_u64()
            .expect("min_depth must be a number") as u32;
        Box::new(integrator::Whitted::new(min_depth))
    } else if ty == "normals_debug" {
        Box::new(integrator::NormalsDebug)
    } else {
        panic!("Unrecognized integrator type '{}'", ty);
    }
}

fn load_textures(path: &Path, elem: &Value) -> LoadedTextures {
    let mut textures = LoadedTextures::none();
    let tex_vec = elem
        .as_array()
        .expect("The 'textures' must be an array of textures to load");
    for (i, t) in tex_vec.iter().enumerate() {
        let name = t
            .get("name")
            .expect(&format!("Error loading texture #{}: A name is required", i)[..])
            .as_str()
            .expect(&format!("Error loading texture #{}: name must be a string", i)[..])
            .to_owned();
        let ty = t
            .get("type")
            .expect(&mat_error(&name, "A texture type is required")[..])
            .as_str()
            .expect(&mat_error(&name, "Texture type must be a string")[..]);
        // Make sure names are unique to avoid people accidently overwriting textures
        if textures.textures.contains_key(&name) {
            panic!(
                "Error loading texture '{}': name conflicts with an existing entry",
                name
            );
        }
        if ty == "image" {
            let mut file_path = PathBuf::new();
            file_path.push(
                t.get("file")
                    .expect("Image textures must specify an image file")
                    .as_str()
                    .expect("Image file name must be a string"),
            );

            if file_path.is_relative() {
                file_path = path.join(file_path);
            }
            let img = image::open(file_path).expect("Failed to load image file");

            textures
                .textures
                .insert(name, Arc::new(texture::Image::new(img)));
        } else if ty == "animated_image" {
            let frames_list = t
                .get("keyframes")
                .expect("animated_image requires keyframes")
                .as_array()
                .expect("animated_image keyframes must be an array");
            if frames_list.len() < 2 {
                panic!("animated_image must have at least 2 frames");
            }
            let frames: Vec<_> = frames_list
                .iter()
                .map(|f| {
                    let mut file_path = PathBuf::new();
                    file_path.push(
                        f.get("file")
                            .expect("Image textures must specify an image file")
                            .as_str()
                            .expect("Image file name must be a string"),
                    );

                    if file_path.is_relative() {
                        file_path = path.join(file_path);
                    }
                    let time = f
                        .get("time")
                        .expect("animated_image keyframe requires time")
                        .as_f64()
                        .expect("animated_image keyframe time must be a number")
                        as f32;
                    let img = texture::Image::new(
                        image::open(file_path).expect("Failed to load image file"),
                    );
                    (time, img)
                })
                .collect();

            textures
                .textures
                .insert(name, Arc::new(texture::AnimatedImage::new(frames)));
        } else if ty == "movie" {
            // A movie is a generated animated_image, based on a format string to find the
            // keyframes and a framerate to play back at

            let file_prefix = t
                .get("file_prefix")
                .expect("A file_prefix for movie is required")
                .as_str()
                .expect("file_prefix for movie must be a string");
            let file_suffix = t
                .get("file_suffix")
                .expect("A file_suffix for movie is required")
                .as_str()
                .expect("file_suffix for movie must be a string");
            let total_frames = t
                .get("frames")
                .expect("# of frames for movie texture is required")
                .as_u64()
                .expect("frames for movie texture must be an int");
            let framerate = t
                .get("framerate")
                .expect("A framerate for movie is required")
                .as_u64()
                .expect("framerate for movie must be an int");

            let frames: Vec<_> = (0..total_frames)
                .map(|frame| {
                    let mut file_path = PathBuf::new();
                    // There's no support for runtime-string formatting, maybe some lib out there for
                    // it but a lot of them seem targetted for web development and are too heavy.
                    file_path.push(format!("{}{:05}{}", file_prefix, frame, file_suffix));
                    if file_path.is_relative() {
                        file_path = path.join(file_path);
                    }
                    let time = frame as f32 / framerate as f32;
                    let img = texture::Image::new(
                        image::open(file_path).expect("Failed to load image file"),
                    );
                    (time, img)
                })
                .collect();

            textures
                .textures
                .insert(name, Arc::new(texture::AnimatedImage::new(frames)));
        } else {
            panic!("Unrecognized texture type '{}' for texture '{}'", ty, name);
        }
    }
    textures
}

/// Generate a material loading error string
fn mat_error(mat_name: &str, msg: &str) -> String {
    format!("Error loading material '{}': {}", mat_name, msg)
}

/// Load the array of materials used in the scene, panics if a material is specified
/// incorrectly. The path to the directory containing the scene file is required to find
/// referenced material data relative to the scene file.
fn load_materials(
    path: &Path,
    elem: &Value,
    textures: &LoadedTextures,
) -> HashMap<String, Arc<dyn Material + Send + Sync>> {
    let mut materials = HashMap::new();
    let mat_vec = elem
        .as_array()
        .expect("The materials must be an array of materials used");
    for (i, m) in mat_vec.iter().enumerate() {
        let name = m
            .get("name")
            .expect(&format!("Error loading material #{}: A name is required", i)[..])
            .as_str()
            .expect(&format!("Error loading material #{}: name must be a string", i)[..])
            .to_owned();
        let ty = m
            .get("type")
            .expect(&mat_error(&name, "a type is required")[..])
            .as_str()
            .expect(&mat_error(&name, "type must be a string")[..]);
        // Make sure names are unique to avoid people accidently overwriting materials
        if materials.contains_key(&name) {
            panic!(
                "Error loading material '{}': name conflicts with an existing entry",
                name
            );
        }
        if ty == "glass" {
            let reflect = textures
                .find_color(
                    m.get("reflect")
                        .expect("reflect color/texture name is required for glass"),
                )
                .expect(&mat_error(&name, "Invalid color specified for reflect of glass")[..]);
            let transmit = textures
                .find_color(
                    m.get("transmit")
                        .expect("transmit color/texture name is required for glass"),
                )
                .expect(&mat_error(&name, "Invalid color specified for transmit of glass")[..]);
            let eta = textures
                .find_scalar(
                    m.get("eta")
                        .expect("eta color/texture name is required for glass"),
                )
                .expect(&mat_error(&name, "Invalid color specified for eta of glass")[..]);

            materials.insert(
                name,
                Arc::new(Glass::new(reflect, transmit, eta)) as Arc<dyn Material + Send + Sync>,
            );
        } else if ty == "rough_glass" {
            let reflect = textures
                .find_color(
                    m.get("reflect")
                        .expect("reflect color/texture name is required for rough glass"),
                )
                .expect(
                    &mat_error(&name, "Invalid color specified for reflect of rough glass")[..],
                );
            let transmit = textures
                .find_color(
                    m.get("transmit")
                        .expect("transmit color/texture name is required for rough glass"),
                )
                .expect(
                    &mat_error(&name, "Invalid color specified for transmit of rough glass")[..],
                );
            let eta = textures
                .find_scalar(
                    m.get("eta")
                        .expect("eta color/texture name is required for rough glass"),
                )
                .expect(&mat_error(&name, "Invalid color specified for eta of rough glass")[..]);
            let roughness = textures
                .find_scalar(
                    m.get("roughness")
                        .expect("roughness color/texture name is required for rough glass"),
                )
                .expect(
                    &mat_error(
                        &name,
                        "Invalid color specified for roughness of rough glass",
                    )[..],
                );

            materials.insert(
                name,
                Arc::new(RoughGlass::new(reflect, transmit, eta, roughness))
                    as Arc<dyn Material + Send + Sync>,
            );
        } else if ty == "matte" {
            let diffuse = textures
                .find_color(
                    m.get("diffuse")
                        .expect("diffuse color/texture name is required for matte"),
                )
                .expect(&mat_error(&name, "Invalid color specified for diffuse of matte")[..]);

            let roughness = textures
                .find_scalar(
                    m.get("roughness")
                        .expect("roughness color/texture is required for matte"),
                )
                .expect(&mat_error(&name, "Invalid roughness specified for roughness")[..]);

            materials.insert(name, Arc::new(Matte::new(diffuse, roughness)));
        } else if ty == "merl" {
            let file_path = Path::new(
                m.get("file")
                    .expect(
                        &mat_error(
                            &name,
                            "A filename containing the MERL material data is required",
                        )[..],
                    )
                    .as_str()
                    .expect(&mat_error(&name, "The MERL file must be a string")[..]),
            );
            if file_path.is_relative() {
                materials.insert(
                    name,
                    Arc::new(Merl::load_file(path.join(file_path).as_path()))
                        as Arc<dyn Material + Send + Sync>,
                );
            } else {
                materials.insert(
                    name,
                    Arc::new(Merl::load_file(file_path)) as Arc<dyn Material + Send + Sync>,
                );
            }
        } else if ty == "metal" {
            let refr_index = textures
                .find_color(
                    m.get("refractive_index")
                        .expect("refractive_index color/texture name is required for metal"),
                )
                .expect(
                    &mat_error(
                        &name,
                        "Invalid color specified for refractive_index of metal",
                    )[..],
                );

            let absorption_coef = textures
                .find_color(
                    m.get("absorption_coefficient")
                        .expect("absorption_coefficient color/texture name is required for metal"),
                )
                .expect(
                    &mat_error(
                        &name,
                        "Invalid color specified for absorption_coefficient of metal",
                    )[..],
                );

            let roughness = textures
                .find_scalar(
                    m.get("roughness")
                        .expect("roughness color/texture is required for metal"),
                )
                .expect(&mat_error(&name, "Invalid roughness specified for metal")[..]);
            materials.insert(
                name,
                Arc::new(Metal::new(refr_index, absorption_coef, roughness))
                    as Arc<dyn Material + Send + Sync>,
            );
        } else if ty == "plastic" {
            let diffuse = textures
                .find_color(
                    m.get("diffuse")
                        .expect("diffuse color/texture name is required for plastic"),
                )
                .expect(&mat_error(&name, "Invalid color specified for diffuse of plastic")[..]);

            let gloss = textures
                .find_color(
                    m.get("gloss")
                        .expect("gloss color/texture name is required for plastic"),
                )
                .expect(&mat_error(&name, "Invalid color specified for diffuse of plastic")[..]);

            let roughness = textures
                .find_scalar(
                    m.get("roughness")
                        .expect("roughness color/texture is required for plastic"),
                )
                .expect(&mat_error(&name, "Invalid roughness specified for plastic")[..]);

            materials.insert(
                name,
                Arc::new(Plastic::new(diffuse, gloss, roughness))
                    as Arc<dyn Material + Send + Sync>,
            );
        } else if ty == "specular_metal" {
            let refr_index =
                textures
                    .find_color(m.get("refractive_index").expect(
                        "refractive_index color/texture name is required for specular metal",
                    ))
                    .expect(
                        &mat_error(
                            &name,
                            "Invalid color specified for refractive_index of specular metal",
                        )[..],
                    );

            let absorption_coef = textures
                .find_color(m.get("absorption_coefficient").expect(
                    "absorption_coefficient color/texture name is required for specular metal",
                ))
                .expect(
                    &mat_error(
                        &name,
                        "Invalid color specified for absorption_coefficient of specular metal",
                    )[..],
                );
            materials.insert(
                name,
                Arc::new(SpecularMetal::new(refr_index, absorption_coef))
                    as Arc<dyn Material + Send + Sync>,
            );
        } else {
            panic!(
                "Error parsing material '{}': unrecognized type '{}'",
                name, ty
            );
        }
    }
    materials
}

/// Loads the array of objects in the scene, assigning them materials from the materials map. Will
/// panic if an incorrectly specified object is found.
fn load_objects(
    path: &Path,
    materials: &HashMap<String, Arc<dyn Material + Send + Sync>>,
    mesh_cache: &mut HashMap<String, HashMap<String, Arc<Mesh>>>,
    elem: &Value,
) -> Vec<Instance> {
    let mut instances = Vec::new();
    let objects = elem
        .as_array()
        .expect("The objects must be an array of objects used");
    for o in objects {
        let name = o
            .get("name")
            .expect("A name is required for an object")
            .as_str()
            .expect("Object name must be a string")
            .to_owned();
        let ty = o
            .get("type")
            .expect("A type is required for an object")
            .as_str()
            .expect("Object type must be a string");

        let transform = match o.get("keyframes") {
            Some(t) => load_keyframes(t).expect("Invalid keyframes specified"),
            None => {
                let t = match o.get("transform") {
                    Some(t) => load_transform(t).expect("Invalid transform specified"),
                    None => panic!("No keyframes or transform specified for object {}", name),
                };
                AnimatedTransform::unanimated(&t)
            }
        };
        if ty == "emitter" {
            let emit_ty = o
                .get("emitter")
                .expect("An emitter type is required for emitters")
                .as_str()
                .expect("Emitter type must be a string");
            let emission = load_animated_color(
                o.get("emission")
                    .expect("An emission color is required for emitters"),
            )
            .expect("Emitter emission must be a color");
            if emit_ty == "point" {
                instances.push(Instance::point_light(transform, emission, name));
            } else if emit_ty == "area" {
                let mat_name = o
                    .get("material")
                    .expect("A material is required for an object")
                    .as_str()
                    .expect("Object material name must be a string");
                let mat = materials
                    .get(mat_name)
                    .unwrap_or_else(|| {
                        panic!("Material {} was not found in the material list", mat_name)
                    })
                    .clone();
                let geom = load_sampleable_geometry(
                    o.get("geometry")
                        .expect("Geometry is required for area lights"),
                );

                instances.push(Instance::area_light(geom, mat, emission, transform, name));
            } else {
                panic!("Invalid emitter type specified: {}", emit_ty);
            }
        } else if ty == "receiver" {
            let mat_name = o
                .get("material")
                .expect("A material is required for an object")
                .as_str()
                .expect("Object material name must be a string");
            let mat = materials
                .get(mat_name)
                .unwrap_or_else(|| {
                    panic!("Material {} was not found in the material list", mat_name)
                })
                .clone();
            let geom = load_geometry(
                path,
                mesh_cache,
                o.get("geometry")
                    .expect("Geometry is required for receivers"),
            );

            instances.push(Instance::receiver(geom, mat, transform, name));
        } else if ty == "group" {
            let group_objects = o
                .get("objects")
                .expect("A group must specify an array of objects in the group");
            let group_instances = load_objects(path, materials, mesh_cache, group_objects);
            for mut gi in group_instances {
                {
                    let t = gi.get_transform().clone();
                    gi.set_transform(transform.clone() * t);
                }
                instances.push(gi);
            }
        } else {
            panic!(
                "Error parsing object '{}': unrecognized type '{}'",
                name, ty
            );
        }
    }
    instances
}

/// Load the geometry specified by the JSON value. Will re-use any already loaded meshes
/// and will place newly loaded meshees in the mesh cache.
fn load_geometry(
    path: &Path,
    meshes: &mut HashMap<String, HashMap<String, Arc<Mesh>>>,
    elem: &Value,
) -> Arc<dyn BoundableGeom + Send + Sync> {
    let ty = elem
        .get("type")
        .expect("A type is required for geometry")
        .as_str()
        .expect("Geometry type must be a string");
    if ty == "sphere" {
        let r = elem
            .get("radius")
            .expect("A radius is required for a sphere")
            .as_f64()
            .expect("radius must be a number") as f32;
        Arc::new(Sphere::new(r))
    } else if ty == "disk" {
        let r = elem
            .get("radius")
            .expect("A radius is required for a disk")
            .as_f64()
            .expect("radius must be a number") as f32;
        let ir = elem
            .get("inner_radius")
            .expect("An inner radius is required for a disk")
            .as_f64()
            .expect("inner radius must be a number") as f32;
        Arc::new(Disk::new(r, ir))
    } else if ty == "plane" {
        // We just treat plane as a special case of Rectangle now
        Arc::new(Rectangle::new(2.0, 2.0))
    } else if ty == "rectangle" {
        let width = elem
            .get("width")
            .expect("A width is required for a rectangle")
            .as_f64()
            .expect("width must be a number") as f32;
        let height = elem
            .get("height")
            .expect("A height is required for a rectangle")
            .as_f64()
            .expect("height must be a number") as f32;
        Arc::new(Rectangle::new(width, height))
    } else if ty == "mesh" {
        let mut file = Path::new(
            elem.get("file")
                .expect("An OBJ file is required for meshes")
                .as_str()
                .expect("OBJ filename must be a string"),
        )
        .to_path_buf();
        let model = elem
            .get("model")
            .expect("A model name is required for geometry")
            .as_str()
            .expect("Model name type must be a string");

        if file.is_relative() {
            file = path.join(file);
        }
        let file_string = file.to_str().expect("Invalid file name");
        if meshes.get(file_string).is_none() {
            meshes.insert(file_string.to_owned(), Mesh::load_obj(Path::new(&file)));
        }
        let file_meshes = &meshes[file_string];
        match file_meshes.get(model) {
            Some(m) => m.clone(),
            None => panic!("Requested model '{}' was not found in '{:?}'", model, file),
        }
    } else {
        panic!("Unrecognized geometry type '{}'", ty);
    }
}

/// Load the sampleable geometry specified by the JSON value. Will panic if the geometry specified
/// is not sampleable.
fn load_sampleable_geometry(elem: &Value) -> Arc<dyn SampleableGeom + Send + Sync> {
    let ty = elem
        .get("type")
        .expect("A type is required for geometry")
        .as_str()
        .expect("Geometry type must be a string");
    if ty == "sphere" {
        let r = elem
            .get("radius")
            .expect("A radius is required for a sphere")
            .as_f64()
            .expect("radius must be a number") as f32;
        Arc::new(Sphere::new(r))
    } else if ty == "disk" {
        let r = elem
            .get("radius")
            .expect("A radius is required for a disk")
            .as_f64()
            .expect("radius must be a number") as f32;
        let ir = elem
            .get("inner_radius")
            .expect("An inner radius is required for a disk")
            .as_f64()
            .expect("inner radius must be a number") as f32;
        Arc::new(Disk::new(r, ir))
    } else if ty == "rectangle" {
        let width = elem
            .get("width")
            .expect("A width is required for a rectangle")
            .as_f64()
            .expect("width must be a number") as f32;
        let height = elem
            .get("height")
            .expect("A height is required for a rectangle")
            .as_f64()
            .expect("height must be a number") as f32;
        Arc::new(Rectangle::new(width, height))
    } else {
        panic!(
            "Geometry of type '{}' is not sampleable and can't be used for area light geometry",
            ty
        );
    }
}

/// Load a vector from the JSON element passed. Returns None if the element
/// did not contain a valid vector (eg. [1.0, 2.0, 0.5])
fn load_vector(elem: &Value) -> Option<Vector> {
    let array = match elem.as_array() {
        Some(a) => a,
        None => return None,
    };
    if array.len() != 3 {
        return None;
    }
    let mut v = [0.0f32; 3];
    for (i, x) in array.iter().enumerate() {
        match x.as_f64() {
            Some(f) => v[i] = f as f32,
            None => return None,
        }
    }
    Some(Vector::new(v[0], v[1], v[2]))
}

/// Load a point from the JSON element passed. Returns None if the element
/// did not contain a valid point (eg. [1.0, 2.0, 0.5])
fn load_point(elem: &Value) -> Option<Point> {
    let array = match elem.as_array() {
        Some(a) => a,
        None => return None,
    };
    if array.len() != 3 {
        return None;
    }
    let mut v = [0.0f32; 3];
    for (i, x) in array.iter().enumerate() {
        match x.as_f64() {
            Some(f) => v[i] = f as f32,
            None => return None,
        }
    }
    Some(Point::new(v[0], v[1], v[2]))
}

/// Load a color from the JSON element passed. Returns None if the element
/// did not contain a valid color.
fn load_color(elem: &Value) -> Option<Colorf> {
    let array = match elem.as_array() {
        Some(a) => a,
        None => return None,
    };
    if array.len() != 3 && array.len() != 4 {
        return None;
    }
    let mut v = Vec::with_capacity(4);
    for x in array.iter() {
        match x.as_f64() {
            Some(f) => v.push(f as f32),
            None => return None,
        }
    }
    let mut c = Colorf::new(v[0], v[1], v[2]);
    if v.len() == 4 {
        c = c * v[3];
    }
    Some(c)
}

/// Load an animated color from the JSON element passed. Returns None if the
/// element did not contain a valid color
fn load_animated_color(elem: &Value) -> Option<AnimatedColor> {
    let array = match elem.as_array() {
        Some(a) => a,
        None => return None,
    };
    if array.is_empty() {
        return None;
    }
    // Check if this is actually just a single color value
    if array[0].is_number() {
        load_color(elem).map(|c| AnimatedColor::with_keyframes(vec![ColorKeyframe::new(&c, 0.0)]))
    } else {
        let mut v = Vec::new();
        for c in array.iter() {
            let time = c
                .get("time")
                .expect("A time must be specified for a color keyframe")
                .as_f64()
                .expect("Time for color keyframe must be a number") as f32;
            let color = load_color(
                c.get("color")
                    .expect("A color must be specified for a color keyframe"),
            )
            .expect("A valid color is required for a color keyframe");
            v.push(ColorKeyframe::new(&color, time));
        }
        Some(AnimatedColor::with_keyframes(v))
    }
}

/// Load a transform stack specified by the element. Will panic on invalidly specified
/// transforms and log the error.
fn load_transform(elem: &Value) -> Option<Transform> {
    let array = match elem.as_array() {
        Some(a) => a,
        None => return None,
    };
    let mut transform = Transform::identity();
    for t in array {
        let ty = t
            .get("type")
            .expect("A type is required for a transform")
            .as_str()
            .expect("Transform type must be a string");
        if ty == "translate" {
            let v = load_vector(
                t.get("translation")
                    .expect("A translation vector is required for translate"),
            )
            .expect("Invalid vector specified for translation direction");

            transform = Transform::translate(&v) * transform;
        } else if ty == "scale" {
            let s = t
                .get("scaling")
                .expect("A scaling value or vector is required for scale");
            let v;
            if s.is_array() {
                v = load_vector(s).expect("Invalid vector specified for scaling vector");
            } else if s.is_number() {
                v = Vector::broadcast(
                    s.as_f64().expect("Invalid float specified for scale value") as f32
                );
            } else {
                panic!("Scaling value should be an array of 3 floats or a single float");
            }

            transform = Transform::scale(&v) * transform;
        } else if ty == "rotate_x" {
            let r = t
                .get("rotation")
                .expect("A rotation in degrees is required for rotate_x")
                .as_f64()
                .expect("rotation for rotate_x must be a number") as f32;

            transform = Transform::rotate_x(r) * transform;
        } else if ty == "rotate_y" {
            let r = t
                .get("rotation")
                .expect("A rotation in degrees is required for rotate_y")
                .as_f64()
                .expect("rotation for rotate_y must be a number") as f32;

            transform = Transform::rotate_y(r) * transform;
        } else if ty == "rotate_z" {
            let r = t
                .get("rotation")
                .expect("A rotation in degrees is required for rotate_z")
                .as_f64()
                .expect("rotation for rotate_z must be a number") as f32;

            transform = Transform::rotate_z(r) * transform;
        } else if ty == "rotate" {
            let r = t
                .get("rotation")
                .expect("A rotation in degrees is required for rotate")
                .as_f64()
                .expect("rotation for rotate must be a number") as f32;
            let axis = load_vector(
                t.get("axis")
                    .expect("An axis vector is required for rotate"),
            )
            .expect("Invalid vector specified for rotation axis");

            transform = Transform::rotate(&axis, r) * transform;
        } else if ty == "matrix" {
            // User has specified a pre-computed matrix for the transform
            let mat = t
                .get("matrix")
                .expect("The rows of the matrix are required for matrix transform")
                .as_array()
                .expect("The rows should be an array");
            let mut rows = Vec::with_capacity(16);
            for r in mat {
                let row = r.as_array().expect(
                    "Each row of the matrix transform must be an array, specifying the row",
                );
                if row.len() != 4 {
                    panic!("Each row of the transformation matrix must contain 4 elements");
                }
                for e in row {
                    rows.push(
                        e.as_f64()
                            .expect("Each element of a matrix row must be a float")
                            as f32,
                    );
                }
            }

            transform = Transform::from_mat(&rows.iter().collect()) * transform;
        } else {
            println!("Unrecognized transform type '{}'", ty);
            return None;
        }
    }
    Some(transform)
}

/// Load a list of keyframes specified by the element. Will panic on invalidly
/// specified keyframes or transforms and log the error
fn load_keyframes(elem: &Value) -> Option<AnimatedTransform> {
    let points = match elem
        .get("control_points")
        .expect("Control points are required for bspline keyframes")
        .as_array()
    {
        Some(a) => a,
        None => return None,
    };
    let knots_json = match elem
        .get("knots")
        .expect("knots are required for bspline keyframes")
        .as_array()
    {
        Some(a) => a,
        None => return None,
    };
    let mut keyframes = Vec::new();
    for t in points {
        let transform = load_transform(
            t.get("transform")
                .expect("A transform is required for a keyframe"),
        )
        .expect("Invalid transform for keyframe");
        keyframes.push(Keyframe::new(&transform));
    }
    let mut knots = Vec::new();
    for k in knots_json {
        knots.push(k.as_f64().expect("Knots must be numbers") as f32);
    }
    let degree = match elem.get("degree") {
        Some(d) => d.as_u64().expect("Curve degree must be a positive integer") as usize,
        None => 3,
    };
    Some(AnimatedTransform::with_keyframes(keyframes, knots, degree))
}
