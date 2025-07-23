use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use sha2::{Digest, Sha256};
use crate::constants::{ TOKEN_PROGRAM_ID, BIT_ARRAY_SIZE, HASH_COUNT, ROOT_KEYS, ROOT_AUTHORITY_SEED };
use crate::state::BloomFilterData;
use crate::error::ErrorCode;


#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn transfer_token<'info>(
  owner: &AccountInfo<'info>,
  from_pubkey: &AccountInfo<'info>,
  to_pubkey: &AccountInfo<'info>,
  amount: u64,
  signer_seeds: &[&[&[u8]]]
) -> std::result::Result<(), ProgramError> {
  let data = TransferTokenParams {
    instruction: 3,
    amount,
  };
  let instruction = solana_program::instruction::Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      solana_program::instruction::AccountMeta::new(*from_pubkey.key, false),
      solana_program::instruction::AccountMeta::new(*to_pubkey.key, false),
      solana_program::instruction::AccountMeta::new_readonly(*owner.key, true)
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    solana_program::program::invoke(
      &instruction,
      &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()]
    )
  } else {
    solana_program::program::invoke_signed(
      &instruction,
      &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()],
      &signer_seeds
    )
  }
}


// Helper functions
fn get_hash(user: &Pubkey, seed: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(user.as_ref());
    hasher.update(seed.to_le_bytes());
    let result = hasher.finalize();
    u64::from_le_bytes(result[..8].try_into().unwrap()) % (BIT_ARRAY_SIZE as u64)
}

pub fn add_to_bloom_filter(user: &Pubkey, bloom_filter: &mut BloomFilterData) {
    for i in 0..HASH_COUNT {
        let index = get_hash(user, i as u64) as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;
        bloom_filter.bit_array[byte_index] |= 1 << bit_index;
    }
}

pub fn might_contain(user: &Pubkey, bloom_filter: &BloomFilterData) -> bool {
    for i in 0..HASH_COUNT {
        let index = get_hash(user, i as u64) as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;
        msg!("Checking Bloom filter: byte_index={}, bit_index={}", byte_index, bit_index);
        if (bloom_filter.bit_array[byte_index] & (1 << bit_index)) == 0 {
            return false; // Definitely not claimed
        }
    }
    true // Probably claimed
}

pub fn verify_root(user: Pubkey) -> Result<()> {
  let user_key = user.to_string();
  let result = ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
  if result == None {
    return Err(ErrorCode::InvalidOwner.into());
  }

  Ok(())
}

pub fn get_authority_account(program_address: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(&[ROOT_AUTHORITY_SEED], program_address)
}

pub fn get_associated_token_address(
  wallet: &Pubkey,
  mint: &Pubkey,
  token_program: &Pubkey
) -> Pubkey {
  let ata_program_id: Pubkey = Pubkey::new_from_array([
    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90, 19, 153, 218, 255,
    16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
  ]);
  Pubkey::find_program_address(
    &[&wallet.to_bytes(), &token_program.to_bytes(), &mint.to_bytes()],
    &ata_program_id
  ).0
}