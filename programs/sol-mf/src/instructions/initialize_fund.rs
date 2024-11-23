use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
#[instruction(fund_name: String)]
pub struct InitializeFund<'info> {
    #[account(mut)]
    pub manager: Signer<'info>,
    #[account(
        init,
        payer = manager,
        space = 8 + Fund::INIT_SPACE,
        seeds = [b"fund".as_ref(), manager.key().as_ref(), fund_name.as_ref()],
        bump,
    )]
    pub fund: Account<'info, Fund>,
    pub treasury_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = manager,
        token::mint = treasury_mint,
        token::authority = fund,
        seeds = [b"fund_treasury", fund.key().as_ref()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<InitializeFund>, fund_name: String, fee_rate_bips: u16) -> Result<()> {
    *ctx.accounts.fund = Fund {
        manager: ctx.accounts.manager.key(),
        name: fund_name,
        treasury_mint: ctx.accounts.treasury_mint.key(),
        treasury_token_account: ctx.accounts.treasury_token_account.key(),
        num_holdings: 0,
        total_deposits: 0,
        total_deposit_shares: 0,
        fee_rate_bips: fee_rate_bips,
        last_updated: Clock::get()?.unix_timestamp,
        bump: ctx.bumps.fund,
        treasury_bump: ctx.bumps.treasury_token_account,
    };

    Ok(())
}
