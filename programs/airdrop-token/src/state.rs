use anchor_lang::prelude::*;
pub use crate::constants::*;

// Account structures
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct BloomFilterData {
    pub bit_array: Vec<u8>
}


impl BloomFilterData {
    pub const SIZE: usize = BIT_ARRAY_SIZE / 8; // Size in bytes
    pub fn new() -> Self {
        Self {
            bit_array: vec![0u8; Self::SIZE], // Initialize with zeros
        }
    }
}

#[account]
pub struct BloomFilterDataAccount {
    pub data: BloomFilterData,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ClaimEntry {
    pub key: Pubkey,
    pub claimed: bool,
}

#[account]
pub struct DisputedClaimsAccount {
    pub claims: Vec<ClaimEntry>,
}
