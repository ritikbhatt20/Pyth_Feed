import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PythContract } from "../target/types/pyth_contract";
import { PublicKey, Keypair, Connection, SystemProgram } from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers";
import { assert } from "chai";

describe("pyth_contract", () => {
  // Set up provider and program
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.PythContract as Program<PythContract>;

  let signer: anchor.web3.Keypair;

  before(async () => {
    // Create a new keypair for the payer
    signer = await getKeypairFromFile("/home/ritikbhatt020/multi-token-escrow/keys/admin-CAT5qnvWfU9LQyprcLrXDMMifR6tL95nCrsNk8Mx12C7.json");

    // Airdrop SOL to the payer to cover transaction fees
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  });

  it("Fetches the BTC/USD price from the Pyth price feed", async () => {
    // Call the fetch_btc_price instruction in your program
    const tx = await program.methods
      .fetchBtcPrice()
      .accounts({
        signer: signer.publicKey,   // Use the connected wallet as the signer
      })
      .signers([signer])
      .rpc();

    console.log("Transaction signature", tx);

    // Add any necessary assertions here, for example:
    // assert.ok(tx, "Transaction was successful");
  });
});
