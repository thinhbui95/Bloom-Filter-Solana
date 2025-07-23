import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { AirdropBloomFilter, IDL } from "../target/types/airdrop_bloom_filter";

export const PROGRAM_ID = new PublicKey("F9P4gcXZKQZRSopRDktRZysjNujwygz6k7Qg9JBnHcXi")
export const CONNECTION: Connection = new Connection("http://127.0.0.1:8899", "confirmed");

export const PROGRAM = new Program(IDL, PROGRAM_ID, new AnchorProvider(
    CONNECTION,
    new Wallet(Keypair.generate()),
    {}
)) as Program<AirdropBloomFilter>;