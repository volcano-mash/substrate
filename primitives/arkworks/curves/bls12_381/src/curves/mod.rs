use crate::{Fq, Fq12Config, Fq2Config, Fq6Config};
use ark_ec::pairing::Pairing;
use ark_ff::Fp12;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress};
use ark_std::{io::Cursor, vec, vec::Vec};
use ark_sub_models::bls12::{Bls12, Bls12Parameters, TwistType};
use sp_io::crypto::bls12_381_final_exponentiation;

pub mod g1;
pub mod g2;
pub(crate) mod util;

pub use self::{
	g1::{G1Affine, G1Projective},
	g2::{G2Affine, G2Projective},
};

pub struct Parameters;

impl Bls12Parameters for Parameters {
	const X: &'static [u64] = &[0xd201000000010000];
	const X_IS_NEGATIVE: bool = true;
	const TWIST_TYPE: TwistType = TwistType::M;
	type Fp = Fq;
	type Fp2Config = Fq2Config;
	type Fp6Config = Fq6Config;
	type Fp12Config = Fq12Config;
	type G1Parameters = self::g1::Parameters;
	type G2Parameters = self::g2::Parameters;

	fn multi_miller_loop(
		a_vec: Vec<ark_sub_models::bls12::G1Prepared<Self>>,
		b_vec: Vec<ark_sub_models::bls12::G2Prepared<Self>>,
	) -> <Bls12<Self> as Pairing>::TargetField {
		let a_vec: Vec<Vec<u8>> = a_vec
			.into_iter()
			.map(|elem| {
				let elem: <Bls12<Self> as Pairing>::G1Prepared = elem.into();
				let mut serialized = vec![0; elem.serialized_size(Compress::Yes)];
				let mut cursor = Cursor::new(&mut serialized[..]);
				elem.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();
				serialized
			})
			.collect();
		let b_vec = b_vec
			.into_iter()
			.map(|elem| {
				let elem: <Bls12<Self> as Pairing>::G2Prepared = elem.into();
				let mut serialized = vec![0u8; elem.serialized_size(Compress::Yes)];
				let mut cursor = Cursor::new(&mut serialized[..]);
				elem.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();
				serialized
			})
			.collect();

		let res = sp_io::crypto::bls12_381_multi_miller_loop(a_vec, b_vec);
		let cursor = Cursor::new(&res[..]);
		let f: <Bls12<Self> as Pairing>::TargetField =
			Fp12::deserialize_with_mode(cursor, Compress::Yes, ark_serialize::Validate::No)
				.unwrap();
		f
	}

	fn final_exponentiation(
		f12: <Bls12<Self> as Pairing>::TargetField,
	) -> <Bls12<Self> as Pairing>::TargetField {
		let mut out: [u8; 576] = [0; 576];
		let mut cursor = Cursor::new(&mut out[..]);
		f12.serialize_with_mode(&mut cursor, Compress::Yes).unwrap();

		let res = bls12_381_final_exponentiation(&out);

		let cursor = Cursor::new(&res[..]);
		let res: <Bls12<Self> as Pairing>::TargetField =
			Fp12::deserialize_with_mode(cursor, Compress::Yes, ark_serialize::Validate::No)
				.unwrap();

		res
	}
}

pub type Bls12_381 = Bls12<Parameters>;
