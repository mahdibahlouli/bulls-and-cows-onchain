// tests/bulls_and_cows_onchain.ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("bulls_and_cows_onchain", () => {
  // Use the local provider that Anchor sets up for `anchor test`
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Get the program (name must match your crate/folder name)
  const program = anchor.workspace
    .BullsAndCowsOnchain as Program<any>;

  // IMPORTANT: seed string must match your Rust seeds EXACTLY:
  // #[account(..., seeds = [b"guessing pda"], bump)]
  const seedBytes = Buffer.from("guessing pda"); // or "guessing_pda" if you used underscore in Rust
  const [guessingPdaPubkey] = anchor.web3.PublicKey.findProgramAddressSync(
    [seedBytes],
    program.programId
  );

  it("initialize + (optional) guess", async () => {
    // Initialize
    const initializeTx = await program.methods
      .initialize()
      .accounts({
        guessingAccount: guessingPdaPubkey,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Initialize tx:", initializeTx);

    // Try a guess (uncomment and set a number to test)
    // const guessingTx = await program.methods
    //   .guess(5)
    //   .accounts({
    //     guessingAccount: guessingPdaPubkey,
    //     payer: provider.wallet.publicKey,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //   })
    //   .rpc();
    // console.log("Guess tx:", guessingTx);
  });
});
