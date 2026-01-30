#![allow(dead_code)]

use pinocchio::Address;

use crate::{helpers::Transmutable, types::U64Bytes};

#[repr(C)]
pub struct Escrow {
    /// Random number to allow multiple escrows through PDAs
    seed: U64Bytes,
    /// Creator of the escrow
    pub maker: Address,
    /// Token address of deposit token
    pub mint_a: Address,
    /// Token address of receiving token
    pub mint_b: Address,
    /// Amount of token B requested
    pub receive: U64Bytes,
    /// PDA bump
    pub bump: u8,
}

// SAFETY: Is only represented by bytes and byte arrays.
unsafe impl Transmutable for Escrow {
    const LEN: usize = size_of::<Self>();
}

impl Escrow {
    pub fn seed(&self) -> u64 {
        u64::from_le_bytes(self.seed)
    }
}
