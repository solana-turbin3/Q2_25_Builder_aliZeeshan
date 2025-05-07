use anchor_lang::prelude::*;

mod make;

declare_id!("25YEjHuKdBiC6TPVNq4jnqphrpjTGzJQbeeXsm6mScFA");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
