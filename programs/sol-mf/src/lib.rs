use anchor_lang::prelude::*;
use instructions::*;

mod constants;
mod error;
mod instructions;
mod state;

declare_id!("DEbamqaZ68gXpBMDwaFBDQ4TM7giTt3rJZ387FmZnsCM");

#[program]
pub mod sol_mf {
    use super::*;

    pub fn initialize_fund(
        ctx: Context<InitializeFund>,
        fund_name: String,
        fee_rate_bips: u16,
    ) -> Result<()> {
        initialize_fund::handler(ctx, fund_name, fee_rate_bips)
    }

    pub fn buy_shares(
        ctx: Context<BuyShares>,
        fund_name: String,
        deposit_amount: u64,
    ) -> Result<()> {
        buy_shares::handler(ctx, fund_name, deposit_amount)
    }
}
