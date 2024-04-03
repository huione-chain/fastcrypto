// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error::{FastCryptoError, FastCryptoResult};
use crate::traits::AllowedRng;
use core::ops::{Add, Div, Mul, Neg, Sub};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::{AddAssign, SubAssign};
use crate::serde_helpers::ToFromByteArray;

pub mod bls12381;
pub mod ristretto255;
pub mod secp256r1;

pub mod multiplier;

/// Trait impl'd by elements of an additive cyclic group.
pub trait GroupElement:
    Copy
    + Clone
    + Debug
    + Eq
    + Add<Output = Self>
    + AddAssign
    + for<'a> Add<&'a Self, Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + for<'a> Sub<&'a Self, Output = Self>
    + Neg<Output = Self>
    + Mul<Self::ScalarType, Output = Self>
    + Div<Self::ScalarType, Output = Result<Self, FastCryptoError>>
    + for<'a> Mul<&'a Self::ScalarType, Output = Self>
    + Sized
{
    /// Type of scalars used in the [Self::mul] multiplication method.
    type ScalarType: Scalar;

    /// Return an instance of the identity element in this group.
    fn zero() -> Self;

    /// Return an instance of the generator for this group.
    fn generator() -> Self;
}

// TODO: Move Serialize + DeserializeOwned to GroupElement.

/// Trait impl'd by scalars to be used with [GroupElement].
pub trait Scalar:
    GroupElement<ScalarType = Self> + Copy + From<u128> + Sized + Debug + Serialize + DeserializeOwned
{
    const SIZE_IN_BYTES: usize;

    fn rand<R: AllowedRng>(rng: &mut R) -> Self;
    fn inverse(&self) -> FastCryptoResult<Self>;
}


/// Assuming that
pub fn deserialize_vector<T: for<'a> Deserialize<'a>>(bytes: &[u8], size: usize) -> FastCryptoResult<Vec<T>> {
    if bytes.len() % size != 0 {
        return Err(FastCryptoError::InvalidInput);
    }
    bytes.chunks(size)
        .map(|chunk|
            bincode::deserialize(chunk)
                .map_err(|_| FastCryptoError::InvalidInput)
        )
        .collect::<FastCryptoResult<Vec<T>>>()
}

/// Trait for group elements that has a fast doubling operation.
pub trait Doubling {
    /// Compute 2 * Self = Self + Self.
    fn double(&self) -> Self;
}

pub trait Pairing: GroupElement {
    type Other: GroupElement;
    type Output;

    fn pairing(&self, other: &Self::Other) -> <Self as Pairing>::Output;
}

/// Trait for groups that have a reduction from a random buffer to a group element that is secure
/// when used for Fiat-Shamir. Note that the resulting group element is not guaranteed to be
/// uniformly distributed, but only to have enough entropy to be used for Fiat-Shamir heuristic.
pub trait FiatShamirChallenge {
    fn fiat_shamir_reduction_to_group_element(uniform_buffer: &[u8]) -> Self;
}

/// Trait for groups that have a standardized "hash_to_point"/"hash_to_curve" function (see
/// [https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-hash-to-curve#section-3].
pub trait HashToGroupElement {
    /// Hashes the given message and maps the result to a group element.
    fn hash_to_group_element(msg: &[u8]) -> Self;
}

/// Trait for groups that support multi-scalar multiplication.
pub trait MultiScalarMul: GroupElement {
    fn multi_scalar_mul(scalars: &[Self::ScalarType], points: &[Self]) -> FastCryptoResult<Self>;
}

/// Faster deserialization in case the input is trusted (otherwise it can be insecure).
pub trait FromTrustedByteArray<const LENGTH: usize>: Sized {
    fn from_trusted_byte_array(bytes: &[u8; LENGTH]) -> FastCryptoResult<Self>;
}
