import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PythContract } from "../target/types/pyth_contract";
import { PublicKey, Keypair } from "@solana/web3.js";
import { PriceUpdateAccount } from "@pythnetwork/pyth-solana-receiver/lib/PythSolanaReceiver";

describe("pyth_contract", () => {
  // Set up provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.PythContract as Program<PythContract>;

  // Pyth price account (replace with a valid account from Pyth)
  const priceUpdateAccount = new PublicKey("YourPythPriceAccountHere");

  it("Fetches the Pyth price and logs it", async () => {
    // Create a new keypair for the payer
    const payer = Keypair.generate();

    // Call the sample instruction
    const tx = await program.methods.sample()
      .accounts({
        payer: provider.wallet.publicKey,
        priceUpdate: priceUpdateAccount,
      })
      .signers([payer])
      .rpc();

    console.log("Transaction signature", tx);
  });
});