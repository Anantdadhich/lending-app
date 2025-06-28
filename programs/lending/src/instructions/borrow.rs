use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, TokenAccount}, token_interface::TokenInterface};
use crate::state::*; 




#[derive(Accounts)]
pub struct Borrow<'info>{
    #[account(mut)] 
    pub signer:Signer<'info>,

    pub mint:InterfaceAccount<'info,Mint>,


      #[account(
        mut,
        seeds=[mint.key().as_ref()],
        bump 
     )]
    pub bank:Account<'info,Bank>, 

    #[account(
      mut,
      seeds=[b"treasury",mint.key().as_ref()],
      bump
    )]

    pub bank_token_account:Interface<'info,TokenAccount> ,

    #[account(
        mut,
       
        seeds=[signer.key().as_ref()],
        bump
    )]
    pub user_account:Account<'info,User> ,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub user_token_account:Interface<'info,TokenAccount>, 

   
    
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
   
    pub system_program:Program<'info,System>,


}
