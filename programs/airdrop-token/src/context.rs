use anchor_lang::prelude::*;
use crate::id;
use crate::constants::{BLOOM_FILTER, ROOT_AUTHORITY_SEED};
use crate::state::*;
use crate::utils::{ get_authority_account, get_associated_token_address };

// Instruction contexts
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [BLOOM_FILTER],
        space = std::mem::size_of::<BloomFilterDataAccount>() + 8 + BloomFilterData::SIZE,
        bump,
    )]

    pub bloom_filter: Account<'info, BloomFilterDataAccount>,

    /// CHECK: program owner, verified using #access_control
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimAirdrop<'info> {
    #[account(
        mut,
        seeds = [BLOOM_FILTER],
        bump,
    )]
    pub bloom_filter: Account<'info, BloomFilterDataAccount>,

    /// CHECK: public user
    #[account(signer, mut)]
    pub user: AccountInfo<'info>,

    /// CHECK: Check by address
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: Checked by address
    #[account()]
    pub token_program: AccountInfo<'info>,

    /// CHECK: Checked by address
    #[account(
        mut,
        address = get_authority_account(&id()).0,
        seeds = [ROOT_AUTHORITY_SEED],
        bump,
    )]
    pub program_authority: AccountInfo<'info>,
    /// CHECK: Checked by address
    #[account(
        mut,
        address = get_associated_token_address(&program_authority.key(), &mint.key(), &token_program.key()),
    )]
    pub program_authority_token_account: AccountInfo<'info>,
    
    /// CHECK: Checked by address
    #[account(
        mut,
        address = get_associated_token_address(&user.key(), &mint.key(), &token_program.key()),
    )]
    pub user_token_account: AccountInfo<'info>,

    /// CHECK: Checked by address
    #[account()]
    pub program: AccountInfo<'info>,
}

