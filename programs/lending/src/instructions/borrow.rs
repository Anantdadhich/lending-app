
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;

use anchor_spl::token_interface::{  self, Mint, TokenAccount, TokenInterface ,TransferChecked};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::constants::{MAX_AGE, SOL_USD_FEED_ID, USDC_USD_FEED_ID};
use crate::state::*; 
use std::f32::consts::E;

use crate::error::ErrorCode;



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

    pub bank_token_account:InterfaceAccount<'info,TokenAccount> ,

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
    pub user_token_account:InterfaceAccount<'info,TokenAccount>, 

   
    pub price_update:Account<'info,PriceUpdateV2>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
   
    pub system_program:Program<'info,System>,


}


pub fn process_borrow(ctx:Context<Borrow>,amount:u64)->Result<()>{
    
    let user=&mut ctx.accounts.user_account;
     let bank=&mut ctx.accounts.bank; 
      let price_update=&mut ctx.accounts.price_update; 
       let total_collateral:u64;

        match ctx.accounts.mint.to_account_info().key() {
            key if key == user.usdc_address =>{
                let sol_feed_id=get_feed_id_from_hex(SOL_USD_FEED_ID)?; 
                let sol_price=price_update.get_price_no_older_than(&Clock ::get()?,MAX_AGE,&sol_feed_id)?;
                let accured_interest=calculate_accured_interest( user.deposited_sol,bank.interest_rate,user.last_updated)?;
                 total_collateral=sol_price.price as u64 *(user.deposited_sol +accured_interest);  
            }, 
            _=>{
                let usdc_feed_id=get_feed_id_from_hex(USDC_USD_FEED_ID)?;
                let usdc_price=price_update.get_price_no_older_than(&Clock::get()?,MAX_AGE,&usdc_feed_id)?; 
                 total_collateral=usdc_price.price as u64 * user.deposited_usdc;
            }

       };  


          let borrowable_amount=total_collateral as u64 * bank.liquidation_threshold;
            


            if borrowable_amount < amount{
               return Err(ErrorCode::OverBorrowableAmount.into());
            
            }


            let transfer_cpi_accounts=TransferChecked{
              from:ctx.accounts.bank_token_account.to_account_info(),
              mint:ctx.accounts.mint.to_account_info(),
              to:ctx.accounts.user_token_account.to_account_info(),
              authority:ctx.accounts.bank_token_account.to_account_info(),
            };

             
             let cpi_programs=ctx.accounts.token_program.to_account_info();

             //lets define pds signer seeds 
             let mint=ctx.accounts.mint.key();

             let signer_keys:&[&[&[u8]]]=&[
              &[
                b"treasury",
                mint.as_ref(),
                &[ctx.bumps.bank_token_account],
              ]
             ];



             let cpi_context=CpiContext::new(
              cpi_programs,
              transfer_cpi_accounts
             ).with_signer(signer_keys);


             let decimals=ctx.accounts.mint.decimals;
              token_interface::transfer_checked(cpi_context, amount, decimals)?;

          if bank.total_borrowed == 0 {
            bank.total_borrowed=amount;
            bank.total_borrowed_shares=amount;
          }
          

          let borrowed_ratio=amount.checked_div(bank.total_borrowed).unwrap();
           let  user_shares=bank.total_borrowed_shares.checked_mul(borrowed_ratio).unwrap(); 

           match ctx.accounts.mint.to_account_info().key() {
                 key if key == user.usdc_address =>{
                    user.borrowed_usdc +=amount;
                    user.borrowed_usdc_shares +=user_shares;
                },
                _ => {
                  user.borrowed_sol += amount;
                  user.borrowed_sol_shares += user_shares;
                }
           }


           bank.total_borrowed +=amount;
           bank.total_borrowed_shares +=user_shares; 
     


     

  Ok(())
}


fn calculate_accured_interest(deposited_ammount:u64,bank_interest:u64,last_updated:i64)->Result<u64>{
    let current_time=Clock::get()?.unix_timestamp; 
    let time_diff=current_time-last_updated;
    let new_rate=(deposited_ammount as f64 *E.powf(bank_interest as f32 *time_diff as f32) as f64 )as u64; 
   
   Ok(new_rate)
}