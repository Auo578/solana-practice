use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct IbuildLike{
    //点赞的人
    pub profile_pubkey: Pubkey,
    //推文
    pub tweet_pubkey: Pubkey,
}

impl IbuildLike{
    pub const SEED_PREFIX: &'static str = "like";

    pub fn new(profile_pubkey:Pubkey,tweet_pubkey:Pubkey) -> Self{
        Self{
            profile_pubkey,
            tweet_pubkey,
        }
    }
}