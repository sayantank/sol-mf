use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Fund {
    pub manager: Pubkey,
    #[max_len(50)]
    pub name: String,
    pub treasury_mint: Pubkey,
    pub treasury_token_account: Pubkey,
    pub num_holdings: u16,
    pub total_deposits: u64,
    pub total_deposit_shares: u64,
    pub fee_rate_bips: u16,
    pub last_updated: i64,
    pub bump: u8,
    pub treasury_bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Shareholder {
    pub owner: Pubkey,
    pub fund: Pubkey,
    pub shares: u64,
    pub bump: u8,
}
