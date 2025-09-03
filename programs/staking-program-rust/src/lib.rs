use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Mint, Token, TokenAccount}
};

declare_id!("54mVfvhQfMKoSDbiJ9Kcoqeh1V1qD3KH4nv6PFFFBgkE");

pub mod constant {
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
    pub const TOKEN_SEED: &[u8] = b"token";
}

#[program]
pub mod staking_program_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn stake(ctx: Context<Initialize>,  amount:  u64) -> Result<()> {
       
        Ok(())
    }

    pub fn destake(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds =[constant::VAULT_SEED, mint.key().as_ref(), b"account"],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = token_vault_account,
    )]

    pub token_vault_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}



