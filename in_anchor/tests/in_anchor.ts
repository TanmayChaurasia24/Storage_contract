import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// @ts-ignore: Import type may not be generated yet
import type { InAnchor } from "../target/types/in_anchor";
import { assert } from "chai";

describe("in_anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.inAnchor as Program<InAnchor>;

  it("Is initialized!", async () => {
    const dataAccount = anchor.web3.Keypair.generate();
    const payer = anchor.web3.Keypair.generate();
    
    const airdrop_txn = await anchor.getProvider().connection.requestAirdrop(payer.publicKey, 1000000000000000);
    await anchor.getProvider().connection.confirmTransaction(airdrop_txn);
    
    // Add your test here.
    const tx_init = await program.methods.initialize("Tanmay").accounts({
      dataAccount: dataAccount.publicKey,
      payer: payer.publicKey
    }).signers([payer, dataAccount]).rpc();

    let dataAccountInfo = await program.account.accountFormat.fetch(dataAccount.publicKey);
    assert(dataAccountInfo.name, "Tanmay");

    await program.methods.update("Tanmay Kumar Chaurasia").accounts({
      dataAccount: dataAccount.publicKey,
      payer: payer.publicKey,
    }).signers([payer]).rpc();

    dataAccountInfo = await program.account.accountFormat.fetch(dataAccount.publicKey);
    assert(dataAccountInfo.name, "Tanmay Kumar Chaurasia");
  });
});
