use bls12::{Bls12, HostFunctions};

pub use ark_bls12_381::{g1::*, g2::*, Parameters};
use ark_std::vec::Vec;
use sp_io::crypto::{bls12_381_final_exponentiation, bls12_381_multi_miller_loop};

pub struct Host;

impl HostFunctions for Host {
	fn multi_miller_loop(a_vec: Vec<Vec<u8>>, b_vec: Vec<Vec<u8>>) -> Vec<u8> {
		return bls12_381_multi_miller_loop(a_vec, b_vec)
	}

	fn final_exponentiation(f12: &[u8]) -> Vec<u8> {
		return bls12_381_final_exponentiation(f12)
	}
}

pub type Bls12_381 = Bls12<Parameters, Host>;
