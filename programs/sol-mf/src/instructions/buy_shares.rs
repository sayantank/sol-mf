use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

#[derive(Accounts)]
#[instruction(fund_name: String)]
pub struct BuyShares<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"fund".as_ref(), manager.key().as_ref(), fund_name.as_ref()],
        has_one = manager,
        has_one = treasury_token_account,
        has_one = treasury_mint,
        bump = fund.bump
    )]
    pub fund: Account<'info, Fund>,
    pub manager: AccountInfo<'info>,
    pub treasury_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        token::mint = treasury_mint,
        token::authority = fund,
        seeds = [b"fund_treasury", fund.key().as_ref()],
        bump = fund.treasury_bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = treasury_mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + Shareholder::INIT_SPACE,
        seeds = [b"shareholder".as_ref(), buyer.key().as_ref(), fund.key().as_ref()],
        bump,
        has_one = fund
    )]
    pub shareholder_account: Account<'info, Shareholder>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<BuyShares>, _fund_name: String, deposit_amount: u64) -> Result<()> {
    // Transfer funds from buyer to treasury
    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.buyer_token_account.to_account_info(),
        mint: ctx.accounts.treasury_mint.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
    let decimals = ctx.accounts.treasury_mint.decimals;

    token_interface::transfer_checked(cpi_ctx, deposit_amount, decimals)?;

    // Calculate shares
    let fund = &mut ctx.accounts.fund;

    let total_deposits = if fund.total_deposits == 0 {
        deposit_amount
    } else {
        fund.total_deposits
    };
    let total_deposit_shares = if fund.total_deposit_shares == 0 {
        deposit_amount
    } else {
        fund.total_deposit_shares
    };

    let deposit_ratio = deposit_amount.checked_div(total_deposits).unwrap();
    let buyer_shares = deposit_ratio.checked_mul(total_deposit_shares).unwrap();

    // Update fund state
    fund.total_deposits = deposit_amount;
    fund.total_deposit_shares = buyer_shares;

    // Update shareholder state
    let shareholder = &mut ctx.accounts.shareholder_account;
    shareholder.shares += buyer_shares;
    shareholder.owner = ctx.accounts.buyer.key();
    shareholder.fund = ctx.accounts.fund.key();
    shareholder.bump = ctx.bumps.shareholder_account;

    Ok(())
}
