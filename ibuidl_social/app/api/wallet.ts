import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor";
import { IbuidlSocial } from "../../target/types/ibuidl_social";

//初始化本地的环境
//与 Solana 网络连接
let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.IbuidlSocial as Program<IbuidlSocial>;

export{program,provider}


//默认本地钱包
export function useDefaultWallet(){
    return anchor.Wallet.local();
}

//Pubkey:99BzfSaGtdnijEaCocg89FCSAju5uMEdUgDp361t58g5
export function useVisitorWallet() {
    // const keypair = anchor.web3.Keypair.fromSecretKey(
    //     //可以在本地生成一个新的，或者用代码生成
    //     //私钥
    //     new Uint8Array(
    //         [
    //         63,80,205,46,33,163,47,194,
    //         107,223,17,122,178,82,34,153,
    //         42,153,153,188,50,170,147,41,
    //         39,162,113,72,71,148,115,49,
    //         120,246,146,179,85,40,198,36,
    //         6,234,43,234,44,168,38,142,
    //         158,31,116,228,68,116,111,50,
    //         39,136,37,105,253,45,182,230
    //     ])
    // );

    const keypair = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array([
            63,80,205,46,33,163,47,194,
            107,223,17,122,178,82,34,153,
            42,153,153,188,50,170,147,41,
            39,162,113,72,71,148,115,49,
            120,246,146,179,85,40,198,36,
            6,234,43,234,44,168,38,142,
            158,31,116,228,68,116,111,50,
            39,136,37,105,253,45,182,230
        ])
    );

    //用私钥创建钱包的结构
    return new anchor.Wallet(keypair);
}

