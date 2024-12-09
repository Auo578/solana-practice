import * as anchor from "@coral-xyz/anchor"
import { program } from "./wallet"


export async function createProfile(
    wallet:anchor.Wallet,
    displayName:string,
) {
    //调用合约,创建了一个profile
    return await program.methods.createProfile(displayName).accounts({
        authority:wallet.publicKey
    })
    .signers([wallet.payer])
    .rpc();
}

//查询账户
export async function getProfile(
    wallet:anchor.Wallet,
) {
    //派生pda
    const [profilePda,] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), wallet.publicKey.toBuffer()],
        program.programId,
    );

    return await program.account.ibuildProfile.fetch(profilePda);
}