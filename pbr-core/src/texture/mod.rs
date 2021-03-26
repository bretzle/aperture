use crate::{
    bounds::Bounds2i,
    interaction::SurfaceInteraction,
    mipmap::{MIPMap, WrapMode},
    paramset::ParamSet,
    spectrum::{Colors, Spectrum},
    transform::Transform,
    utils,
};
use log::{debug, error, info, warn};
use maths::*;
use num::Zero;
use std::{
    collections::HashMap,
    f32::consts::FRAC_1_PI,
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul},
    path::Path,
    sync::Arc,
};

pub type TextureSpectrum = dyn Texture<Spectrum>;
pub type TextureFloat = dyn Texture<f32>;

pub trait Texture<T>: Debug + Send + Sync {
    fn evaluate(&self, si: &SurfaceInteraction) -> T;
}

#[derive(Debug)]
pub struct ConstantTexture<T> {
    value: T,
}

impl<T: Copy> ConstantTexture<T> {
    pub fn new(value: T) -> ConstantTexture<T> {
        ConstantTexture { value }
    }
}

impl ConstantTexture<f32> {
    pub fn create_float(_tex2world: &Transform, tp: &TextureParams<'_>) -> ConstantTexture<f32> {
        ConstantTexture::new(tp.find_float("value", 1.0))
    }
}

impl ConstantTexture<Spectrum> {
    pub fn create_spectrum(
        _tex2world: &Transform,
        tp: &TextureParams<'_>,
    ) -> ConstantTexture<Spectrum> {
        ConstantTexture::new(tp.find_spectrum("value", Colors::WHITE))
    }
}

impl<T: Copy + Debug + Send + Sync> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _si: &SurfaceInteraction<'_, '_>) -> T {
        self.value
    }
}

pub struct TextureParams<'a> {
    geom_params: &'a ParamSet,
    material_params: &'a ParamSet,
    float_textures: &'a HashMap<String, Arc<dyn Texture<f32>>>,
    spectrum_textures: &'a HashMap<String, Arc<dyn Texture<Spectrum>>>,
}

impl<'a> TextureParams<'a> {
    pub fn new(
        gp: &'a ParamSet,
        mp: &'a ParamSet,
        ft: &'a HashMap<String, Arc<dyn Texture<f32>>>,
        st: &'a HashMap<String, Arc<dyn Texture<Spectrum>>>,
    ) -> TextureParams<'a> {
        TextureParams {
            geom_params: gp,
            material_params: mp,
            float_textures: ft,
            spectrum_textures: st,
        }
    }

    pub fn find_int(&self, n: &str, d: i32) -> i32 {
        let d = self.material_params.find_one_int(n, d);
        self.geom_params.find_one_int(n, d)
    }

    pub fn find_string(&self, n: &str, d: &str) -> String {
        let mat_string = self.material_params.find_one_string(n, d.to_owned());
        self.geom_params.find_one_string(n, mat_string)
    }

    pub fn find_filename(&self, n: &str, d: &str) -> String {
        let mat_string = self.material_params.find_one_filename(n, d.to_owned());
        self.geom_params.find_one_filename(n, mat_string)
    }

    pub fn find_bool(&self, n: &str, d: bool) -> bool {
        let d = self.material_params.find_one_bool(n, d);
        self.geom_params.find_one_bool(n, d)
    }

    pub fn find_float(&self, n: &str, d: f32) -> f32 {
        let d = self.material_params.find_one_float(n, d);
        self.geom_params.find_one_float(n, d)
    }

    pub fn find_vector3f(&self, n: &str, d: Vector3f) -> Vector3f {
        let d = self.material_params.find_one_vector3f(n, d);
        self.geom_params.find_one_vector3f(n, d)
    }

    pub fn find_spectrum(&self, n: &str, d: Spectrum) -> Spectrum {
        let d = self.material_params.find_one_spectrum(n, d);
        self.geom_params.find_one_spectrum(n, d)
    }

    pub fn get_spectrum_texture(&self, n: &str, default: &Spectrum) -> Arc<dyn Texture<Spectrum>> {
        let mut name = self.geom_params.find_texture(n, "".to_owned());
        if &name == "" {
            name = self.material_params.find_texture(n, "".to_owned());
        }
        if &name != "" {
            if let Some(tex) = self.spectrum_textures.get(&name) {
                return Arc::clone(tex);
            } else {
                error!(
                    "Couldn't find spectrum texture {} for parameter {}",
                    name, n
                );
            }
        }
        // If texture wasn't found
        let val = self.material_params.find_one_spectrum(n, *default);
        let val = self.geom_params.find_one_spectrum(n, val);
        Arc::new(ConstantTexture::new(val))
    }

    pub fn get_float_texture(&self, n: &str, default: f32) -> Arc<dyn Texture<f32>> {
        let mut name = self.geom_params.find_texture(n, "".to_owned());
        if &name == "" {
            name = self.material_params.find_texture(n, "".to_owned());
        }
        if &name != "" {
            if let Some(tex) = self.float_textures.get(&name) {
                return Arc::clone(tex);
            } else {
                error!("Couldn't find float texture {} for parameter {}", name, n);
            }
        }
        // If texture wasn't found
        let val = self.material_params.find_one_float(n, default);
        let val = self.geom_params.find_one_float(n, val);
        Arc::new(ConstantTexture::new(val))
    }

    pub fn get_float_texture_or_none(&self, n: &str) -> Option<Arc<dyn Texture<f32>>> {
        let mut name = self.geom_params.find_texture(n, "".to_owned());
        if &name == "" {
            name = self.material_params.find_texture(n, "".to_owned());
        }
        if &name != "" {
            if let Some(tex) = self.float_textures.get(&name) {
                return Some(Arc::clone(tex));
            } else {
                error!("Couldn't find float texture {} for parameter {}", name, n);
                return None;
            }
        }
        // If texture wasn't found
        self.geom_params
            .find_float(n)
            .or_else(|| self.material_params.find_float(n))
            .map(|val| {
                let tex: Arc<dyn Texture<f32>> = Arc::new(ConstantTexture::new(val[0]));
                tex
            })
    }
}

pub trait TextureMapping2D: Debug + Send + Sync {
    fn map(&self, si: &SurfaceInteraction) -> (Point2f, Vector2f, Vector2f);
}

#[derive(Debug)]
pub struct UVMapping2D {
    su: f32,
    sv: f32,
    du: f32,
    dv: f32,
}

impl UVMapping2D {
    pub fn new(su: f32, sv: f32, du: f32, dv: f32) -> Self {
        Self { su, sv, du, dv }
    }
}

impl TextureMapping2D for UVMapping2D {
    fn map(&self, si: &SurfaceInteraction) -> (Point2f, Vector2f, Vector2f) {
        (
            Point2f::new(self.su * si.uv.x + self.du, self.sv * si.uv.y + self.dv),
            // dstdx
            Vector2f::new(self.su * si.dudx, self.sv * si.dvdx),
            // dstdy
            Vector2f::new(self.su * si.dudy, self.sv * si.dvdy),
        )
    }
}

#[derive(Debug)]
pub struct SphericalMapping2D {
    inner: UVMapping2D,
    world_to_tex: Transform,
}

impl SphericalMapping2D {
    pub fn new(su: f32, sv: f32, du: f32, dv: f32, world_to_tex: Transform) -> Self {
        Self {
            inner: UVMapping2D::new(su, sv, du, dv),
            world_to_tex,
        }
    }

    fn sphere(&self, p: &Point3f) -> Point2f {
        let vec =
            (self.world_to_tex.transform_point(p).0 - Point3f::new(0.0, 0.0, 0.0)).normalize();
        let theta = spherical_theta(&vec);
        let phi = spherical_phi(&vec);

        Point2f::new(theta * FRAC_1_PI, phi * INV_2_PI)
    }
}

impl TextureMapping2D for SphericalMapping2D {
    fn map(&self, si: &SurfaceInteraction) -> (Point2f, Vector2f, Vector2f) {
        let st = self.sphere(&si.hit.p);
        let mut dstdx;
        let mut dstdy;

        // Compute texture coordinate differentials for sphere mapping
        let delta = 0.1;
        let st_delta_x = self.sphere(&(si.hit.p + delta * si.dpdx));
        dstdx = (st_delta_x - st) / delta;
        let st_delta_y = self.sphere(&(si.hit.p + delta * si.dpdy));
        dstdy = (st_delta_y - st) / delta;

        // Handle sphere mapping discontinuity for coordinate differentials
        if dstdx[1] > 0.5 {
            dstdx[1] = 1.0 - dstdx[1];
        } else if dstdx[1] < -0.5 {
            dstdx[1] = -(dstdx[1] + 1.0);
        }
        if dstdy[1] > 0.5 {
            dstdy[1] = 1.0 - (dstdy)[1];
        } else if dstdy[1] < -0.5 {
            dstdy[1] = -(dstdy[1] + 1.0);
        }

        (st, dstdx, dstdy)
    }
}

#[derive(Debug)]
pub struct ScaleTexture<T> {
    tex1: Arc<dyn Texture<T>>,
    tex2: Arc<dyn Texture<T>>,
}

impl<T> Texture<T> for ScaleTexture<T>
where
    T: Debug,
    T: Send,
    T: Sync,
    T: Mul<Output = T>,
{
    fn evaluate(&self, si: &SurfaceInteraction<'_, '_>) -> T {
        self.tex1.evaluate(si) * self.tex2.evaluate(si)
    }
}

impl ScaleTexture<Spectrum> {
    pub fn create(tp: &TextureParams<'_>) -> ScaleTexture<Spectrum> {
        let tex1 = tp.get_spectrum_texture("tex1", &Colors::WHITE);
        let tex2 = tp.get_spectrum_texture("tex2", &Colors::WHITE);

        ScaleTexture { tex1, tex2 }
    }
}

impl ScaleTexture<f32> {
    pub fn create(tp: &TextureParams<'_>) -> ScaleTexture<f32> {
        let tex1 = tp.get_float_texture("tex1", 1.0);
        let tex2 = tp.get_float_texture("tex2", 1.0);

        ScaleTexture { tex1, tex2 }
    }
}

#[derive(Debug)]
pub struct MixTexture<T> {
    tex1: Arc<dyn Texture<T>>,
    tex2: Arc<dyn Texture<T>>,
    amount: Arc<TextureFloat>,
}

impl<T> Texture<T> for MixTexture<T>
where
    T: Debug,
    T: Mul<f32, Output = T>,
    T: Add<Output = T>,
{
    fn evaluate(&self, si: &SurfaceInteraction<'_, '_>) -> T {
        let t1 = self.tex1.evaluate(si);
        let t2 = self.tex2.evaluate(si);
        let amt = self.amount.evaluate(si);

        t1 * (1.0 - amt) + t2 * amt
    }
}

impl MixTexture<f32> {
    pub fn create_float(_tex2world: &Transform, tp: &TextureParams<'_>) -> MixTexture<f32> {
        MixTexture {
            tex1: tp.get_float_texture("tex1", 0.0),
            tex2: tp.get_float_texture("tex2", 1.0),
            amount: tp.get_float_texture("amount", 0.5),
        }
    }
}

impl MixTexture<Spectrum> {
    pub fn create_spectrum(_tex2world: &Transform, tp: &TextureParams<'_>) -> MixTexture<Spectrum> {
        MixTexture {
            tex1: tp.get_spectrum_texture("tex1", &Colors::BLACK),
            tex2: tp.get_spectrum_texture("tex2", &Colors::WHITE),
            amount: tp.get_float_texture("amount", 0.5),
        }
    }
}

#[derive(Debug)]
pub struct ImageTexture<T> {
    mapping: Box<dyn TextureMapping2D>,
    mipmap: Arc<MIPMap<T>>,
}

impl<T> ImageTexture<T>
where
    T: Zero,
    T: Clone,
    T: Copy,
    T: Clampable,
    T: Debug,
    T: AddAssign<T>,
    T: Mul<f32, Output = T>,
    T: Div<f32, Output = T>,
    T: Sized,
    T: Send + Sync,
{
    pub fn new<F: Fn(&Spectrum) -> T>(
        path: &Path,
        wrap_mode: WrapMode,
        trilerp: bool,
        max_aniso: f32,
        scale: f32,
        gamma: bool,
        map: Box<dyn TextureMapping2D>,
        convert: F,
    ) -> ImageTexture<T> {
        debug!("Loading texture {}", path.display());
        let (res, texels) = match utils::read_image(path) {
            Ok((mut pixels, res)) => {
                // Flip image in y; texture coordinate space has (0,0) at the lower
                // left corner.
                for y in 0..res.y / 2 {
                    for x in 0..res.x {
                        let o1 = (y * res.x + x) as usize;
                        let o2 = ((res.y - 1 - y) * res.x + x) as usize;
                        pixels.swap(o1, o2);
                    }
                }

                (res, pixels)
            }
            Err(e) => {
                warn!(
                    "Could not open texture file. Using grey texture instead: {}",
                    e
                );
                (Point2i::new(1, 1), vec![Spectrum::grey(0.18)])
            }
        };

        let converted_texels: Vec<T> = texels
            .iter()
            .map(|p| {
                let s = if gamma {
                    scale * p.inverse_gamma_correct()
                } else {
                    scale * *p
                };
                convert(&s)
            })
            .collect();

        let mipmap = Arc::new(MIPMap::new(
            res,
            &converted_texels[..],
            trilerp,
            max_aniso,
            wrap_mode,
        ));
        ImageTexture {
            mapping: map,
            mipmap,
        }
    }
}

impl ImageTexture<Spectrum> {
    pub fn create(_tex2world: &Transform, tp: &TextureParams<'_>) -> ImageTexture<Spectrum> {
        let typ = tp.find_string("mapping", "uv");
        let map = if typ == "uv" {
            let su = tp.find_float("uscale", 1.0);
            let sv = tp.find_float("vscale", 1.0);
            let du = tp.find_float("udelta", 0.0);
            let dv = tp.find_float("vdelta", 0.0);

            UVMapping2D::new(su, sv, du, dv)
        } else {
            unimplemented!()
        };
        let max_aniso = tp.find_float("maxanisotropy", 8.0);
        let trilerp = tp.find_bool("trilinear", false);
        let wrap = tp.find_string("wrap", "repeat");
        let wrap_mode = if wrap == "black" {
            WrapMode::Black
        } else if wrap == "clamp" {
            WrapMode::Clamp
        } else {
            WrapMode::Repeat
        };
        let scale = tp.find_float("scale", 1.0);
        let filename = tp.find_filename("filename", "");
        let gamma = tp.find_bool(
            "gamma",
            utils::has_extension(&filename, "tga") || utils::has_extension(&filename, "png"),
        );

        Self::new(
            Path::new(&filename),
            wrap_mode,
            trilerp,
            max_aniso,
            scale,
            gamma,
            Box::new(map),
            convert_to_spectrum,
        )
    }

    pub fn dump_mipmap(&self) {
        info!("Dumping MIPMap levels for debugging...");
        self.mipmap
            .pyramid
            .iter()
            .enumerate()
            .for_each(|(i, level)| {
                let mut buf = Vec::new();
                for y in 0..level.v_size() {
                    for x in 0..level.u_size() {
                        let p = level[(x, y)];
                        buf.push(p[0]);
                        buf.push(p[1]);
                        buf.push(p[2]);
                    }
                }
                utils::write_image(
                    format!("mipmap_level_{}.png", i),
                    &buf[..],
                    &Bounds2i::from_elements(0, 0, level.u_size() as i32, level.v_size() as i32),
                    Point2i::new(level.u_size() as i32, level.v_size() as i32),
                )
                .unwrap();
            });
    }
}

impl ImageTexture<f32> {
    pub fn create(_tex2world: &Transform, tp: &TextureParams<'_>) -> ImageTexture<f32> {
        let typ = tp.find_string("mapping", "uv");
        let map = if typ == "uv" {
            let su = tp.find_float("uscale", 1.0);
            let sv = tp.find_float("vscale", 1.0);
            let du = tp.find_float("udelta", 0.0);
            let dv = tp.find_float("vdelta", 0.0);

            UVMapping2D::new(su, sv, du, dv)
        } else {
            unimplemented!()
        };
        let max_aniso = tp.find_float("maxanisotropy", 8.0);
        let trilerp = tp.find_bool("trilinear", false);
        let wrap = tp.find_string("wrap", "repeat");
        let wrap_mode = if wrap == "black" {
            WrapMode::Black
        } else if wrap == "clamp" {
            WrapMode::Clamp
        } else {
            WrapMode::Repeat
        };
        let scale = tp.find_float("scale", 1.0);
        let filename = tp.find_filename("filename", "");
        let gamma = tp.find_bool(
            "gamma",
            utils::has_extension(&filename, "tga") || utils::has_extension(&filename, "png"),
        );

        Self::new(
            Path::new(&filename),
            wrap_mode,
            trilerp,
            max_aniso,
            scale,
            gamma,
            Box::new(map),
            convert_to_float,
        )
    }
}
fn convert_to_spectrum(from: &Spectrum) -> Spectrum {
    *from
}

fn convert_to_float(from: &Spectrum) -> f32 {
    from.y()
}

impl<T> Texture<T> for ImageTexture<T>
where
    T: Zero,
    T: Clone,
    T: Copy,
    T: Send,
    T: Sync,
    T: Clampable,
    T: Debug,
    T: AddAssign<T>,
    T: Mul<f32, Output = T>,
    T: Div<f32, Output = T>,
    T: Sized,
{
    fn evaluate(&self, si: &SurfaceInteraction<'_, '_>) -> T {
        let (st, dstdx, dstdy) = self.mapping.map(si);
        self.mipmap.lookup_diff(st, dstdx, dstdy)
    }
}
