use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize,BorshSerialize};



#[derive(BorshDeserialize,BorshSerialize,Debug)]
pub enum SocialInstruction{
    //初始化不同类型的账户
    InitializeUser{seed_type: String},
    //关注
    FollowUser{user_to_follow: Pubkey},
    //取消关注
    UnfollowUser{user_to_unfollow: Pubkey},
    // 查询关注
    QueryFollow,
    //发帖
    PostContent{content: String},
    //查询发帖
    QueryPosts,
}