import {
    getAssociatedTokenAddressSync,
    getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { Connection, PublicKey } from "@solana/web3.js";

export function getBloomFilterPubkey(
    programID: PublicKey,
): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("bloom-filter")],
        programID,
    )[0];
}

export async function getUserTokenAccount(
    userPubkey: PublicKey,
    mintPubkey: PublicKey,
): Promise<PublicKey> {
    return getAssociatedTokenAddressSync(mintPubkey, userPubkey);
}

export async function getProgramTokenAccount(
    programID: PublicKey,
    mintPubkey: PublicKey,
): Promise<PublicKey> {
    return getAssociatedTokenAddressSync(mintPubkey, programID);
}

export function getProgramAuthority(
    programID: PublicKey,
): PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("root-authority")],
        programID,
    )[0];
}



