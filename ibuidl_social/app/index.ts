//在外侧写，因为需要放到anchor的执行环境里去执行
//所以直接在anchor.toml里增加一行命令

import { createProfile, getProfile } from "./api/profile";
import { createTokenMintAccount } from "./api/token";
import { createLike, createTweet, getTweet } from "./api/tweet";
import { program, useDefaultWallet, useVisitorWallet } from "./api/wallet"
import * as anchor from "@coral-xyz/anchor";

(async () => {
    const defaultWallet = useDefaultWallet();
    const visitorWallet = useVisitorWallet();

    // try{
    //     const r1 = await createProfile(defaultWallet, "Bob");
    //     console.log(r1);
    
    //     const r2 = await getProfile(defaultWallet);
    //     console.log(r2);

    //     const r3 = await createProfile(visitorWallet, "Alice");
    //     console.log(r3);
    
    //     const r4 = await getProfile(visitorWallet);
    //     console.log(r4);
    // }catch(e){
    //     console.log(e);
    // }


    // const [pda,r3] = await createTweet(defaultWallet,"hello world");
    // console.log(r3);

    // const r4 = await getTweet(defaultWallet, pda);
    // console.log(r4);

    // const r5 = await createLike(visitorWallet,pda);
    // console.log(r5);

    // const r6 = await getTweet(defaultWallet,pda);
    // console.log(r6);

    // // const r7 = await createLike(visitorWallet,pda);
    // // console.log(r5);

    const [tokenPda,r] = await createTokenMintAccount(defaultWallet);
    console.log(tokenPda.toString(),r);
})()