import { PublicKey, Keypair, Connection } from "@solana/web3.js";

import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';

import * as services from "../services";

import { createMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID, createAssociatedTokenAccount } from "@solana/spl-token";
import { ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

const PROGRAM_ID: PublicKey = services.PROGRAM_ID;

describe("Airdrop Token Program", () => {
  let connection = services.CONNECTION;
  const bloomFilterPubkey = services.getBloomFilterPubkey(PROGRAM_ID);
  let defaultAccount: Keypair;
  let tokenMint: Keypair = Keypair.generate();

  before(async () => {
    const walletPath = path.join(os.homedir(), '.config', 'solana', 'id.json');
    const savedKey = JSON.parse(fs.readFileSync(walletPath, 'utf-8'));
    defaultAccount = Keypair.fromSecretKey(new Uint8Array(savedKey));
    const tokenMint = Keypair.generate();

  });

  it("Initialize the program", async () => {
    const tx = await services.initialize(
      connection,
      defaultAccount,
      bloomFilterPubkey,
    );
    console.log("Transaction signature:", tx);
  });

  it("Claim airdrop", async () => {
    const mint = await createMint(
      connection,
      defaultAccount,
      defaultAccount.publicKey,
      null,
      0, // Decimals
      tokenMint
    );

    console.log("Token Mint Address:", mint.toBase58());
    const userTokenAccountInfo = await createAssociatedTokenAccount(
      connection,
      defaultAccount,           // payer
      mint,      // mint
      defaultAccount.publicKey,  // owner
      undefined,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
      true
    );

    console.log("User Token Account:", userTokenAccountInfo.toBase58());
    let programAuthority = services.getProgramAuthority(PROGRAM_ID);
    console.log("Program Authority:", programAuthority.toBase58());


    const programAuthorityTokenAccount = await createAssociatedTokenAccount(
      connection,
      defaultAccount,
      mint,      // mint
      programAuthority,  // owner
      undefined,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
      true // allowOwnerOffCurve
    );
    console.log("Program Token Account:", programAuthorityTokenAccount.toBase58());

    await mintTo(
      connection,
      defaultAccount,
      mint,
      programAuthorityTokenAccount,
      defaultAccount.publicKey,
      1000000000 // Mint 1 million tokens
    );

    const tx = await services.claimAirdrop
      (
        connection,
        defaultAccount,
        mint,
        bloomFilterPubkey,
        programAuthority,
        programAuthorityTokenAccount,
        userTokenAccountInfo
      );
    console.log("Transaction signature:", tx);
  });
});