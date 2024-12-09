use anchor_lang::prelude::*;


pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("77w5CKC886cTLVabH7g25xDqiwm5o1mxcjnWdwUTnkKe");

#[program]
pub mod ibuidl_social {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>,display_name: String) -> Result<()>{
        instructions::profile::create_profile(ctx, display_name)
        
    }

    //创建帖子
    pub fn create_tweet(ctx: Context<CreatTweet>,body: String) -> Result<()>{
        instructions::tweet::create_tweet(ctx, body)
    }

    //点赞功能
    pub fn create_like(ctx: Context<CreatLike>) -> Result<()>{
        instructions::tweet::create_like(ctx)
    }
    //创建mint_account, metadata_account
    pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()>{
        instructions::token::create_token_mint_account(ctx)
    }
}






