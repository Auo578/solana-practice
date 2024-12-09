use anchor_lang::prelude::*;

use crate::state::{like::IbuildLike, profile::IbuildProfile, tweet::IbuildTweet};


pub fn create_tweet(ctx: Context<CreatTweet>,body: String) -> Result<()>{
  let profile = &mut ctx.accounts.profile;
  profile.tweet_count += 1;

  let tweet = IbuildTweet::new(body);
  ctx.accounts.tweet.set_inner(tweet.clone());
  
  Ok(())
}

#[derive(Accounts)]
pub struct CreatTweet<'info>{
    #[account(
        init,
        payer = authority,
        space = IbuildTweet::INIT_SPACE,
        seeds = [
        IbuildTweet::SEED_PREFIX.as_bytes(),
        profile.key().as_ref(),
        (profile.tweet_count + 1).to_string().as_ref(),
        ],
        bump,
    )]
    pub tweet: Account<'info, IbuildTweet>,

    #[account(
        mut,
        seeds = [
            IbuildProfile::SEED_PREFIX.as_bytes(),
            authority.key().as_ref()
            ],
        bump,
    )]
    pub profile: Account<'info, IbuildProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info,System>,

}


pub fn create_like(ctx: Context<CreatLike>) -> Result<()>{
    let tweet = &mut ctx.accounts.tweet; 
    tweet.like_count += 1 ;

    let like_rel = IbuildLike::new(ctx.accounts.profile.key(), tweet.key());
    ctx.accounts.like.set_inner(like_rel);
    
    Ok(())
}

//指明实现这个指令所依赖的账户

#[derive(Accounts)]
pub struct CreatLike<'info>{
    //记录点赞的关系
    #[account(
        init,
        payer = authority,
        space = 8 + IbuildLike::INIT_SPACE,
        seeds = [
            IbuildLike::SEED_PREFIX.as_bytes().as_ref(),
            //点赞的人的
            profile.key().as_ref(),
            //推文的
            tweet.key().as_ref(),
        ],
        bump,
    )]
    pub like: Account<'info, IbuildLike>,


   //被点赞的推文
    #[account(mut)]
    pub tweet: Account<'info, IbuildTweet>, 
    
    //点赞的人
    #[account(
        mut,
        seeds = [
            IbuildProfile::SEED_PREFIX.as_bytes(),
            authority.key().as_ref()
            ],
        bump,
    )]
    pub profile: Account<'info,IbuildProfile>,

    //签署交易的账户
    #[account(mut)]
    pub authority:Signer<'info>,
    pub system_program: Program<'info,System>,

}