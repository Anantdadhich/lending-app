use anchor_lang::prelude::*;

declare_id!("BrrQ3W1GLKS7RrL7wpP9UX39en6zkXkvZKEz8ouS8L2w");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
