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

use ark_bls12_381::{Bls12_381, Fq12, G1Affine, G2Affine};
use ark_ec::pairing::{MillerLoopOutput, Pairing};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use sp_std::vec::Vec;

/// Compute multi pairing through arkworks
pub fn multi_pairing(vec_a: Vec<Vec<u8>>, vec_b: Vec<Vec<u8>>) -> Vec<u8> {
	let g1: Vec<_> = vec_a
		.iter()
		.map(|a| G1Affine::deserialize_with_mode(&a[..], Compress::Yes, Validate::Yes).unwrap())
		.collect();
	let g2: Vec<_> = vec_b
		.iter()
		.map(|b| G2Affine::deserialize_with_mode(&b[..], Compress::Yes, Validate::Yes).unwrap())
		.collect();
	let res = Bls12_381::multi_pairing(&g1, &g2);
	// serialize the result
	let mut res_bytes = [0u8; 576];
	res.0.serialize_compressed(&mut res_bytes[..]).unwrap();
	res_bytes.to_vec()
}

/// Compute multi miller loop through arkworks
pub fn multi_miller_loop(a_vec: Vec<Vec<u8>>, b_vec: Vec<Vec<u8>>) -> Vec<u8> {
	let g1: Vec<_> = a_vec
		.iter()
		.map(|a| G1Affine::deserialize_with_mode(&a[..], Compress::Yes, Validate::Yes).unwrap())
		.collect();
	let g2: Vec<_> = b_vec
		.iter()
		.map(|b| G2Affine::deserialize_with_mode(&b[..], Compress::Yes, Validate::Yes).unwrap())
		.collect();
	let res = Bls12_381::multi_miller_loop(&g1, &g2);
	// serialize the result
	let mut res_bytes = [0u8; 576];
	res.0.serialize_compressed(&mut res_bytes[..]).unwrap();
	res_bytes.to_vec()
}

/// Compute final exponentiation through arkworks
pub fn final_exponentiation(f12: &[u8]) -> Vec<u8> {
	let f12 = Fq12::deserialize_with_mode(f12, Compress::Yes, Validate::Yes).unwrap();
	let res = Bls12_381::final_exponentiation(MillerLoopOutput(f12)).unwrap();
	// serialize the result
	let mut res_bytes = [0u8; 576];
	res.0.serialize_compressed(&mut res_bytes[..]).unwrap();
	res_bytes.to_vec()
}
