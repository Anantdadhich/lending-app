use anchor_lang::prelude::*;
use instructions::*;  

mod state;
mod instructions;



declare_id!("BrrQ3W1GLKS7RrL7wpP9UX39en6zkXkvZKEz8ouS8L2w");

#[program]
pub mod lending {


  
   

    use super::*;

    pub fn init_bank(ctx:Context<InitBank>,liquidation_threshold:u64,max_ltv:u64)->Result<()> {

           process_init_bank(ctx, liquidation_threshold, max_ltv);
        Ok(()) 
    }

    pub fn init_user(ctx:Context<InitUser>,usdc_address:Pubkey)->Result<()> {
         process_init_user(ctx, usdc_address);
        Ok(())
    }

    
           

     
}


