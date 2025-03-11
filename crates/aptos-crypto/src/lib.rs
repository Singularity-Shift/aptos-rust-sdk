// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! A library supplying various cryptographic primitives
pub mod asymmetric_encryption;
pub mod bls12381;
pub mod compat;
pub mod ed25519;
pub mod elgamal;
pub mod encoding_type;
pub mod error;
pub mod hash;
pub mod hkdf;
pub mod multi_ed25519;
pub mod poseidon_bn254;
pub mod secp256k1_ecdsa;
pub mod secp256r1_ecdsa;
pub mod test_utils;
pub mod traits;
#[cfg(test)]
mod unit_tests;
pub mod validatable;

pub use self::traits::*;
pub use hash::HashValue;
// Reexport once_cell and serde_name for use in CryptoHasher Derive implementation.
#[doc(hidden)]
pub use once_cell as _once_cell;
#[doc(hidden)]
pub use serde_name as _serde_name;
