use super::{Rgb, Spectrum};
use crate::{
    parser::{value::Value, NamedToken},
    remove_default,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum BSDFFloat {
    Texture(String),
    Float(f32),
}

// BSDF representation
#[derive(Debug)]
pub enum BSDF {
    Matte {
        kd: Spectrum,             // 0.5
        sigma: Option<BSDFFloat>, // Pure lambertian if not provided
        bumpmap: Option<BSDFFloat>,
    },
    Metal {
        eta: Spectrum,              // Cu
        k: Spectrum,                // Cu
        distribution: Distribution, // 0.01 Iso
        bumpmap: Option<BSDFFloat>,
    },
    Substrate {
        kd: Spectrum,               // 0.5
        ks: Spectrum,               // 0.5
        distribution: Distribution, // 0.1
        bumpmap: Option<BSDFFloat>,
    },
    Glass {
        kr: Spectrum, // 1
        kt: Spectrum, // 1
        distribution: Option<Distribution>,
        eta: BSDFFloat, // 1.5
        bumpmap: Option<BSDFFloat>,
    },
    Mirror {
        kr: Spectrum, // 0.9
        bumpmap: Option<BSDFFloat>,
    },
}

impl BSDF {
    pub fn new(mut named_token: NamedToken, unamed: bool) -> Option<Self> {
        // Get the BSDF type
        let bsdf_type = if unamed {
            named_token.internal_type
        } else {
            named_token
                .values
                .remove("type")
                .expect("bsdf type param is required")
                .into_string()
        };

        let bumpmap = named_token
            .values
            .remove("bumpmap")
            .map(|v| v.into_bsdf_float());

        let parse_distribution =
            |map: &mut HashMap<String, Value>, default: Option<f32>| -> Option<Distribution> {
                let remaproughness =
                    remove_default!(map, "remaproughness", Value::Boolean(true)).into_bool();
                let alpha = match map.remove("roughness") {
                    Some(v) => Some(Roughness::Isotropic(v.into_bsdf_float())),
                    None => {
                        let u = map.remove("uroughness");
                        let v = map.remove("vroughness");
                        match (u, v) {
                            (Some(u), Some(v)) => {
                                let u = u.into_bsdf_float();
                                let v = v.into_bsdf_float();
                                match (u, v) {
                                    (BSDFFloat::Float(v_u), BSDFFloat::Float(v_v)) => {
                                        if v_u == v_v {
                                            Some(Roughness::Isotropic(BSDFFloat::Float(v_v)))
                                        } else {
                                            Some(Roughness::Anisotropic {
                                                u: BSDFFloat::Float(v_u),
                                                v: BSDFFloat::Float(v_v),
                                            })
                                        }
                                    }
                                    (u, v) => Some(Roughness::Anisotropic { u, v }),
                                }
                            }
                            (None, None) => None,
                            (u, v) => panic!("{:?} {:?} roughness issue", u, v),
                        }
                    }
                };

                let alpha = alpha.map_or_else(
                    || default.map(|val| Roughness::Isotropic(BSDFFloat::Float(val))),
                    Some,
                );

                alpha.map(|roughness| Distribution {
                    roughness,
                    remaproughness,
                })
            };

        let bsdf = match &bsdf_type[..] {
            "matte" => {
                let kd = remove_default!(named_token.values, "Kd", Value::Rgb(Rgb::color(0.5)))
                    .into_spectrum();
                let sigma = named_token
                    .values
                    .remove("sigma")
                    .map(|v| v.into_bsdf_float());
                Some(BSDF::Matte { kd, sigma, bumpmap })
            }
            "metal" => {
                // TODO: Need to be able to export other material params
                let eta = remove_default!(
                    named_token.values,
                    "eta",
                    Value::Rgb(Rgb {
                        r: 0.199_990_69,
                        g: 0.922_084_6,
                        b: 1.099_875_9
                    })
                )
                .into_spectrum();
                let k = remove_default!(
                    named_token.values,
                    "k",
                    Value::Rgb(Rgb {
                        r: 3.904_635_4,
                        g: 2.447_633_3,
                        b: 2.137_652_6
                    })
                )
                .into_spectrum();

                let distribution = parse_distribution(&mut named_token.values, Some(0.01)).unwrap();
                Some(BSDF::Metal {
                    eta,
                    k,
                    distribution,
                    bumpmap,
                })
            }
            "substrate" => {
                let kd = remove_default!(named_token.values, "Kd", Value::Rgb(Rgb::color(0.5)))
                    .into_spectrum();
                let ks = remove_default!(named_token.values, "Ks", Value::Rgb(Rgb::color(0.5)))
                    .into_spectrum();
                let distribution = parse_distribution(&mut named_token.values, Some(0.1)).unwrap();
                Some(BSDF::Substrate {
                    kd,
                    ks,
                    distribution,
                    bumpmap,
                })
            }
            "glass" => {
                let kr = remove_default!(named_token.values, "Kr", Value::Rgb(Rgb::color(1.0)))
                    .into_spectrum();
                let kt = remove_default!(named_token.values, "Kt", Value::Rgb(Rgb::color(1.0)))
                    .into_spectrum();
                let eta = if let Some(eta) = named_token.values.remove("eta") {
                    eta.into_bsdf_float()
                } else {
                    remove_default!(named_token.values, "index", Value::Float(vec![1.5]))
                        .into_bsdf_float()
                };
                let distribution = parse_distribution(&mut named_token.values, None);

                Some(BSDF::Glass {
                    kr,
                    kt,
                    distribution,
                    eta,
                    bumpmap,
                })
            }
            "mirror" => {
                let kr = remove_default!(named_token.values, "Kr", Value::Rgb(Rgb::color(0.9)))
                    .into_spectrum();
                Some(BSDF::Mirror { kr, bumpmap })
            }
            _ => {
                warn!("BSDF case with {} is not cover", bsdf_type);
                None
            }
        };

        if bsdf.is_some() & !named_token.values.is_empty() {
            panic!("Miss parameters: {:?}", named_token.values);
        }

        bsdf
    }
}

#[derive(Debug)]
pub enum Roughness {
    Isotropic(BSDFFloat),
    Anisotropic { u: BSDFFloat, v: BSDFFloat },
}

#[derive(Debug)]
pub struct Distribution {
    pub roughness: Roughness, // Depends of the material (metal: 0.01 iso, glass optional)
    pub remaproughness: bool, // True
}
