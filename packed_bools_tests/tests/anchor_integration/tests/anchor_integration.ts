import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import { AnchorIntegration } from "../target/types/anchor_integration";

const DEBUG = false;

describe("anchor_integration", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorIntegration as Program<AnchorIntegration>;

  if (!DEBUG) {
    console.log = function () {};
  } else {
    program.provider.connection.onLogs("all", ({ logs }) => {
      console.log(logs);
    });
  }

  it("Is initialized!", async () => {

    // Fund a user
    let user = anchor.web3.Keypair.generate();
    let user_funded = await provider.connection
      .requestAirdrop(user.publicKey, LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(user_funded)
      console.log(`    user funded: ${user_funded}`);

    // Check user was funded
    let user_balance = await provider.connection
      .getBalance(user.publicKey);
      console.log(`    user balance: ${user_balance}`);
    
    // Initialize pda with some flags
    const tx = await program.methods
      .initialize()
      .accounts({
          user: user.publicKey
      })
      .signers([user])
      .rpc();
    console.log("Your transaction signature", tx);

    // Get flags byte
    let [user_account_address, _bump] = findProgramAddressSync(
      [Buffer.from("my_account_seeds")],
      program.programId
    );
    let user_account = await program.account.myAccount.fetch(user_account_address)
    // check that it is equal to what we expect
    const FIRST_FLAG = 0;
    const SECOND_FLAG = 1;
    const THIRD_FLAG = 1;
    const EXPECT = (FIRST_FLAG * 2**0) + (SECOND_FLAG * 2**1) + (THIRD_FLAG * 2**2);
    assert.equal(user_account.accountFlags, EXPECT);
  });
});
