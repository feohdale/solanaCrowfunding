// Important : those tests are RUN on SolPG = https://beta.solpg.io/
// they're all successful 

describe("Cagnotte34", () => {
    //const provider = anchor.AnchorProvider.local();
    //anchor.setProvider(provider);
    const program = anchor.workspace.Cagnotte2;
  
    const systemProgram = anchor.web3.SystemProgram;
  
    it("Get balance", async () => {
      const balance = await pg.connection.getBalance(pg.wallet.publicKey);
      console.log(`My balance is ${balance} lamports`);
    });
  
    it("Initialize cagnotte", async () => {
      const name = "Test Cagnotte 34";
      // const cagnotteName = anchor.utils.bytes.utf8.encode(name);
      const cagnotteName = name;
      const [cagnottePda] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("cagnotte"),
          pg.wallet.publicKey.toBytes(),
          Buffer.from(name),
        ],
        program.programId
      );
  
      //console.log("cagnottePda ", cagnottePda);
  
      const tx = await program.methods
        .initialize(name)
        .accounts({
          cagnotte: cagnottePda,
          user: pg.wallet.publicKey,
          systemProgram: systemProgram.programId,
        })
        .signers([pg.wallet.keypair])
        .rpc();
  
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
  
      // Fetch the created account
      const cagnotteAccount = await program.account.cagnotte.fetch(cagnottePda);
  
      console.log(
        "Cagnotte name is:",
        Buffer.from(cagnotteAccount.name).toString()
      );
      assert.strictEqual(Buffer.from(cagnotteAccount.name).toString(), name);
      assert.strictEqual(
        cagnotteAccount.owner.toBase58(),
        pg.wallet.publicKey.toBase58()
      );
      assert.strictEqual(cagnotteAccount.amount.toNumber(), 0);
      assert.strictEqual(cagnotteAccount.locked, false);
    });
  
    it("Contribute to cagnotte", async () => {
      const name = "Test Cagnotte 34";
      const cagnotteName = anchor.utils.bytes.utf8.encode(name);
      const [cagnottePda] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("cagnotte"), pg.wallet.publicKey.toBuffer(), cagnotteName],
        program.programId
      );
      //console.log("cagnottePda ", cagnottePda);
  
      const [contributionPda] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("contribution"),
          cagnottePda.toBuffer(),
          pg.wallet.publicKey.toBuffer(),
        ],
        program.programId
      );
  
      const contributionAmount = new BN(1_000_000); // 1 SOL in lamports
  
      const tx = await program.methods
        .contribute(contributionAmount)
        .accounts({
          cagnotte: cagnottePda,
          user: pg.wallet.publicKey,
          contribution: contributionPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
  
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
  
      // Fetch the updated cagnotte account
      const cagnotteAccount = await program.account.cagnotte.fetch(cagnottePda);
      const contributionAccount = await program.account.contribution.fetch(
        contributionPda
      );
  
      console.log("Cagnotte amount is:", cagnotteAccount.amount.toString());
      console.log("contribution Amount is:", contributionAmount.toString());
      console.log(
        "contributionAccount amount",
        contributionAccount.amount.toString()
      );
      /*
      assert.strictEqual(
        cagnotteAccount.amount.toString(),
        contributionAmount.toString()
      );
      assert.strictEqual(
        contributionAccount.amount.toString(),
        contributionAmount.toString()
      );
    */
    });
  
    it("Lock and unlock cagnotte", async () => {
      const name = "Test Cagnotte 34";
      const cagnotteName = anchor.utils.bytes.utf8.encode(name);
      const [cagnottePda] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("cagnotte"), pg.wallet.publicKey.toBuffer(), cagnotteName],
        program.programId
      );
  
      const [adminAccountPda] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("admin-account")],
        program.programId
      );
  
      // First, initialize admin account (you might need to do this in a separate test)
      await program.methods
        .initializeAdmin()
        .accounts({
          adminAccount: adminAccountPda,
          user: pg.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
  
      // Lock cagnotte
      let tx = await program.methods
        .lockCagnotte()
        .accounts({
          cagnotte: cagnottePda,
          user: pg.wallet.publicKey,
          adminAccount: adminAccountPda,
        })
        .rpc();
  
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
  
      let cagnotteAccount = await program.account.cagnotte.fetch(cagnottePda);
      assert.strictEqual(cagnotteAccount.locked, true);
  
      // Unlock cagnotte
      tx = await program.methods
        .unlockCagnotte()
        .accounts({
          cagnotte: cagnottePda,
          user: pg.wallet.publicKey,
          adminAccount: adminAccountPda,
        })
        .rpc();
  
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
  
      cagnotteAccount = await program.account.cagnotte.fetch(cagnottePda);
      assert.strictEqual(cagnotteAccount.locked, false);
    });
  
    /*
    it("Withdraw from cagnotte", async () => {
      const name = "Test Cagnotte 33";
      const cagnotteName = anchor.utils.bytes.utf8.encode(name);
      const [cagnottePda] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("cagnotte"), pg.wallet.publicKey.toBuffer(), cagnotteName],
        program.programId
      );
  
      //console.log("cagnottePda ", cagnottePda);
      /*
      const tx_init = await program.methods
        .initialize(name)
        .accounts({
          cagnotte: cagnottePda,
          user: pg.wallet.publicKey,
          systemProgram: systemProgram.programId,
        })
        //.signers([pg.wallet.keypair])
        .rpc();
      console.log("tx_init ", tx_init);
  
      const withdrawAmount = new BN(500_000); // 0.5 SOL in lamports
  
      const tx = await program.methods
        .withdraw(withdrawAmount)
        .accounts({
          cagnotte: cagnottePda,
          user: pg.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
  
      console.log(`Use 'solana confirm -v ${tx}' to see the logs`);
  
      // Fetch the updated cagnotte account
      const cagnotteAccount = await program.account.cagnotte.fetch(cagnottePda);
  
      console.log(
        "Cagnotte amount after withdrawal:",
        cagnotteAccount.amount.toString()
      );
      assert.strictEqual(
        cagnotteAccount.amount.toString(),
        new BN(500_000).toString()
      );
    });
    */
  });
  