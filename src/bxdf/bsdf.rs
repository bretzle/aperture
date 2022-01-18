//! Defines the BSDF which acts as a container for composing the various BRDFs
//! and BTDFs that describe the surface's properties

use enum_set::EnumSet;
use std::cmp;

use crate::{
    bxdf::{BxDF, BxDFType, BxDFs},
    film::Colorf,
    geometry::DifferentialGeometry,
    linalg::{self, Normal, Point, Vector},
    sampler::Sample,
};

/// The BSDF contains the various BRDFs and BTDFs that describe the surface's properties
/// at some point. It also transforms incident and outgoing light directions into
/// shading space to make the BxDFs easier to implement.
/// TODO: We really need the memory pool. Each time we get the bsdf from a
/// material we need to allocate a decent amount of stuff since they each need
/// their own tangent, bitangent and differential geometry reference.
pub struct BSDF<'a> {
    /// The hit point
    pub p: Point,
    /// Shading normal, may be perturbed by bump mapping
    pub n: Normal,
    /// The actual geometry normal
    pub ng: Normal,
    /// Tangent vector for the surface
    pub tan: Vector,
    /// Bitangent vector for the surface
    pub bitan: Vector,
    /// Refractive index of the geometry
    pub eta: f32,
    bxdfs: &'a [&'a BxDFs<'a>],
}

impl<'a> BSDF<'a> {
    /// Create a new BSDF using the BxDFs passed to shade the differential geometry with
    /// refractive index `eta`
    pub fn new<'b>(bxdfs: &'a [&'a BxDFs], eta: f32, dg: &DifferentialGeometry<'b>) -> Self {
        let n = dg.n.normalized();
        let mut bitan = dg.dp_du.normalized();
        let tan = linalg::cross(&n, &bitan);
        bitan = linalg::cross(&tan, &n);
        Self {
            p: dg.p,
            n,
            ng: dg.ng,
            tan,
            bitan,
            bxdfs,
            eta,
        }
    }

    /// Return the total number of BxDFs
    pub fn num_bxdfs(&self) -> usize {
        self.bxdfs.len()
    }

    /// Return the number of BxDFs matching the flags
    pub fn num_matching(&self, flags: EnumSet<BxDFType>) -> usize {
        self.bxdfs.iter().filter(|x| x.matches(flags)).count()
    }

    /// Transform the vector from world space to shading space
    pub fn to_shading(&self, v: &Vector) -> Vector {
        Vector::new(
            linalg::dot(v, &self.bitan),
            linalg::dot(v, &self.tan),
            linalg::dot(v, &self.n),
        )
    }

    /// Transform the vectro from shading space to world space
    pub fn from_shading(&self, v: &Vector) -> Vector {
        Vector::new(
            self.bitan.x * v.x + self.tan.x * v.y + self.n.x * v.z,
            self.bitan.y * v.x + self.tan.y * v.y + self.n.y * v.z,
            self.bitan.z * v.x + self.tan.z * v.y + self.n.z * v.z,
        )
    }

    /// Evaluate the BSDF for the outgoing and incident light directions
    /// `w_o` and `w_i` in world space, sampling the desired subset of BxDFs
    /// selected by the flags passed. `wo_world` and `wi_world` should point from
    /// the hit point in the outgoing and incident light directions respectively.
    pub fn eval(
        &self,
        wo_world: &Vector,
        wi_world: &Vector,
        mut flags: EnumSet<BxDFType>,
    ) -> Colorf {
        let w_o = self.to_shading(wo_world).normalized();
        let w_i = self.to_shading(wi_world).normalized();
        // Determine if we should evaluate reflection or transmission based on the
        // geometry normal and the light directions
        if w_o.z * w_i.z > 0.0 {
            flags.remove(&BxDFType::Transmission);
        } else {
            flags.remove(&BxDFType::Reflection);
        }
        // Find all matching BxDFs and add their contribution to the material's color
        self.bxdfs
            .iter()
            .filter_map(|x| {
                if x.matches(flags) {
                    Some(x.eval(&w_o, &w_i))
                } else {
                    None
                }
            })
            .fold(Colorf::broadcast(0.0), |x, y| x + y)
    }

    /// Sample a component of the BSDF to get an incident light direction for light
    /// leaving the surface along `w_o`.
    /// `samples` are the 3 random values to use when sampling a component of the BSDF
    /// and a the chosen BSDF
    /// Returns the color, direction, pdf and the type of BxDF that was sampled.
    pub fn sample(
        &self,
        wo_world: &Vector,
        flags: EnumSet<BxDFType>,
        samples: &Sample,
    ) -> (Colorf, Vector, f32, EnumSet<BxDFType>) {
        let n_matching = self.num_matching(flags);
        if n_matching == 0 {
            return (
                Colorf::broadcast(0.0),
                Vector::broadcast(0.0),
                0.0,
                EnumSet::new(),
            );
        }
        let comp = cmp::min((samples.one_d * n_matching as f32) as usize, n_matching - 1);
        let bxdf = self.matching_at(comp, flags);
        let w_o = self.to_shading(wo_world).normalized();
        let (mut f, w_i, mut pdf) = bxdf.sample(&w_o, &samples.two_d);
        if w_i.length_sqr() == 0.0 {
            return (
                Colorf::broadcast(0.0),
                Vector::broadcast(0.0),
                0.0,
                EnumSet::new(),
            );
        }
        let wi_world = self.from_shading(&w_i).normalized();

        // TODO: We re-use our functions but actually do a lot of redundant computation. I'm not
        // sure that the compiler will eliminate it. Should just copy in the code from pdf and eval
        if !bxdf.bxdf_type().contains(&BxDFType::Specular) && n_matching > 1 {
            pdf = self.pdf(wo_world, &wi_world, flags);
        }

        if !bxdf.bxdf_type().contains(&BxDFType::Specular) {
            f = self.eval(wo_world, &wi_world, flags);
        }
        (f, wi_world, pdf, bxdf.bxdf_type())
    }

    /// Compute the pdf for sampling the pair of incident and outgoing light directions for
    /// the BxDFs matching the flags set
    pub fn pdf(&self, wo_world: &Vector, wi_world: &Vector, flags: EnumSet<BxDFType>) -> f32 {
        let w_o = self.to_shading(wo_world).normalized();
        let w_i = self.to_shading(wi_world).normalized();
        let (pdf_val, n_comps) = self
            .bxdfs
            .iter()
            .filter_map(|x| {
                if x.matches(flags) {
                    Some(x.pdf(&w_o, &w_i))
                } else {
                    None
                }
            })
            .fold((0.0, 0), |(p, n), y| (p + y, n + 1));
        if n_comps > 0 {
            pdf_val / n_comps as f32
        } else {
            0.0
        }
    }

    /// Get the `i`th BxDF that matches the flags passed. There should not be fewer than i
    /// BxDFs that match the flags
    fn matching_at(&self, i: usize, flags: EnumSet<BxDFType>) -> &BxDFs {
        let mut it = self.bxdfs.iter().filter(|x| x.matches(flags)).skip(i);
        match it.next() {
            Some(b) => *b,
            None => panic!("Out of bounds index for BxDF type {:?}", flags),
        }
    }
}
