import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createTweet(
  wallet: anchor.Wallet,
  body: string
): Promise<[anchor.web3.PublicKey, string]> {
  const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const profile = await program.account.ibuildProfile.fetch(profilePda);

  const [tweetPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tweet"),
      profilePda.toBuffer(),
      Buffer.from(`${profile.tweetCount + 1}`),
    ],
    program.programId
  );
  //修改string

  return [
    tweetPda,
    await program.methods
      .createTweet(body)
      .accounts({
        authority: wallet.publicKey,
        tweet: tweetPda,
      })
      .rpc(),
  ];
}

export async function getTweet(
  wallet: anchor.Wallet,
  tweetPda: anchor.web3.PublicKey
) {
  return await program.account.ibuildTweet.fetch(tweetPda);
}

export async function createLike(
  wallet: anchor.Wallet,
  tweetPda: anchor.web3.PublicKey
) {
  return await program.methods
    .createLike()
    .accounts({
        tweet:tweetPda,
        authority:wallet.publicKey,
    })
    .signers([wallet.payer])
    .rpc();
}
