use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IbuildTweet {
    pub like_count: u64,
    #[max_len(50)]
    body: String,
}

impl IbuildTweet {
    pub const SEED_PREFIX: &'static str = "tweet";

    pub fn new(body: String) -> Self {
        Self {
            body,
            like_count: 0,
        }
    }
}
