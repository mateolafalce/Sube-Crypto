import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SubeCrypto } from "../target/types/sube_crypto";
import { PublicKey} from '@solana/web3.js'

describe("sube-crypto", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SubeCrypto as Program<SubeCrypto>;

  it("Is initialized!", async () => {
    const [BusLine, _bump] = await PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const tx = await program.methods.initializeBusLine(
      new anchor.BN(1785714),
      new anchor.BN(2000000),
      new anchor.BN(2100000),
      new anchor.BN(2200000),
      new anchor.BN(2300000),
    )
    .accounts({
      sube: BusLine,
      signer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});