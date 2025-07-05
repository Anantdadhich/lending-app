

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::error::ErrorCode;

use crate::{constants::{MAX_AGE, SOL_USD_FEED_ID, USDC_USD_FEED_ID}, state::{Bank, User}};

#[derive(Accounts)]
pub struct Liquidate<'info>{
    #[account(mut)]
    pub liquidator:Signer<'info>,

    pub price_update:Account<'info,PriceUpdateV2>,


    pub collateral_mint:InterfaceAccount<'info,Mint>,

    pub borrowed_mint:InterfaceAccount<'info,Mint>,
     #[account(
        mut,
        seeds=[collateral_mint.key().as_ref()],
        bump
     )]
    pub bank_collateral:Account<'info,Bank>,
    
    #[account(
        mut,
        seeds=[borrowed_mint.key().as_ref()],
        bump
    )] 

    pub bank_borrowed:Account<'info,Bank>,
      #[account(
        mut,
        seeds=[b"treasury" ,collateral_mint.key().as_ref()],
        bump
     )]
    pub bank_collateral_token_account:InterfaceAccount<'info,TokenAccount>, 
      #[account(
        mut,
        seeds=[b"treasury" ,borrowed_mint.key().as_ref()],
        bump
     )]
    pub bank_borrowed_token_account:InterfaceAccount<'info,TokenAccount>,
      #[
        account(
            mut,
            seeds=[liquidator.key().as_ref()],
            bump
        )
      ]
     pub user_account:Account<'info,User>,
      
      #[account(
        init_if_needed,
        payer=liquidator,
        associated_token::mint=collateral_mint,
        associated_token::authority=liquidator,
        associated_token::token_program=token_program
      )]
     pub liquidator_collateral_token_account:InterfaceAccount<'info,TokenAccount>,

         #[account(
        init_if_needed,
        payer=liquidator,
        associated_token::mint=borrowed_mint,
        associated_token::authority=liquidator,
        associated_token::token_program=token_program
      )]
     pub liquidator_borrowed_token_account:InterfaceAccount<'info,TokenAccount>,




    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>, 
}

pub fn process_liquidate(ctx:Context<Liquidate>)->Result<()>{

    let collateral_bank=&mut ctx.accounts.bank_collateral; 

     let user=&mut ctx.accounts.user_account;

     let price_update=&mut ctx.accounts.price_update; 

     let sol_feed_id=get_feed_id_from_hex(SOL_USD_FEED_ID)?; 
     let usdc_usd_feed_id=get_feed_id_from_hex(USDC_USD_FEED_ID)?; 


     let sol_price=price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &sol_feed_id)?;
     let usdc_price=price_update.get_price_no_older_than(&Clock::get()?, MAX_AGE, &usdc_usd_feed_id)?; 

     let total_collateral=(sol_price.price as u64 * user.deposited_sol) + (usdc_price.price as u64 + user.deposited_usdc); 
     let total_borrowed=(sol_price.price as u64 * user.borrowed_sol) + (usdc_price.price as u64 + user.borrowed_usdc);

     let health_factor=total_collateral + collateral_bank.liquidation_bonus / total_borrowed ;

     if health_factor >=1 {
      return  Err(ErrorCode::InsficientCollateral.into());  
     }


     let liquidation_amount=total_borrowed + collateral_bank.liquidation_threshold; 


     let transfer_cpi_accounts=TransferChecked{
      from:ctx.accounts.liquidator_borrowed_token_account.to_account_info(),
      mint:ctx.accounts.borrowed_mint.to_account_info(),
      to:ctx.accounts.bank_borrowed_token_account.to_account_info(),
      authority:ctx.accounts.liquidator.to_account_info()
     };

   let cpi_programs=ctx.accounts.token_program.to_account_info();

   let cpi_context_tobank=CpiContext::new(cpi_programs.clone(), transfer_cpi_accounts);

   let decimals=ctx.accounts.borrowed_mint.decimals;

   token_interface::transfer_checked(cpi_context_tobank, liquidation_amount, decimals)?; 

   let liquidation_bonus=(liquidation_amount * collateral_bank.liquidation_bonus) + liquidation_amount;

   let transfer_to_liquidator=TransferChecked{
    from:ctx.accounts.bank_collateral_token_account.to_account_info(),
    mint:ctx.accounts.collateral_mint.to_account_info(),
    to:ctx.accounts.liquidator_collateral_token_account.to_account_info(),
    authority:ctx.accounts.bank_collateral_token_account.to_account_info()
   };

   let mint_key=ctx.accounts.collateral_mint.key();

      let signer_keys:&[&[&[u8]]]=&[
              &[
                b"treasury",
                mint_key.as_ref(),
                &[ctx.bumps.bank_collateral_token_account],
              ]
             ];
    let cpi_context_to_collateral=CpiContext::new(cpi_programs.clone(),transfer_to_liquidator).with_signer(signer_keys);

    let collaterl_decimals=ctx.accounts.collateral_mint.decimals; 


     token_interface::transfer_checked(cpi_context_to_collateral, liquidation_bonus, collaterl_decimals)?; 

   
    Ok(())
}