import { BankrunProvider } from "anchor-bankrun";
import  {describe ,it} from "node:test"




import IDL from "../target/idl/lending.json";

import { Lending } from "../target/types/lending";
import { BanksClient, ProgramTestContext, startAnchor } from "solana-bankrun";
import { Connection, Keypair } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";
import { BN, Program } from "@coral-xyz/anchor";
import { createAccount, createMint, mintTo } from "spl-token-bankrun";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BankrunContextWrapper } from "../bankrun-utils/bankrunConnection";


describe("Lending smart contract tests", async ()=>{
     let signer:Keypair; 
     let provider:BankrunProvider;
     let context:ProgramTestContext;
     let program:Program<Lending>;
     
     
     let usdcbankAccount:PublicKey ;
     let solbankAccount:PublicKey ;  

     let solTokenAccount:PublicKey; 

     let banksclient:BanksClient; 
     let bankrunContextwrapper:BankrunContextWrapper;


   
const dvenetconnection=new Connection("https://api.devnet.solana.com");
const pyth=new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");
const accountinfo=await dvenetconnection.getAccountInfo(pyth);
 

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

     bankrunContextwrapper=new BankrunContextWrapper(context);
     const connection=bankrunContextwrapper.connection.toConnection();
 
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

program=new Program<Lending>(IDL as Lending,provider) ; 

banksclient=context.banksClient ;

signer=provider.wallet.payer; 


const mintusdc=await createMint(
     //@ts-ignore
     banksclient,
     signer,
     signer.publicKey ,
     null,
     2
);

const mintsol=await createMint(
     //@ts-ignore
     banksclient,
     signer,
     signer.publicKey,
     null,
     2
);

    
//lets create the program address 
[usdcbankAccount] =PublicKey.findProgramAddressSync(
     [Buffer.from("treasury"),mintusdc.toBuffer()],
     program.programId
);

[solbankAccount]=PublicKey.findProgramAddressSync(
     [Buffer.from("treasury"),mintsol.toBuffer()],
     program.programId
);

[solTokenAccount]=PublicKey.findProgramAddressSync(
     [Buffer.from("treasury"),mintsol.toBuffer()],
     program.programId
)

console.log("usdc bank account",usdcbankAccount.toBase58()) 

console.log("sol bank account",solbankAccount.toBase58())

 it("Test Init User",async ()=> {
     const inituserTx=await program.methods.initUser(mintusdc).accounts(
          {
               signer:signer.publicKey 
          }
     ).rpc({
          commitment:"confirmed"
     })

     console.log("create user account",inituserTx) 
 })



 it("Test init and fund USDC bank",async ()=>{
     const initusdcbanktx=await program.methods.initBank(new BN(1),new BN(1)).accounts({
          signer:signer.publicKey, 
          mint:mintusdc ,
          tokenProgram:TOKEN_PROGRAM_ID
     }).rpc({
          commitment:"confirmed"
     });

     console.log("init usdc bank account" ,initusdcbanktx  ); 
     
     const amount=10_000 * 10 ** 9 ;

     const minttx=await mintTo(
          //@ts-ignore
          banksclient, 
          signer, 
          mintusdc, 
          usdcbankAccount,  
          signer, 
          amount 
     ); 
     

       console.log("Mint to usdc bank " ,minttx) 


 }) 


 it("test init and fund sol bank ",async ()=> {
      const initsol=await program.methods.initBank(new BN(1),new BN(1)).
      accounts({
          signer:signer.publicKey, 
          mint:mintsol,
          tokenProgram:TOKEN_PROGRAM_ID
      }).rpc({
          commitment:"confirmed"
      })

      console.log("create sol bank account",initsol ) 

      const amount=10_000 * 10 ** 9; 

      const mintsoltx=mintTo(
          //@ts-ignore
          banksclient,
          signer,
          mintsol,
          solbankAccount, 
          signer ,
           amount
      ); 

      console.log("mint to sol bank sig" ,mintsoltx)  
 })


  it("Create and fund token account",async ()=> {
      const usdctokenAccount=await createAccount(
          //@ts-ignore
          banksclient,
          signer,
          mintusdc,
          signer.publicKey
       
      ) ;
      console.log("USDC token account created",usdctokenAccount); 
        

      const amount=10_000 * 10 **9 ;
      const mintusdtx=await mintTo(
           //@ts-ignore
          banksclient, 
          signer,
          mintusdc, 
          usdctokenAccount ,
          signer,
          amount 
      )

      console.log("mint to usdc bank signature",mintusdtx);

  })
   
  it("test deposit",async ()=>{
     const depostusdc=await program.methods.deposit(new BN(100000000000))
     .accounts({
          signer:signer.publicKey, 
          mint:mintusdc, 
          tokenProgram:TOKEN_PROGRAM_ID
     }).rpc({
          commitment:"confirmed"
     }); 

     console.log("deposit usdc" ,depostusdc); 
     
     

  }); 


  it("test borrow",async ()=> {
       const borrowsol=await program.methods.borrow(new BN(1)).accounts({
          signer:signer.publicKey, 
          mint:mintsol, 
          tokenProgram:TOKEN_PROGRAM_ID, 
          priceUpdate:solusdfeedpricefeedamount, 
       }).rpc({
          commitment:"confirmed"
       })


       console.log("borrow sol ",borrowsol)    


  })

  it("test repay",async ()=> {
     const repaysol=await program.methods.repay(new BN(1)).accounts({
          signer:signer.publicKey, 
          mint:mintsol, 
          tokenProgram:TOKEN_PROGRAM_ID
     }).rpc({
          commitment:"confirmed"
     })

     console.log("Repay sol ",repaysol) 
  })

  it("test withdraw", async () => { 

     const withdraw=await program.methods.withdraw(new BN(100))
     .accounts({
          signer:signer.publicKey,
          mint:mintusdc, 
          tokenProgram:TOKEN_PROGRAM_ID 
     }).rpc({
          commitment:"confirmed"
     })

     console.log("withdraw usdc  ",withdraw) 
     
  }); 
      
}) ;