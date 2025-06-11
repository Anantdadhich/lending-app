use anchor_lang::prelude::*; 

#[account] 
#[derive(InitSpace)]
pub struct  User{
   pub owner:Pubkey 

}
