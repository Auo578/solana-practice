use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct IbuildProfile{
    pub tweet_count: u32,
    #[max_len(20)]
    pub display_name: String,
}

impl  IbuildProfile{
    pub const SEED_PREFIX: &'static str = "profile";
} 