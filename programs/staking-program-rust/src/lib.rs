use anchor_lang::prelude::*;

declare_id!("54mVfvhQfMKoSDbiJ9Kcoqeh1V1qD3KH4nv6PFFFBgkE");

#[program]
pub mod staking_program_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
