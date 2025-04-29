// No imports needed: web3, anchor, pg and more are globally available in Solana Playground

describe("Counter Program Test", () => {
  const counterKeypair = anchor.web3.Keypair.generate(); // Counter account
  const wallet = pg.wallet; // User wallet
  const program = pg.program; // Program reference

  it("Initialize the counter account", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        user: wallet.publicKey,
        counter: counterKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([wallet.keypair, counterKeypair]) // Important: wallet and counter must sign
      .rpc();

    console.log(
      "Initialize Transaction:",
      `https://solana.fm/tx/${tx}?cluster=devnet-solana`
    );

    const counterAccount = await program.account.counter.fetch(
      counterKeypair.publicKey
    );
    console.log("Counter after initialize:", counterAccount.count.toString());
  });

  it("Increment the counter", async () => {
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .signers([wallet.keypair]) // Only wallet signs
      .rpc();

    console.log(
      "Increment Transaction:",
      `https://solana.fm/tx/${tx}?cluster=devnet-solana`
    );

    const counterAccount = await program.account.counter.fetch(
      counterKeypair.publicKey
    );
    console.log("Counter after increment:", counterAccount.count.toString());
  });

  it("Decrement the counter", async () => {
    const tx = await program.methods
      .decrement()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .signers([wallet.keypair]) // Only wallet signs
      .rpc();

    console.log(
      "Decrement Transaction:",
      `https://solana.fm/tx/${tx}?cluster=devnet-solana`
    );

    const counterAccount = await program.account.counter.fetch(
      counterKeypair.publicKey
    );
    console.log("Counter after decrement:", counterAccount.count.toString());
  });
});
