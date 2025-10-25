use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );
        
        let transfer_accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts
        );
        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
    let vault_balance = ctx.accounts.vault.to_account_info().lamports();
    
    require!(amount > 0, VaultError::InvalidAmount);
    require!(vault_balance >= amount, VaultError::InsufficientFunds);

    let bump = ctx.bumps.vault;
    let signer_seeds: &[&[&[u8]]] = &[
        &[
            b"vault",
            ctx.accounts.signer.key.as_ref(),
            &[bump],
        ],
    ];

    let transfer_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.signer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        transfer_accounts,
        signer_seeds,
    );

    transfer(cpi_ctx, amount)?;
    Ok(())
}

}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe because we validate the PDA with seeds
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid vault account")]
    InvalidVault,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}



// ERROR: Custom program error: 0x1004

// PROGRAM LOGS:
//  22222222222222222222222222222222222222222222 invoke [1]
//  log: AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id.
//  22222222222222222222222222222222222222222222 consumed 2461 of 1400000 compute units
//  22222222222222222222222222222222222222222222 failed: custom program error: 0x1004