import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CoinFlip } from "../target/types/coin_flip";
import {PublicKey} from "@solana/web3.js";

describe("coin-flip", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const provider = anchor.AnchorProvider.env();
  const signer = provider.publicKey;

  const program = anchor.workspace.CoinFlip as Program<CoinFlip>;

  // it("initialize storage", async () => {
  //   // Add your test here.
  //   try{
  //     const [pda, _] = await PublicKey.findProgramAddressSync(
  //       [
  //         anchor.utils.bytes.utf8.encode('storage')
  //       ],
  //       program.programId
  //     )
  
  //     const tx = await program.methods.initstorage()
  //     .accounts({
  //       pda : pda,
  //       signer : signer,
  //     })
  //     .rpc();
  //     console.log("Your transaction signature", tx);  

  //     let result = await program.account.answer.fetch(pda);
  //     console.log(result)
  //   }
  //   catch(error){
  //     console.log(error);
  //   }
  //   });

  it("transfer sol to pda", async () => {
    // Add your test here.
    try{
      const [pda, _] = await PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode('coinflip1')
        ],
        program.programId
      )
  
      const [storagepda, bump] = await PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode('storage')
        ],
        program.programId
      )
      const tx = await program.methods.main()
      .accounts({
        pda : pda,
        signer : signer,
        stroagePda : storagepda
      })
      .rpc();
      console.log("Your transaction signature", tx);  

      let result = await program.account.answer.fetch(storagepda);
      console.log(result)
    }
    catch(error){
      console.log(error);
    }
    });
});
