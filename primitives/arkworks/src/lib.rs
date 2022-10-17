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

use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ec::{
	bls12::{G1Prepared, G2Prepared},
	pairing::{self, *},
	AffineRepr, CurveGroup, Group,
};
use ark_ff::{Field, PrimeField};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize, Compress, Validate};
use sp_std::vec::Vec;

pub fn pairing(a: &[u8], b: &[u8]) -> Vec<u8> {
	let g1 = G1Affine::deserialize_with_mode(a, Compress::Yes, Validate::Yes).unwrap();
	let g2 = G2Affine::deserialize_with_mode(b, Compress::Yes, Validate::Yes).unwrap();
	let res = Bls12_381::pairing(&g1, &g2);
	// serialize the result
	let mut res_bytes = [0u8; 576];
	res.0.serialize_compressed(&mut res_bytes[..]).unwrap();
	res_bytes.to_vec()
}
