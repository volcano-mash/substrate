#![cfg_attr(not(feature = "std"), no_std)]

use ark_ec::{
	models::CurveConfig,
	pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use ark_ff::fields::fp12_2over3over2::Fp12;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress};
use ark_std::{io::Cursor, marker::PhantomData, vec, vec::Vec};
use derivative::Derivative;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub use ark_ec::models::bls12::{
	Bls12Parameters, TwistType,
};

pub mod g1;
pub mod g2;

pub use self::g1::{G1Affine, G1Prepared, G1Projective};
pub use self::g2::{G2Affine, G2Prepared, G2Projective};

pub trait HostFunctions: 'static {
	fn multi_miller_loop(a_vec: Vec<Vec<u8>>, b_vec: Vec<Vec<u8>>) -> Vec<u8>;
	fn final_exponentiation(f12: &[u8]) -> Vec<u8>;
}

#[derive(Derivative)]
#[derivative(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Bls12<P: Bls12Parameters, Q: HostFunctions> {
	phantom1: PhantomData<fn() -> P>,
	phantom2: PhantomData<fn() -> Q>,
}

impl<P: Bls12Parameters, Q: HostFunctions> Pairing for Bls12<P, Q> {
	type BaseField = <P::G1Parameters as CurveConfig>::BaseField;
	type ScalarField = <P::G1Parameters as CurveConfig>::ScalarField;
	type G1 = G1Projective<P>;
	type G1Affine = G1Affine<P>;
	type G1Prepared = G1Prepared<P>;
	type G2 = G2Projective<P>;
	type G2Affine = G2Affine<P>;
	type G2Prepared = G2Prepared<P>;
	type TargetField = Fp12<P::Fp12Config>;

	fn multi_miller_loop(
		a: impl IntoIterator<Item = impl Into<Self::G1Prepared>>,
		b: impl IntoIterator<Item = impl Into<Self::G2Prepared>>,
	) -> MillerLoopOutput<Self> {
		let a_vec: Vec<Vec<u8>> = a
			.into_iter()
			.map(|elem| {
				let elem: Self::G1Prepared = elem.into();
				let mut serialized = vec![0; elem.serialized_size(Compress::Yes)];
				let mut cursor = Cursor::new(&mut serialized[..]);
				elem.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();
				serialized
			})
			.collect();
		let b_vec = b
			.into_iter()
			.map(|elem| {
				let elem: Self::G2Prepared = elem.into();
				let mut serialized = vec![0u8; elem.serialized_size(Compress::Yes)];
				let mut cursor = Cursor::new(&mut serialized[..]);
				elem.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();
				serialized
			})
			.collect();

		let res = Q::multi_miller_loop(a_vec, b_vec);
		let cursor = Cursor::new(&res[..]);
		let f: Self::TargetField =
			Fp12::deserialize_with_mode(cursor, Compress::Yes, ark_serialize::Validate::No)
				.unwrap();

		MillerLoopOutput(f)
	}

	fn final_exponentiation(f: MillerLoopOutput<Self>) -> Option<PairingOutput<Self>> {
		let mut out: [u8; 576] = [0; 576];
		let mut cursor = Cursor::new(&mut out[..]);
		f.0.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();

		let res = Q::final_exponentiation(&out[..]);

		let cursor = Cursor::new(&res[..]);
		let r: Self::TargetField =
			Fp12::deserialize_with_mode(cursor, Compress::Yes, ark_serialize::Validate::No)
				.unwrap();

		Some(PairingOutput(r))
	}
}
