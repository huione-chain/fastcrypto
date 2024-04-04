// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::bls12381::min_pk::DST_G2;
use crate::bls12381::min_sig::DST_G1;
use crate::encoding::{Encoding, Hex};
use crate::error::{FastCryptoError, FastCryptoResult};
use crate::groups::{
    FiatShamirChallenge, FromTrustedByteArray, GroupElement, HashToGroupElement, MultiScalarMul,
    Pairing, Scalar as ScalarType,
};
use crate::serde_helpers::BytesRepresentation;
use crate::serde_helpers::ToFromByteArray;
use crate::traits::AllowedRng;
use crate::utils::log2_byte;
use crate::{generate_bytes_representation, serialize_deserialize_with_to_from_byte_array};
use blst::{
    blst_bendian_from_scalar, blst_final_exp, blst_fp, blst_fp12, blst_fp12_inverse, blst_fp12_mul,
    blst_fp12_one, blst_fp12_sqr, blst_fp_from_bendian, blst_fr, blst_fr_add, blst_fr_cneg,
    blst_fr_from_scalar, blst_fr_from_uint64, blst_fr_inverse, blst_fr_mul, blst_fr_rshift,
    blst_fr_sub, blst_hash_to_g1, blst_hash_to_g2, blst_lendian_from_scalar, blst_miller_loop,
    blst_p1, blst_p1_add_or_double, blst_p1_affine, blst_p1_cneg, blst_p1_compress,
    blst_p1_from_affine, blst_p1_in_g1, blst_p1_mult, blst_p1_to_affine, blst_p1_uncompress,
    blst_p2, blst_p2_add_or_double, blst_p2_affine, blst_p2_cneg, blst_p2_compress,
    blst_p2_from_affine, blst_p2_in_g2, blst_p2_mult, blst_p2_to_affine, blst_p2_uncompress,
    blst_scalar, blst_scalar_fr_check, blst_scalar_from_be_bytes, blst_scalar_from_bendian,
    blst_scalar_from_fr, p1_affines, p2_affines, BLS12_381_G1, BLS12_381_G2, BLST_ERROR,
};
use fastcrypto_derive::GroupOpsExtend;
use hex_literal::hex;
use once_cell::sync::OnceCell;
use serde::{de, Deserialize};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ptr;

/// Elements of the group G_1 in BLS 12-381.
#[derive(Clone, Copy, Eq, PartialEq, GroupOpsExtend)]
#[repr(transparent)]
pub struct G1Element(blst_p1);

/// Elements of the group G_2 in BLS 12-381.
#[derive(Clone, Copy, Eq, PartialEq, GroupOpsExtend)]
#[repr(transparent)]
pub struct G2Element(blst_p2);

/// Elements of the subgroup G_T of F_q^{12} in BLS 12-381. Note that it is written in additive notation here.
#[derive(Clone, Copy, Eq, PartialEq, GroupOpsExtend)]
pub struct GTElement(blst_fp12);

/// This represents a scalar modulo r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
/// which is the order of the groups G1, G2 and GT. Note that r is a 255 bit prime.
#[derive(Clone, Copy, Eq, PartialEq, GroupOpsExtend)]
pub struct Scalar(pub blst_fr);

pub const SCALAR_LENGTH: usize = 32;
pub const G1_ELEMENT_BYTE_LENGTH: usize = 48;
pub const G2_ELEMENT_BYTE_LENGTH: usize = 96;
pub const GT_ELEMENT_BYTE_LENGTH: usize = 576;
pub const FP_BYTE_LENGTH: usize = 48;

impl Add for G1Element {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = blst_p1::default();
        unsafe {
            blst_p1_add_or_double(&mut ret, &self.0, &rhs.0);
        }
        Self(ret)
    }
}

impl Sub for G1Element {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::add(self, Self::neg(rhs))
    }
}

impl Neg for G1Element {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut ret = self.0;
        unsafe {
            blst_p1_cneg(&mut ret, true);
        }
        Self(ret)
    }
}

/// The size of this scalar in bytes.
fn size_in_bytes(scalar: &blst_scalar) -> usize {
    let mut i = scalar.b.len();
    debug_assert_eq!(i, 32);
    while i != 0 && scalar.b[i - 1] == 0 {
        i -= 1;
    }
    i
}

/// Given a scalar and its size in bytes (computed using [size_in_bytes], this method returns the size
/// of the scalar in bits.
fn size_in_bits(scalar: &blst_scalar, size_in_bytes: usize) -> usize {
    if size_in_bytes == 0 {
        0
    } else {
        8 * size_in_bytes - 7 + log2_byte(scalar.b[size_in_bytes - 1])
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Scalar> for G1Element {
    type Output = FastCryptoResult<Self>;

    fn div(self, rhs: Scalar) -> Self::Output {
        let inv = rhs.inverse()?;
        Ok(self * inv)
    }
}

impl Mul<Scalar> for G1Element {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut scalar: blst_scalar = blst_scalar::default();
        unsafe {
            blst_scalar_from_fr(&mut scalar, &rhs.0);
        }

        // Count the number of bytes to be multiplied.
        let bytes = size_in_bytes(&scalar);
        if bytes == 0 {
            return G1Element::zero();
        }

        // If rhs = 1, return self
        if bytes == 1 && scalar.b[0] == 1 {
            return self;
        }

        let mut result = blst_p1::default();
        unsafe {
            blst_p1_mult(
                &mut result,
                &self.0,
                &(scalar.b[0]),
                size_in_bits(&scalar, bytes),
            );
        }

        Self(result)
    }
}

impl MultiScalarMul for G1Element {
    fn multi_scalar_mul(scalars: &[Self::ScalarType], points: &[Self]) -> FastCryptoResult<Self> {
        if scalars.len() != points.len() || scalars.is_empty() {
            return Err(FastCryptoError::InvalidInput);
        }
        // Unfortunately we copy since blst does not filter out inf
        // https://github.com/supranational/blst/blob/master/src/multi_scalar.c#L11
        let (scalars, points): (Vec<_>, Vec<_>) = scalars
            .iter()
            .zip(points.iter())
            .filter(|(&s, &p)| s != Scalar::zero() && p != Self::zero())
            .map(|(&s, &p)| (s, p))
            .unzip();
        // We already checked that scalars is not empty above so if it's empty here, it means
        // that all the points are zero.
        if scalars.is_empty() {
            return Ok(Self::zero());
        }

        // Inspired by blstrs.
        let points = to_blst_type_slice(&points);
        let points = p1_affines::from(points);
        let mut scalar_bytes: Vec<u8> = Vec::with_capacity(scalars.len() * 32);
        for a in scalars.iter().map(|s| s.0) {
            let mut scalar: blst_scalar = blst_scalar::default();
            unsafe {
                blst_scalar_from_fr(&mut scalar, &a);
            }
            scalar_bytes.extend_from_slice(&scalar.b);
        }
        // The scalar field size is smaller than 2^255, so we need at most 255 bits.
        let res = points.mult(scalar_bytes.as_slice(), 255);
        Ok(Self(res))
    }
}

// Bound the lifetime of points to the output slice.
fn to_blst_type_slice<From, To>(points: &[From]) -> &[To] {
    // SAFETY: the cast from `&[G1Element]` to `&[blst_p1]` is safe because
    // G1Element is a transparent wrapper around blst_p1. The lifetime of
    // output slice is the same as the input slice.
    unsafe { std::slice::from_raw_parts(points.as_ptr() as *const To, points.len()) }
}

impl GroupElement for G1Element {
    type ScalarType = Scalar;

    fn zero() -> Self {
        Self(blst_p1::default())
    }

    fn generator() -> Self {
        let mut ret = blst_p1::default();
        unsafe {
            blst_p1_from_affine(&mut ret, &BLS12_381_G1);
        }
        Self(ret)
    }
}

impl Pairing for G1Element {
    type Other = G2Element;
    type Output = GTElement;

    fn pairing(&self, other: &Self::Other) -> <Self as Pairing>::Output {
        let mut self_affine = blst_p1_affine::default();
        let mut other_affine = blst_p2_affine::default();
        let mut res = blst_fp12::default();
        unsafe {
            blst_p1_to_affine(&mut self_affine, &self.0);
            blst_p2_to_affine(&mut other_affine, &other.0);
            blst_miller_loop(&mut res, &other_affine, &self_affine);
            blst_final_exp(&mut res, &res);
        }
        GTElement(res)
    }
}

impl HashToGroupElement for G1Element {
    fn hash_to_group_element(msg: &[u8]) -> Self {
        let mut res = blst_p1::default();
        unsafe {
            blst_hash_to_g1(
                &mut res,
                msg.as_ptr(),
                msg.len(),
                DST_G1.as_ptr(),
                DST_G1.len(),
                ptr::null(),
                0,
            );
        }
        Self(res)
    }
}

impl FromTrustedByteArray<G1_ELEMENT_BYTE_LENGTH> for G1Element {
    fn from_trusted_byte_array(bytes: &[u8; G1_ELEMENT_BYTE_LENGTH]) -> FastCryptoResult<Self> {
        let mut ret = blst_p1::default();
        unsafe {
            let mut affine = blst_p1_affine::default();
            if blst_p1_uncompress(&mut affine, bytes.as_ptr()) != BLST_ERROR::BLST_SUCCESS {
                return Err(FastCryptoError::InvalidInput);
            }
            blst_p1_from_affine(&mut ret, &affine);
        }
        Ok(G1Element(ret))
    }
}

impl ToFromByteArray<G1_ELEMENT_BYTE_LENGTH> for G1Element {
    fn from_byte_array(bytes: &[u8; G1_ELEMENT_BYTE_LENGTH]) -> FastCryptoResult<Self> {
        let ret = Self::from_trusted_byte_array(bytes)?;
        unsafe {
            // Verify that the deserialized element is in G1
            if !blst_p1_in_g1(&ret.0) {
                return Err(FastCryptoError::InvalidInput);
            }
        }
        Ok(ret)
    }

    fn to_byte_array(&self) -> [u8; G1_ELEMENT_BYTE_LENGTH] {
        let mut bytes = [0u8; G1_ELEMENT_BYTE_LENGTH];
        unsafe {
            blst_p1_compress(bytes.as_mut_ptr(), &self.0);
        }
        bytes
    }
}

impl Debug for G1Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = Hex::encode(self.to_byte_array());
        write!(f, "{:?}", bytes)
    }
}

serialize_deserialize_with_to_from_byte_array!(G1Element);
generate_bytes_representation!(G1Element, G1_ELEMENT_BYTE_LENGTH, G1ElementAsBytes);

impl Add for G2Element {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = blst_p2::default();
        unsafe {
            blst_p2_add_or_double(&mut ret, &self.0, &rhs.0);
        }
        Self(ret)
    }
}

impl Sub for G2Element {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::add(self, Self::neg(rhs))
    }
}

impl Neg for G2Element {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut ret = self.0;
        unsafe {
            blst_p2_cneg(&mut ret, true);
        }
        Self(ret)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Scalar> for G2Element {
    type Output = FastCryptoResult<Self>;

    fn div(self, rhs: Scalar) -> Self::Output {
        let inv = rhs.inverse()?;
        Ok(self * inv)
    }
}

impl Mul<Scalar> for G2Element {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut scalar: blst_scalar = blst_scalar::default();
        unsafe {
            blst_scalar_from_fr(&mut scalar, &rhs.0);
        }

        // Count the number of bytes to be multiplied.
        let bytes = size_in_bytes(&scalar);

        if bytes == 0 {
            return G2Element::zero();
        }

        // If rhs = 1, return self
        if bytes == 1 && scalar.b[0] == 1 {
            return self;
        }

        let mut result = blst_p2::default();
        unsafe {
            blst_p2_mult(
                &mut result,
                &self.0,
                &(scalar.b[0]),
                size_in_bits(&scalar, bytes),
            );
        }

        Self(result)
    }
}

impl MultiScalarMul for G2Element {
    fn multi_scalar_mul(scalars: &[Self::ScalarType], points: &[Self]) -> FastCryptoResult<Self> {
        if scalars.len() != points.len() || scalars.is_empty() {
            return Err(FastCryptoError::InvalidInput);
        }
        // Unfortunately we copy since blst does not filter out inf
        // https://github.com/supranational/blst/blob/master/src/multi_scalar.c#L11
        let (scalars, points): (Vec<_>, Vec<_>) = scalars
            .iter()
            .zip(points.iter())
            .filter(|(&s, &p)| s != Scalar::zero() && p != Self::zero())
            .map(|(&s, &p)| (s, p))
            .unzip();
        // We already checked that scalars is not empty above so if it's empty here, it means
        // that all the points are zero.
        if scalars.is_empty() {
            return Ok(Self::zero());
        }

        // Inspired by blstrs.
        let points = to_blst_type_slice(&points);
        let points = p2_affines::from(points);
        let mut scalar_bytes: Vec<u8> = Vec::with_capacity(scalars.len() * 32);
        for a in scalars.iter().map(|s| s.0) {
            let mut scalar: blst_scalar = blst_scalar::default();
            unsafe {
                blst_scalar_from_fr(&mut scalar, &a);
            }
            scalar_bytes.extend_from_slice(&scalar.b);
        }
        // The scalar field size is smaller than 2^255, so we need at most 255 bits.
        let res = points.mult(scalar_bytes.as_slice(), 255);
        Ok(Self(res))
    }
}

impl GroupElement for G2Element {
    type ScalarType = Scalar;

    fn zero() -> Self {
        Self(blst_p2::default())
    }

    fn generator() -> Self {
        let mut ret = blst_p2::default();
        unsafe {
            blst_p2_from_affine(&mut ret, &BLS12_381_G2);
        }
        Self(ret)
    }
}

impl HashToGroupElement for G2Element {
    fn hash_to_group_element(msg: &[u8]) -> Self {
        let mut res = blst_p2::default();
        unsafe {
            blst_hash_to_g2(
                &mut res,
                msg.as_ptr(),
                msg.len(),
                DST_G2.as_ptr(),
                DST_G2.len(),
                ptr::null(),
                0,
            );
        }
        Self(res)
    }
}

impl FromTrustedByteArray<G2_ELEMENT_BYTE_LENGTH> for G2Element {
    fn from_trusted_byte_array(bytes: &[u8; G2_ELEMENT_BYTE_LENGTH]) -> FastCryptoResult<Self> {
        let mut ret = blst_p2::default();
        unsafe {
            let mut affine = blst_p2_affine::default();
            if blst_p2_uncompress(&mut affine, bytes.as_ptr()) != BLST_ERROR::BLST_SUCCESS {
                return Err(FastCryptoError::InvalidInput);
            }
            blst_p2_from_affine(&mut ret, &affine);
        }
        Ok(G2Element(ret))
    }
}

impl ToFromByteArray<G2_ELEMENT_BYTE_LENGTH> for G2Element {
    fn from_byte_array(bytes: &[u8; G2_ELEMENT_BYTE_LENGTH]) -> FastCryptoResult<Self> {
        let ret = Self::from_trusted_byte_array(bytes)?;
        unsafe {
            // Verify that the deserialized element is in G2
            if !blst_p2_in_g2(&ret.0) {
                return Err(FastCryptoError::InvalidInput);
            }
        }
        Ok(ret)
    }

    fn to_byte_array(&self) -> [u8; G2_ELEMENT_BYTE_LENGTH] {
        let mut bytes = [0u8; G2_ELEMENT_BYTE_LENGTH];
        unsafe {
            blst_p2_compress(bytes.as_mut_ptr(), &self.0);
        }
        bytes
    }
}

impl Debug for G2Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = Hex::encode(self.to_byte_array());
        write!(f, "{:?}", bytes)
    }
}

serialize_deserialize_with_to_from_byte_array!(G2Element);
generate_bytes_representation!(G2Element, G2_ELEMENT_BYTE_LENGTH, G2ElementAsBytes);

impl Add for GTElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = blst_fp12::default();
        unsafe {
            blst_fp12_mul(&mut ret, &self.0, &rhs.0);
        }
        Self(ret)
    }
}

impl Sub for GTElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::add(self, Self::neg(rhs))
    }
}

impl Neg for GTElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut ret = self.0;
        unsafe {
            blst_fp12_inverse(&mut ret, &self.0);
        }
        Self(ret)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Scalar> for GTElement {
    type Output = FastCryptoResult<Self>;

    fn div(self, rhs: Scalar) -> Self::Output {
        let inv = rhs.inverse()?;
        Ok(self * inv)
    }
}

impl Mul<Scalar> for GTElement {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        if rhs == Scalar::zero() {
            return Self::zero();
        }

        if rhs.0 == BLST_FR_ONE {
            return self;
        }

        let mut y: blst_fp12 = blst_fp12::default();
        let mut n = rhs.0;
        let mut x = self.0;

        // Compute n * x using repeated doubling (~ additive version of exponentiation by repeated squaring)
        unsafe {
            // Keep going while n > 1
            while n != blst_fr::default() && n != BLST_FR_ONE {
                if is_odd(&n) {
                    blst_fr_sub(&mut n, &n, &BLST_FR_ONE);
                    y *= x;
                }
                blst_fp12_sqr(&mut x, &x);
                blst_fr_rshift(&mut n, &n, 1);
            }
            y *= x;
            Self(y)
        }
    }
}

impl GroupElement for GTElement {
    type ScalarType = Scalar;

    fn zero() -> Self {
        unsafe { Self(*blst_fp12_one()) }
    }

    fn generator() -> Self {
        static G: OnceCell<blst_fp12> = OnceCell::new();
        Self(*G.get_or_init(Self::compute_generator))
    }
}

impl GTElement {
    fn compute_generator() -> blst_fp12 {
        // Compute the generator as e(G1, G2).
        let mut res = blst_fp12::default();
        unsafe {
            blst_miller_loop(&mut res, &BLS12_381_G2, &BLS12_381_G1);
            blst_final_exp(&mut res, &res);
        }
        res
    }
}

const P_AS_BYTES: [u8; FP_BYTE_LENGTH] = hex!("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab");

// Note that the serialization below is uncompressed, i.e. it uses 576 bytes.
impl FromTrustedByteArray<GT_ELEMENT_BYTE_LENGTH> for GTElement {
    fn from_trusted_byte_array(bytes: &[u8; GT_ELEMENT_BYTE_LENGTH]) -> FastCryptoResult<Self> {
        // The following is based on the order from
        // https://github.com/supranational/blst/blob/b4ebf88014251f1cfefeb6cf1cd4df7c40dc568f/src/fp12_tower.c#L773-L786C2
        let mut gt: blst_fp12 = Default::default();
        let mut current = 0; // simpler to track
        for i in 0..3 {
            for j in 0..2 {
                for k in 0..2 {
                    let mut fp = blst_fp::default();
                    let slice = &bytes[current..current + FP_BYTE_LENGTH];
                    // We compare with P_AS_BYTES to ensure that we process a canonical representation
                    // which uses mod p elements.
                    if *slice >= P_AS_BYTES[..] {
                        return Err(FastCryptoError::InvalidInput);
                    }
                    unsafe {
                        blst_fp_from_bendian(&mut fp, slice.as_ptr());
                    }
                    gt.fp6[j].fp2[i].fp[k] = fp;
                    current += FP_BYTE_LENGTH;
                }
            }
        }
        Ok(Self(gt))
    }
}

// Note that the serialization below is uncompressed, i.e. it uses 576 bytes.
impl ToFromByteArray<GT_ELEMENT_BYTE_LENGTH> for GTElement {
    fn from_byte_array(bytes: &[u8; GT_ELEMENT_BYTE_LENGTH]) -> FastCryptoResult<Self> {
        let gt = Self::from_trusted_byte_array(bytes)?;
        match gt.0.in_group() {
            true => Ok(gt),
            false => Err(FastCryptoError::InvalidInput),
        }
    }

    fn to_byte_array(&self) -> [u8; GT_ELEMENT_BYTE_LENGTH] {
        self.0.to_bendian()
    }
}

impl Debug for GTElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = Hex::encode(self.to_byte_array());
        write!(f, "{:?}", bytes)
    }
}

serialize_deserialize_with_to_from_byte_array!(GTElement);

impl GroupElement for Scalar {
    type ScalarType = Self;

    fn zero() -> Self {
        Self(blst_fr::default())
    }

    fn generator() -> Self {
        Self(BLST_FR_ONE)
    }
}

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = blst_fr::default();
        unsafe {
            blst_fr_add(&mut ret, &self.0, &rhs.0);
        }
        Self(ret)
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = blst_fr::default();
        unsafe {
            blst_fr_sub(&mut ret, &self.0, &rhs.0);
        }
        Self(ret)
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        let mut ret = blst_fr::default();
        unsafe {
            blst_fr_cneg(&mut ret, &self.0, true);
        }
        Self(ret)
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut ret = blst_fr::default();
        unsafe {
            blst_fr_mul(&mut ret, &self.0, &rhs.0);
        }
        Self(ret)
    }
}

impl From<u128> for Scalar {
    fn from(value: u128) -> Self {
        let mut ret = blst_fr::default();
        let buff = [value as u64, (value >> 64) as u64, 0u64, 0u64];
        unsafe {
            blst_fr_from_uint64(&mut ret, buff.as_ptr());
        }
        Self(ret)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Scalar> for Scalar {
    type Output = FastCryptoResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let inv = rhs.inverse()?;
        Ok(self * inv)
    }
}

impl ScalarType for Scalar {
    fn rand<R: AllowedRng>(rng: &mut R) -> Self {
        let mut buffer = [0u8; 64];
        rng.fill_bytes(&mut buffer);
        reduce_mod_uniform_buffer(&buffer)
    }

    fn inverse(&self) -> FastCryptoResult<Self> {
        if *self == Scalar::zero() {
            return Err(FastCryptoError::InvalidInput);
        }
        let mut ret = blst_fr::default();
        unsafe {
            blst_fr_inverse(&mut ret, &self.0);
        }
        Ok(Self(ret))
    }
}

/// Reduce a big-endian integer of arbitrary size modulo the scalar field size and return the scalar.
/// If the input bytes are uniformly distributed, the output will be uniformly distributed in the
/// scalar field.
///
/// The input buffer must be at least 48 bytes long to ensure that there is only negligible bias in
/// the output.
pub(crate) fn reduce_mod_uniform_buffer(buffer: &[u8]) -> Scalar {
    assert!(buffer.len() >= 48);
    let mut ret = blst_fr::default();
    let mut tmp = blst_scalar::default();
    unsafe {
        blst_scalar_from_be_bytes(&mut tmp, buffer.as_ptr(), buffer.len());
        blst_fr_from_scalar(&mut ret, &tmp);
    }
    Scalar(ret)
}

impl FiatShamirChallenge for Scalar {
    fn fiat_shamir_reduction_to_group_element(uniform_buffer: &[u8]) -> Self {
        reduce_mod_uniform_buffer(uniform_buffer)
    }
}

impl FromTrustedByteArray<SCALAR_LENGTH> for Scalar {
    fn from_trusted_byte_array(bytes: &[u8; SCALAR_LENGTH]) -> FastCryptoResult<Self> {
        let mut ret = blst_fr::default();
        unsafe {
            let mut scalar = blst_scalar::default();
            blst_scalar_from_bendian(&mut scalar, bytes.as_ptr());
            blst_fr_from_scalar(&mut ret, &scalar);
        }
        Ok(Scalar(ret))
    }
}

impl ToFromByteArray<SCALAR_LENGTH> for Scalar {
    fn from_byte_array(bytes: &[u8; SCALAR_LENGTH]) -> FastCryptoResult<Self> {
        let mut ret = blst_fr::default();
        unsafe {
            let mut scalar = blst_scalar::default();
            blst_scalar_from_bendian(&mut scalar, bytes.as_ptr());
            if !blst_scalar_fr_check(&scalar) {
                return Err(FastCryptoError::InvalidInput);
            }
            blst_fr_from_scalar(&mut ret, &scalar);
        }
        Ok(Scalar(ret))
    }

    fn to_byte_array(&self) -> [u8; SCALAR_LENGTH] {
        let mut bytes = [0u8; SCALAR_LENGTH];
        unsafe {
            let mut scalar = blst_scalar::default();
            blst_scalar_from_fr(&mut scalar, &self.0);
            blst_bendian_from_scalar(bytes.as_mut_ptr(), &scalar);
        }
        bytes
    }
}

impl Debug for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = Hex::encode(self.to_byte_array());
        write!(f, "{:?}", bytes)
    }
}

serialize_deserialize_with_to_from_byte_array!(Scalar);

pub(crate) fn is_odd(value: &blst_fr) -> bool {
    let odd: bool;
    unsafe {
        let mut scalar = blst_scalar::default();
        blst_scalar_from_fr(&mut scalar, value);
        let mut bytes = [0u8; 32];
        blst_lendian_from_scalar(bytes.as_mut_ptr(), &scalar);
        odd = bytes[0] % 2 == 1;
    }
    odd
}

/// This represents the multiplicative unit scalar
const BLST_FR_ONE: blst_fr = blst_fr {
    l: [
        8589934590,
        6378425256633387010,
        11064306276430008309,
        1739710354780652911,
    ],
};
