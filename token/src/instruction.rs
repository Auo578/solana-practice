use borsh::{BorshDeserialize,BorshSerialize};

//一般指令都是enum

#[derive(BorshDeserialize,BorshSerialize)]
pub enum TokenInstruction{
    CreateToken {decimals: u8},
    Mint{amount:u64}
}