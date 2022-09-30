use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sube_crypto {
    use super::*;

    pub fn initialize_bus_line(
        ctx: Context<InitializeAdminAccount>,
        0to3km: u64,
        3to6km: u64,
        6to12km: u64,
        12to27km: u64,
        more27km: u64
    ) -> Result<()> {
        let (services_pda, bump): (Pubkey, u8) = Pubkey::find_program_address(&[ctx.accounts.user.key().as_ref()], &Pubkey::from_str("Ca8tecWTapYzeGfa8FvAMSo6JCheTRPvQhsjebZm56YE").unwrap());
        let sube: &mut Account<SoLotery> = &mut ctx.accounts.sube;
        sube.authority = ctx.accounts.user.key();
        sube.bump_original = bump;
        sube.prices = [0to3km, 3to6km, 6to12km, 12to27km, more27km];
        Ok(())
    }
    pub fn take_a_trip(
        ctx: Context<Trip>,
        km: u8,
    ) -> Result<()> {
        require!(km < 6,  Err(ErrorCode::OverdueCredits.into()));

        let _lamports: u64 = 1000000000;
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        let transfer = system_instruction::transfer(
            &from_pubkey, &to_pubkey, ctx.accounts.sube.prices[km],
        );
        if km == 0 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 1 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 5 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 6 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        if km == 7 {
            anchor_lang::solana_program::program::invoke(
                &transfer,
                &[from.to_account_info(), to.to_account_info().clone()],
            ).expect("Error");
            msg!("Transfered Lamports");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdminAccount<'info> {
    #[account(init, seeds = [user.key().as_ref()], bump, payer = user, space = 85)]
    pub sube: Account<'info, SubeAdminAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Trip<'info> {
    #[account(mut, seeds = [sube.authority.key().as_ref()], bump = sube.bump_original)]
    pub sube: Account<'info, SubeAdminAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct SubeAdminAccount {
    pub authority: Pubkey, 
    pub bump_original: u8,
    pub prices: Vec<u64>
}
#[error_code]
pub enum ErrorCode {
    #[msg("Enter a value corresponding to your route")]InvalidaKilometer
}