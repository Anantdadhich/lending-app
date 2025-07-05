use anchor_lang::prelude::*;
 
 #[error_code]
pub enum ErrorCode{
  #[msg("insufficent funds ")] 
     InsufficientFunds,
      #[msg("amount exceed the borrow amount ")] 

     OverBorrowableAmount,
       #[msg("amount exceed the borrow amount ")] 

     OverRepaythenBorrowAmount,
#[msg("insufficient collateral ")] 

     InsficientCollateral

}