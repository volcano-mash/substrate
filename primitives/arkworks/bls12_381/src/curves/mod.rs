use bls12::Bls12;

pub use ark_bls12_381::{g1::*, g2::*, Parameters};

pub type Bls12_381 = Bls12<Parameters>;

#[cfg(test)]
mod tests;
