use anchor_lang::prelude::*;
pub const  BIT_ARRAY_SIZE: usize = 24000; // 24000 bits = 3000 bytes
pub const  HASH_COUNT: usize = 16; // 16 hash functions for ~0.0083% false positive rate
pub const  AIRDROP_AMOUNT: u64 = 100_000; // 0.1 tokens in lamports

#[cfg(all(not(feature = "mainnet"), not(feature = "devnet")))]
pub const ROOT_KEYS: &[&str] = &[
  "CqtX4R1iqGfsev3BA4akeJsxgkPpadAU8nPerXvT1UEw",
  "5v7kpP9WFZTvjs4bEn1fKcQ944hrfmXxSySxXVf4sRCJ"
];

pub const ROOT_AUTHORITY_SEED: &[u8] = b"root-authority";
pub const BLOOM_FILTER: &[u8] = b"bloom-filter";
pub const TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169]);
