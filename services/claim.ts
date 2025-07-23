import * as anchor from "@coral-xyz/anchor";
import { PROGRAM } from "./constants";
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

export async function claimAirdrop(
    connection: Connection,
    userKeypair: Keypair,
    tokenMint: PublicKey,
    bloomFilterPubkey: PublicKey,
    programAuthority: PublicKey,
    programAuthorityTokenAccountKey: PublicKey,
    userTokenAccountKey: PublicKey,
) {
    const inst = await PROGRAM.methods
        .claimAirdrop()
        .accounts({
            bloomFilter: bloomFilterPubkey,
            user: userKeypair.publicKey,
            mint: tokenMint,
            tokenProgram: TOKEN_PROGRAM_ID,
            programAuthority: programAuthority,
            programAuthorityTokenAccount: programAuthorityTokenAccountKey,
            userTokenAccount: userTokenAccountKey,
            program: PROGRAM.programId,
        })
        .instruction();

    const transaction = new Transaction().add(inst);
    return connection.sendTransaction(transaction, [userKeypair], {
        skipPreflight: true,
    });
}