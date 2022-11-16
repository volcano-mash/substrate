#![cfg_attr(not(feature = "std"), no_std)]
#![deny(future_incompatible, nonstandard_style, rust_2018_idioms)]
#![forbid(unsafe_code)]

//! This library implements the BLS12_381 curve generated by [Sean Bowe](https://electriccoin.co/blog/new-snark-curve/).
//! The name denotes that it is a Barreto--Lynn--Scott curve of embedding degree
//! 12, defined over a 381-bit (prime) field.
//! This curve was intended to replace the BN254 curve to provide a higher
//! security level without incurring a large performance overhead.
//!
//!
//! Curve information:
//! * Base field: q = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
//! * Scalar field: r =
//!   52435875175126190479447740508185965837690552500527637822603658699938581184513
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 32
//! * G1 curve equation: y^2 = x^3 + 4
//! * G2 curve equation: y^2 = x^3 + Fq2(4, 4)

#[cfg(feature = "curve")]
mod curves;
// mod fields;

#[cfg(feature = "curve")]
pub use ark_bls12_381::{fr::*, fq::*, fq2::*, fq6::*, fq12::*};
pub use curves::*;
// pub use fields::*;
