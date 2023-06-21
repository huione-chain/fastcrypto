// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains an implementation of a Merkle Tree data structure (Merkle, R.C. (1988): A Digital Signature
//! Based on a Conventional Encryption Function) which is a data structure that allows an arbitrary
//! number of elements of a given type `T` to be added as leaves to the tree and we can then construct
//! proofs logarithmic in the number of leaves that a certain leaf has a given value. Such proofs can
//! be verified by a small verifier which only needs to know the root of the tree.
//!
//! # Example
//! ```rust
//! # use fastcrypto::hash::Sha256;
//! # use fastcrypto_data::merkle_tree::*;
//! let elements = [[1u8], [2u8], [3u8]];
//! let mut tree = MerkleTree::<32, Sha256, [u8; 1]>::new();
//! tree.insert_elements(elements.iter());
//!
//! let index = 1;
//! let proof = tree.prove(index);
//!
//! let verifier = tree.create_verifier().unwrap();
//! assert!(verifier.verify_with_element(index, &elements[index], &proof));
//! ```

use std::borrow::Borrow;
use std::marker::PhantomData;

use fastcrypto::error::FastCryptoError;
use fastcrypto::hash::HashFunction;
use rs_merkle::{Hasher, MerkleProof, MerkleTree as ExternalMerkleTree};

/// This represents a Merkle Tree with an arbitrary number of elements of type `T`. The [prove] function
/// can generate proofs that the leaf of a given index has a certain hash value.
///
/// New elements may be added continuously but once a verifier is generated with the [create_verifier]
/// function, the proofs are only valid for the state of the tree at that point.
///
/// To avoid second-preimage attacks, a 0x00 byte is prepended to the hash data for leaf nodes (see
/// [LeafHasher]), and 0x01 is prepended when computing internal node hashes (see [InternalNodeHasher]).
pub struct MerkleTree<const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>, T: AsRef<[u8]>> {
    tree: ExternalMerkleTree<InternalNodeHasher<DIGEST_LENGTH, H>>,
    _type: PhantomData<T>,
}

impl<const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>, T: AsRef<[u8]>>
    MerkleTree<DIGEST_LENGTH, H, T>
{
    pub fn new() -> Self {
        MerkleTree {
            tree: ExternalMerkleTree::new(),
            _type: PhantomData::default(),
        }
    }
}

impl<const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>, T: AsRef<[u8]>> Default
    for MerkleTree<DIGEST_LENGTH, H, T>
{
    fn default() -> Self {
        Self::new()
    }
}

/// This verifier can verify proofs generated by [MerkleTree::prove].
pub struct MerkleTreeVerifier<
    const DIGEST_LENGTH: usize,
    H: HashFunction<DIGEST_LENGTH>,
    T: AsRef<[u8]>,
> {
    root: [u8; DIGEST_LENGTH],
    number_of_leaves: usize,
    _hash_function: PhantomData<H>,
    _type: PhantomData<T>,
}

impl<const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>, T: AsRef<[u8]>>
    MerkleTreeVerifier<DIGEST_LENGTH, H, T>
{
    /// Verify a [Proof] that an element with the given hash was at this index of this tree at the time
    /// this verifier was created.
    pub fn verify_with_hash(
        &self,
        index: usize,
        leaf_hash: [u8; DIGEST_LENGTH],
        proof: &Proof<DIGEST_LENGTH, H, T>,
    ) -> bool {
        proof
            .proof
            .verify(self.root, &[index], &[leaf_hash], self.number_of_leaves)
    }

    /// Verify a [Proof] that an element was at this index of this tree at the time this verifier was
    /// created.
    pub fn verify_with_element(
        &self,
        index: usize,
        element: &T,
        proof: &Proof<DIGEST_LENGTH, H, T>,
    ) -> bool {
        self.verify_with_hash(
            index,
            LeafHasher::<DIGEST_LENGTH, H>::hash(element.as_ref()),
            proof,
        )
    }
}

impl<const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>, T: AsRef<[u8]>>
    MerkleTree<DIGEST_LENGTH, H, T>
{
    /// Hash an element using the hash function used for this tree.
    pub fn hash(element: &T) -> [u8; DIGEST_LENGTH] {
        LeafHasher::<DIGEST_LENGTH, H>::hash(element.as_ref())
    }

    /// Insert element in this tree and return the index of the newly inserted element.
    fn insert_hash(&mut self, hash: [u8; DIGEST_LENGTH]) -> usize {
        self.tree.insert(hash).commit();
        self.tree.leaves_len() - 1
    }

    /// Insert all elements in the iterator into this tree. The elements are given consecutive indices
    /// and the return value is the index of the last element.
    fn insert_hashes(&mut self, hashes: impl Iterator<Item = [u8; DIGEST_LENGTH]>) -> usize {
        for hash in hashes {
            self.tree.insert(hash);
        }
        self.tree.commit();
        self.tree.leaves_len() - 1
    }

    /// Insert element in this tree and return the index of the newly inserted element.
    pub fn insert_element(&mut self, element: &T) -> usize {
        self.insert_hash(Self::hash(element))
    }

    /// Insert all elements in the iterator into this tree. The elements are given consecutive indices
    /// and the return value is the index of the last element.
    pub fn insert_elements(&mut self, elements: impl Iterator<Item = impl Borrow<T>>) -> usize {
        self.insert_hashes(elements.map(|element| Self::hash(element.borrow())))
    }

    /// Create a proof for the element at the given index.
    pub fn prove(&self, index: usize) -> Proof<DIGEST_LENGTH, H, T> {
        Proof {
            proof: self.tree.proof(&[index]),
            _type: PhantomData::default(),
        }
    }

    /// Create a [MerkleTreeVerifier] for the current state of this tree.
    pub fn create_verifier(
        &self,
    ) -> Result<MerkleTreeVerifier<DIGEST_LENGTH, H, T>, FastCryptoError> {
        Ok(MerkleTreeVerifier {
            root: self
                .tree
                .root()
                .ok_or_else(|| FastCryptoError::GeneralError("Tree is empty".to_string()))?,
            number_of_leaves: self.tree.leaves_len(),
            _hash_function: PhantomData::default(),
            _type: PhantomData::default(),
        })
    }

    /// Return the number of leaves in this tree.
    pub fn number_of_leaves(&self) -> usize {
        self.tree.leaves_len()
    }

    /// Returns the root of this tree.
    pub fn root(&self) -> Result<[u8; DIGEST_LENGTH], FastCryptoError> {
        self.tree
            .root()
            .ok_or_else(|| FastCryptoError::GeneralError("Tree is empty".to_string()))
    }
}

/// A proof that a leaf with a given index in a Merkle Tree has a certain hash value.
pub struct Proof<const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>, T: AsRef<[u8]>> {
    proof: MerkleProof<InternalNodeHasher<DIGEST_LENGTH, H>>,
    _type: PhantomData<T>,
}

/// A hash function which given input `X` computes `H(PREFIX ||X)`
struct PrefixedHasher<const PREFIX: u8, const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>>
{
    _hasher: PhantomData<H>,
}

impl<const PREFIX: u8, const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>> Clone
    for PrefixedHasher<PREFIX, DIGEST_LENGTH, H>
{
    fn clone(&self) -> Self {
        Self {
            _hasher: PhantomData::default(),
        }
    }
}

impl<const PREFIX: u8, const DIGEST_LENGTH: usize, H: HashFunction<DIGEST_LENGTH>> Hasher
    for PrefixedHasher<PREFIX, DIGEST_LENGTH, H>
{
    type Hash = [u8; DIGEST_LENGTH];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut input = vec![];
        input.push(PREFIX);
        input.extend_from_slice(data);
        H::digest(input).digest
    }
}

/// Computes H(0x01 || X)
type InternalNodeHasher<const DIGEST_LENGTH: usize, H> = PrefixedHasher<0x01, DIGEST_LENGTH, H>;

/// Computes H(0x00 || X)
type LeafHasher<const DIGEST_LENGTH: usize, H> = PrefixedHasher<0x00, DIGEST_LENGTH, H>;

#[cfg(test)]
mod tests {
    use fastcrypto::hash::Sha256;

    use crate::merkle_tree::MerkleTree;

    #[test]
    fn test_merkle_tree() {
        let mut tree = MerkleTree::<32, Sha256, Vec<u8>>::new();

        // An empty tree does not have a root
        assert!(tree.root().is_err());
        assert!(tree.create_verifier().is_err());

        let elements = [vec![1u8], vec![2u8], vec![3u8]];
        let index = 1;
        let element = &elements[index];

        // Adding elements should change the number of leaves
        assert_eq!(0, tree.number_of_leaves());
        assert_eq!(elements.len() - 1, tree.insert_elements(elements.iter()));
        assert_eq!(elements.len(), tree.number_of_leaves());

        // Generate proof for a given element and verify
        let proof = tree.prove(index);
        let verifier = tree.create_verifier().unwrap();
        assert!(verifier.verify_with_element(index, element, &proof));
        assert!(!verifier.verify_with_element(index, &elements[index - 1], &proof));

        // Adding another element changes the root and the old proof should no longer verify
        let root = tree.root().unwrap();
        tree.insert_element(&vec![4u8]);
        let new_root = tree.root().unwrap();
        assert_ne!(root, new_root);
        let new_verifier = tree.create_verifier().unwrap();
        assert!(!new_verifier.verify_with_element(index, element, &proof));
    }
}
