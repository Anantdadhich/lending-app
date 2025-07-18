use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };

use crate::state::{Bank, User};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Repay<'info>{
  #[account(mut)]
  pub signer:Signer<'info>,

  pub mint:InterfaceAccount<'info,Mint>,

   #[account(
    mut,
    seeds=[mint.key().as_ref()],
    bump,
   )]
  pub bank_account:Account<'info,Bank>,

  #[account(
     mut,
     seeds=[b"treasury",mint.key().as_ref()],
     bump
  )]

  pub bank_token_account:InterfaceAccount<'info,TokenAccount>,

  #[account(
    mut,
    seeds=[signer.key().as_ref()],
    bump
  )]
  pub user:Account<'info,User> ,
  #[account(
    init_if_needed,
    payer=signer,
    associated_token::mint=mint,
    associated_token::authority=signer,
    associated_token::token_program=token_program
  )]

  pub user_token_account:InterfaceAccount<'info,TokenAccount>, 
  
    pub token_program:Interface<'info,TokenInterface> ,
     pub associated_token_program:Program<'info,AssociatedToken> ,
      pub system_program:Program<'info,System>   
}

pub fn process_repay(ctx:Context<Repay>,amount:u64)->Result<()>{
     let user=&mut ctx.accounts.user; 
     let bank=&mut ctx.accounts.bank_account;

        let borrow_asset;

        match ctx.accounts.mint.to_account_info().key(){
            key if key == user.usdc_address =>{
                borrow_asset=user.borrowed_usdc;
            }
            ,
            _ => {
                borrow_asset=user.borrowed_sol;
            }
        }

        //check if user try to repaqy more than borrow 

         if borrow_asset < amount {
            return Err(ErrorCode::OverRepaythenBorrowAmount.into());
         }

         // transdfer cpi 
         let transfer_cpi_accounts=TransferChecked {
           from:ctx.accounts.user_token_account.to_account_info(),
           mint:ctx.accounts.mint.to_account_info(),
           to:ctx.accounts.bank_token_account.to_account_info(),
           authority:ctx.accounts.signer.to_account_info(),
         };
        

        let cpi_programs=ctx.accounts.token_program.to_account_info();
          
        let mint=ctx.accounts.mint.key();

            let signer_keys:&[&[&[u8]]]=&[
              &[
                b"treasury",
                mint.as_ref(),
                &[ctx.bumps.bank_token_account],
              ]
             ];

        let cpi_ctx=CpiContext::new(cpi_programs,transfer_cpi_accounts).with_signer(signer_keys);

        let decimals=ctx.accounts.mint.decimals;

        token_interface::transfer_checked(cpi_ctx, amount, decimals)?;
        
        let borrowed_ratio=amount.checked_div(borrow_asset).unwrap();
        let user_shares=bank.total_borrowed_shares.checked_mul(borrowed_ratio).unwrap(); 

         match  ctx.accounts.mint.to_account_info().key(){
            key if key ==user.usdc_address => {
                user.borrowed_usdc -=amount; 
             user.borrowed_usdc_shares -=user_shares;
            },
            _ =>{
                user.borrowed_sol -=amount;
                user.borrowed_sol_shares -=user_shares; 
            }
         } 

         bank.total_borrowed -= amount ;
         bank.total_borrowed_shares -=user_shares; 


    Ok(())
}