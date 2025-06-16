use anchor_lang::prelude::*;
use instructions::*;  

mod state;
mod instructions;



declare_id!("BrrQ3W1GLKS7RrL7wpP9UX39en6zkXkvZKEz8ouS8L2w");

#[program]
pub mod lending {


  
    use std::process;

    use super::*;

    pub fn init_bank(ctx:Context<InitBank>,liquidation_threshold:u64,max_ltv:u64)->Result<()> {
         
        Ok(()) 
    }

    pub fn init_user(ctx:Context<InitUser>,usdc_address:Pubkey)->Result<()> {
                
        Ok(())
    }
           

     
}


