use crate::altbn128::fq::Fq;
use crate::altbn128::fr::Fr;
use ark_ec::{
    models::CurveConfig,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
};

pub type G1Affine = Affine<Config>;
pub type G1Projective = Projective<Config>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

pub const G1_GENERATOR_X: Fq = ark_ff::MontFp!("1");
pub const G1_GENERATOR_Y: Fq = ark_ff::MontFp!("2");

// TODO: update curve config
impl CurveConfig for Config {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = (x - 1)^2 / 3  = 76329603384216526031706109802092473003
    const COFACTOR: &'static [u64] = &[0x1, 0x0];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 52435875175126190458656871551744051925719901746859129887267498875565241663483
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = ark_ff::MontFp!("1");
}

impl SWCurveConfig for Config {
    /// COEFF_A = 0
    const COEFF_A: Fq = ark_ff::MontFp!("0");

    /// COEFF_B = 4
    #[rustfmt::skip]
    const COEFF_B: Fq = ark_ff::MontFp!("3");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
}
