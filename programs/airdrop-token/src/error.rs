use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
  #[msg("Airdrop: Not an owner.")]
  InvalidOwner,
  #[msg("Airdrop: User may have already claimed.")]
  AlreadyClaimed,
  #[msg("Airdrop: Disputed claims PDA full.")]
  DisputedClaimsFull,
  #[msg("Airdrop: Not authorized.")]
  NotAuthorized,
}