import { PublicKey } from "@solana/web3.js";

describe("wall_of_wish", () => {
  const program = pg.program;
  const wallet = pg.wallet;

  const [wishPda, wishBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("wish"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Creates a wish on the wall", async () => {
    const wishTitle = "I want to be a Blockchain developer";

    const txSignature = await program.methods
      .initialize(wishTitle)
      .accounts({
        wishAccount: wishPda,
        signer: wallet.publicKey,
        systemProgram: program.programId,
      })
      .rpc({ commitment: "confirmed" });

    const wishAccount = await program.account.aWish.fetch(wishPda, "confirmed");

    console.log(JSON.stringify(wishAccount, null, 2));
    console.log(
      "Transaction Signature:",
      `https://solana.fm/tx/${txSignature}?cluster=devnet-solana`
    );
  });
});
