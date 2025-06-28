use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Withdraw<'info>{
     #[account(mut)]
   pub signer:Signer<'info> , 
    
    pub mint :InterfaceAccount<'info,Mint> ,
    #[account(
        mut,
        seeds=[mint.key().as_ref()] ,
        bump
    )]
    pub bank:Account<'info,Bank> ,
     
     #[account(
        mut,
         seeds=[b"treasury",mint.key().as_ref()],
        bump
     )]
     pub bank_token_account:InterfaceAccount<'info,TokenAccount> ,

     #[account(
        mut,
        seeds=[signer.key().as_ref()] ,
        bump
     )] 
     pub user_account:Account<'info,User>,
     //where we are going to withdrawing the asset 
       #[account(
        init_if_needed,   //if ata does not exist we create is using the ass_token_program 
        payer=signer,      //the user wallet will pay the rent for the account  
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program,
       )]
      pub user_token_account:InterfaceAccount<'info,TokenAccount> ,

      pub token_program:Interface<'info,TokenInterface> ,
     pub associated_token_program:Program<'info,AssociatedToken> ,
      pub system_program:Program<'info,System>    

}


pub fn process_withdraw(ctx:Context<Withdraw>,amount:u64)->Result<()> {

      let user=&mut ctx.accounts.user_account; 

      let deposited_value; 
      //we have to figure out what should token we have to withdraw 
      if ctx.accounts.mint.to_account_info().key() == user.usdc_address {
           deposited_value=user.deposited_usdc;
      }else {
        deposited_value=user.deposited_sol;
      }
     
      //check if user try to withdraw more than have 

      if amount > deposited_value {
      return Err(ErrorCode::InsufficientFunds.into());
      } 

      let transfer_cpi_acconts=TransferChecked{
        from:ctx.accounts.bank_token_account.to_account_info(),
        mint:ctx.accounts.mint.to_account_info(),
        to:ctx.accounts.user_token_account.to_account_info(),
        authority:ctx.accounts.bank_token_account.to_account_info()
      };



    Ok(())
}