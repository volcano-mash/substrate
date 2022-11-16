use bls12::{Bls12, Bls12Parameters, TwistType};

pub use ark_bls12_381::{g1::*, g2::*, Parameters};

// pub mod g1;
// pub mod g2;
// pub(crate) mod util;

#[cfg(test)]
mod tests;


pub type Bls12_381 = Bls12<Parameters>;

// pub struct Parameters;

// impl Bls12Parameters for Parameters {
// 	const X: &'static [u64] = &[0xd201000000010000];
// 	const X_IS_NEGATIVE: bool = true;
// 	const TWIST_TYPE: TwistType = TwistType::M;
// 	type Fp = Fq;
// 	type Fp2Config = Fq2Config;
// 	type Fp6Config = Fq6Config;
// 	type Fp12Config = Fq12Config;
// 	type G1Parameters = ark_bls12_381::g1::Parameters;
// 	type G2Parameters = self::g2::Parameters;
// }
