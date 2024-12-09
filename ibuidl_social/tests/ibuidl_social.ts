import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IbuidlSocial } from "../target/types/ibuidl_social";

describe("ibuidl_social", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  //加载和交互程序 使用 IDL（Interface Description Language）文件连接 Anchor 程序
  const program = anchor.workspace.IbuidlSocial as Program<IbuidlSocial>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
