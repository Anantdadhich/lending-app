use anchor_lang::prelude::*; 

#[account] 
#[derive(InitSpace)]
pub struct  User{
  //account 
   pub owner:Pubkey ,
    //sol token  
  pub deposited_sol:u64,
   
  pub depsoited_sol_shares:u64 ,

  pub deposited_usdc:u64 ,

  pub deposited_usdc_shares:u64 ,
 
   
  pub borrowed_sol:u64 ,

  pub borrowed_usdc:u64 ,

  pub borrowed_sol_shares:u64,

  pub borrowed_usdc_shares:u64 ,

  pub usdc_address:Pubkey ,

  pub health_factor:u64 ,

  pub last_updated:i64 

}


#[account] 
#[derive(InitSpace)] 
pub struct Bank{ 
  //pda or public ket that manages the bank 
   pub authority:Pubkey,
    //token mint      
   pub mint_address:Pubkey, 
    //total tokens deposited  
   pub total_deposits:u64 ,
    
   pub total_deposits_shares:u64 ,

   pub total_borrowed:u64 ,


   pub total_borrowed_shares:u64 ,
  
   pub liquidation_threshold:u64 ,
     //how much debt user can repaid during single time of liquidation   
   pub liquidation_close_factor:u64 ,
    //max loan to value  
   pub max_ltv:u64 ,
    //extra rewards the liquidator can earn to repaing risky loans 
   pub liquidation_bonus:u64 , 

   pub interest_rate:u64,

   pub last_updated:i64   
} 
 
