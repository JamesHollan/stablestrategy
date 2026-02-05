use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("StaB1eStrat3gy11111111111111111111111111111");

#[program]
pub mod stable_strategy {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        distribution_mode: DistributionMode,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.project_mint = ctx.accounts.project_mint.key();
        config.stable_mint = ctx.accounts.stable_mint.key();
        config.distribution_mode = distribution_mode;
        Ok(())
    }

    pub fn distribute(
        ctx: Context<Distribute>,
        recipients: Vec<RecipientInfo>,
        total_amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;

        require_keys_eq!(config.authority, ctx.accounts.authority.key(), StableError::Unauthorized);

        let vault_balance = ctx.accounts.stable_vault.amount;
        require!(vault_balance >= total_amount, StableError::InsufficientFunds);

        let per_recipient_amount = match config.distribution_mode {
            DistributionMode::Equal => {
                require!(recipients.len() > 0, StableError::InvalidRecipients);
                total_amount
                    .checked_div(recipients.len() as u64)
                    .ok_or(StableError::MathError)?
            }
            DistributionMode::Proportional => {
                // In a real implementation, you would:
                // 1. Sum all "weight" fields.
                // 2. Compute each share as total_amount * weight / total_weight.
                // Here we keep it simple and just require off-chain pre-calculation.
                0
            }
        };

        for r in recipients.iter() {
            // In a real implementation, you would verify that:
            // - r.stable_token_account is a valid token account for stable_mint.
            // - The holder actually is in the top 250 holders (verified off-chain + on-chain if needed).

            let amount_to_send = match config.distribution_mode {
                DistributionMode::Equal => per_recipient_amount,
                DistributionMode::Proportional => r.amount, // precomputed off-chain
            };

            if amount_to_send == 0 {
                continue;
            }

            let cpi_accounts = Transfer {
                from: ctx.accounts.stable_vault.to_account_info(),
                to: r.stable_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            };

            let seeds: &[&[u8]] = &[
                b"vault_authority",
                config.key().as_ref(),
                &[ctx.bumps["vault_authority"]],
            ];
            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );

            token::transfer(cpi_ctx, amount_to_send)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + Config::LEN)]
    pub config: Account<'info, Config>,

    pub project_mint: Account<'info, Mint>,
    pub stable_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut, has_one = authority)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        constraint = stable_vault.mint == config.stable_mint
    )]
    pub stable_vault: Account<'info, TokenAccount>,

    /// CHECK: PDA authority for the vault
    #[account(seeds = [b"vault_authority", config.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub project_mint: Pubkey,
    pub stable_mint: Pubkey,
    pub distribution_mode: DistributionMode,
}

impl Config {
    pub const LEN: usize = 32 + 32 + 32 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DistributionMode {
    Equal,
    Proportional,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecipientInfo {
    pub stable_token_account: Pubkey,
    pub amount: u64, // used in proportional mode
}

#[error_code]
pub enum StableError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Invalid recipients")]
    InvalidRecipients,
    #[msg("Math error")]
    MathError,
}
