import { describe, it, before } from "node:test";
import assert from "node:assert";
import * as escrowClient from "../clients/js/src/generated";
import {
  Address,
  generateKeyPairSigner,
  getExplorerLink,
  getSignatureFromTransaction,
  KeyPairSigner,
  Lamports,
  LAMPORTS_PER_SOL,
  signTransaction,
  signTransactionMessageWithSigners,
} from "gill";
//import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
  buildCreateTokenTransaction,
  buildMintTokensTransaction,
  getAssociatedTokenAccountAddress,
  SYSTEM_PROGRAM_ADDRESS,
  TOKEN_2022_PROGRAM_ADDRESS,
} from "gill/programs";
import { createSolanaClient } from "gill";
import { Keypair } from "@solana/web3.js";

let maker: KeyPairSigner;
let taker: KeyPairSigner;
let mintA: KeyPairSigner;
let mintB: KeyPairSigner;
let makerAtaA: KeyPairSigner;
let escrow: Address;
let vaultA: Address;
let associatedTokenProgram: Address = ASSOCIATED_TOKEN_PROGRAM_ADDRESS;
let tokenProgram: Address = TOKEN_2022_PROGRAM_ADDRESS;
let systemProgram: Address = SYSTEM_PROGRAM_ADDRESS;
let seed: bigint | number = 1;
let deposit: bigint | number = 1;
let receiveAmount: bigint;
let feePayer: KeyPairSigner;
//let rpc, rpcSubscriptions, sendAndConfirmTransaction;

const { rpc, rpcSubscriptions, sendAndConfirmTransaction } = createSolanaClient(
  {
    urlOrMoniker: "localnet",
  }
);

const createSigner = async () => {
  let keyPair = await generateKeyPairSigner();
  await rpc.requestAirdrop(keyPair.address, 5_000_000_000 as any).send();

  await new Promise((resolve) => setTimeout(resolve, 2000)); // wait for airdrop to finalize
  console.log("Keypair generated with address: ", keyPair.address);
  return keyPair;
};

const createMint = async (
  signer: KeyPairSigner,
  mint: KeyPairSigner,
  name: string
) => {
  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
  const tx = await buildCreateTokenTransaction({
    feePayer: signer,
    latestBlockhash,
    mintAuthority: signer,
    mint: mint,
    decimals: 10,
    metadata: {
      name: name,
      symbol: "TEST",
      uri: "https://example.com/metadata.json",
      isMutable: true,
    },
    tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
  });
};

const mintTokens = async (
  feePayer: KeyPairSigner,
  mint: KeyPairSigner,
  mintAuthority: KeyPairSigner,
  destinationOwner: Address,
  amount: number
) => {
  const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
  const tx = await buildMintTokensTransaction({
    feePayer,
    version: "legacy",
    latestBlockhash,
    amount: 10 * amount,
    mint: mint,
    mintAuthority,
    destination: destinationOwner,
    tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
  });

  const signedTransaction = await signTransactionMessageWithSigners(tx);
  console.log(
    "Explorer: ",
    getExplorerLink({
      cluster: "localhost",
      transaction: getSignatureFromTransaction(signedTransaction),
    })
  );
};

describe("anchor-escrow", () => {
  // Configure the client to use the local cluster.
  before(async () => {
    feePayer = await createSigner();
    maker = await createSigner();
    taker = await createSigner();

    mintA = await generateKeyPairSigner();
    mintB = await generateKeyPairSigner();

    await createMint(feePayer, mintA, "mint A");
    await createMint(feePayer, mintB, "mint B");

    await mintTokens(feePayer, mintA, feePayer, maker.address, 10);
  });
  it("Is make", async () => {
    /*let expectedMakerAtaA = await getAssociatedTokenAccountAddress({
      mintA.address,
      maker.address,
      TOKEN_2022_PROGRAM_ADDRESS
    });*/

    const ix = escrowClient.getInitializeInstruction({
      maker,
    });
  });
  it("Is take", async () => {});
});
