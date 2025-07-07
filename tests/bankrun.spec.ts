import { BankrunProvider } from "anchor-bankrun";
import  {describe ,it} from "node:test"



//@ts-ignore
import IDL from "../target/idl/lending.json";

import { Lending } from "../target/types/lending";
import { ProgramTestContext, startAnchor } from "solana-bankrun";
import { Connection } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";



describe("Lending smart contract tests", async ()=>{

     let provider:BankrunProvider;
     let context:ProgramTestContext;
   
   
const connection=new Connection("https://api.devnet.solana.com");
const pyth=new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");
const accountinfo=await connection.getAccountInfo(pyth);
 

context=await startAnchor(
          "",
          [{
               name:"lending",
               programId:new PublicKey(IDL.address),
          }],[{
               address: pyth  ,
               info:accountinfo
          },
     ]
     );

     provider=new BankrunProvider(context);
 
    const SOL_FEED_ID="0x41f858bae36e7ee3f4a3a6d4f176f0893d4a261460a52763350d00f8648195ee";
   const PythReceiver=new PythSolanaReceiver(
     {
          connection,
          wallet:provider.wallet
     }
   );

   const solusdfeedpricefeedamount=PythReceiver.getPriceFeedAccountAddress(0,SOL_FEED_ID);

    
const solusdfeedpriceaccountpubkey=new PublicKey(solusdfeedpricefeedamount);
 const feefaccountinfo=await connection.getAccountInfo(solusdfeedpriceaccountpubkey);

 context.setAccount(solusdfeedpriceaccountpubkey,feefaccountinfo);

 console.log("price feed",solusdfeedpricefeedamount);
 console.log("price account info ",accountinfo)

})