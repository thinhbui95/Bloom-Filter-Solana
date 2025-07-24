pub mod error;
pub mod context;
pub mod constants;
pub mod state;
pub mod utils;
use anchor_lang::prelude::*;
use crate::error::ErrorCode;
pub use crate::context::*;
pub use crate::constants::*;
pub use crate::state::*;
pub use crate::utils::*;

declare_id!("F9P4gcXZKQZRSopRDktRZysjNujwygz6k7Qg9JBnHcXi");

// Program definition
#[program]
pub mod airdrop_bloom_filter {
    use super::*;

    // Initialize Bloom filter and disputed claims PDA
    #[access_control(verify_root(*ctx.accounts.authority.key))]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        *ctx.accounts.bloom_filter = BloomFilterDataAccount {
            data: BloomFilterData::new(),
            bump: ctx.bumps.bloom_filter,
        };
        msg!("Initialized Bloom filter ({} bits)", BIT_ARRAY_SIZE);
        Ok(())
    }

    // Claim airdrop
    pub fn claim_airdrop(ctx: Context<ClaimAirdrop>) -> Result<()> {
        let bloom_filter = &mut ctx.accounts.bloom_filter.data;
        let user = ctx.accounts.user.key();

        // Check Bloom filter
        if might_contain(&user, &bloom_filter) { // May be a false positive
            return Err(ErrorCode::AlreadyClaimed.into());
        }

        // Add user to Bloom filter
        add_to_bloom_filter(&user, bloom_filter);

        // Transfer tokens (SPL Token program)
        utils::transfer_token(
            &ctx.accounts.program_authority.to_account_info(),
            &ctx.accounts.program_authority_token_account.to_account_info(),
            &ctx.accounts.user_token_account.to_account_info(),
            AIRDROP_AMOUNT,
            &[&[ROOT_AUTHORITY_SEED, &[ctx.bumps.program_authority]]],
        )?;

        msg!("Airdrop claimed for user: {} ({} lamports)", user, AIRDROP_AMOUNT);
        Ok(())
    }
}
