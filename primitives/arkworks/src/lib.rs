// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Hashing Functions.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use ark_bls12_381::{Bls12_381, Fq12};
use ark_ec::{
	pairing::{MillerLoopOutput, Pairing}, bls12::G1Prepared,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use sp_std::vec::Vec;
use ark_std::io::Cursor;

const F12_COMPRESSED_SIZE: usize = 576;

/// Compute multi pairing through arkworks
pub fn multi_pairing(vec_a: Vec<Vec<u8>>, vec_b: Vec<Vec<u8>>) -> Vec<u8> {
	let g1: Vec<_> = vec_a
		.iter()
		.map(|a| {
			let cursor = Cursor::new(&a[..]);
			<Bls12_381 as Pairing>::G1Prepared::deserialize_with_mode(
				cursor,
				Compress::Yes,
				Validate::No,
			)
			.unwrap()
		})
		.collect();
	let g2: Vec<_> = vec_b
		.iter()
		.map(|b| {
			let cursor = Cursor::new(&b[..]);
			<Bls12_381 as Pairing>::G2Prepared::deserialize_with_mode(
				cursor,
				Compress::Yes,
				Validate::No,
			)
			.unwrap()
		})
		.collect();
	let res = Bls12_381::multi_pairing(g1, g2);
	// serialize the result
	let mut res_bytes = [0u8; F12_COMPRESSED_SIZE];
	let mut cursor = Cursor::new(&mut res_bytes[..]);
	res.0.serialize_compressed(&mut cursor).unwrap();
	res_bytes.to_vec()
}

/// Compute multi miller loop through arkworks
pub fn multi_miller_loop(a_vec: Vec<Vec<u8>>, b_vec: Vec<Vec<u8>>) -> Vec<u8> {
	let g1: Vec<_> = a_vec
		.iter()
		.map(|a| {
			let cursor = Cursor::new(&a[..]);
			<Bls12_381 as Pairing>::G1Affine::deserialize_with_mode(
				cursor,
				Compress::Yes,
				Validate::No,
			)
			.map(<Bls12_381 as Pairing>::G1Prepared::from).unwrap()
		})
		.collect();
	let g2: Vec<_> = b_vec
		.iter()
		.map(|b| {
			let cursor = Cursor::new(&b[..]);
			<Bls12_381 as Pairing>::G2Affine::deserialize_with_mode(
				cursor,
				Compress::Yes,
				Validate::No,
			)
			.map(<Bls12_381 as Pairing>::G2Prepared::from).unwrap()
		})
		.collect();
	let res = Bls12_381::multi_miller_loop(g1, g2);
	// serialize the result
	let mut res_bytes = [0u8; F12_COMPRESSED_SIZE];
	let mut cursor = Cursor::new(&mut res_bytes[..]);
	res.0.serialize_compressed(&mut cursor).unwrap();
	res_bytes.to_vec()
}

/// Compute final exponentiation through arkworks
pub fn final_exponentiation(f12: &[u8]) -> Vec<u8> {
	let cursor = Cursor::new(&f12[..]);
	let f12 = Fq12::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
	let res = Bls12_381::final_exponentiation(MillerLoopOutput(f12)).unwrap();
	// serialize the result
	let mut res_bytes = [0u8; F12_COMPRESSED_SIZE];
	let mut cursor = Cursor::new(&mut res_bytes[..]);
	res.0.serialize_compressed(&mut cursor).unwrap();
	res_bytes.to_vec()
}

#[cfg(test)]
mod tests {
	use super::*;
	use ark_ec::AffineRepr;
	use sp_std::vec;

	/// Just to make sure that everything behaves as expected with all the (de-)serialization
	/// happening.
	#[test]
	fn multi_pairing_works() {
		let [a, b] = [G1Affine::generator(), G1Affine::generator()];
		let [c, d] = [G2Affine::generator(), G2Affine::generator()];

		let [mut a_serialized, mut b_serialized] = [[0u8; 48], [0u8; 48]];
		let [mut c_serialized, mut d_serialized] = [[0u8; 96], [0u8; 96]];

		a.serialize_with_mode(a_serialized.as_mut_slice(), Compress::Yes).unwrap();
		b.serialize_with_mode(b_serialized.as_mut_slice(), Compress::Yes).unwrap();
		c.serialize_with_mode(c_serialized.as_mut_slice(), Compress::Yes).unwrap();
		d.serialize_with_mode(d_serialized.as_mut_slice(), Compress::Yes).unwrap();
		let result_1 = multi_pairing(
			vec![a_serialized.to_vec(), b_serialized.to_vec()],
			vec![c_serialized.to_vec(), d_serialized.to_vec()],
		);
		let result_1 = Fq12::deserialize_with_mode(
			&result_1[..],
			ark_serialize::Compress::Yes,
			ark_serialize::Validate::No,
		)
		.unwrap();
		let result_2 = Bls12_381::multi_pairing([a, b], [c, d]);
		assert_eq!(result_1, result_2.0);
	}
}
