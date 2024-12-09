use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    }, 
        token::{Mint, Token},
};
pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()>{
    
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"mint_v9",
        &[ctx.bumps.mint_account],
    ]];
    //向metadata合约发出一次cpi的调用
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            //要调用谁
            ctx.accounts.token_metadata_program.to_account_info(), 
            CreateMetadataAccountsV3{
                metadata:ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                mint_authority: ctx.accounts.mint_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.mint_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            }, 
            signer_seeds,
        ), 
        DataV2{
            name: "ibuid;".to_string(),
            symbol: "IBUIDL".to_string(),
            uri: "http://ibuidl.com".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, 
        true, 
        None,
    )?;
    
    Ok(())
}
    
#[derive(Accounts)]
pub struct CreateTokenMintAccount<'info>{
    //创建metadata account
    /// CHECK: Validate address by derving pda
    #[account(
        mut,
        //seeds是固定的
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint_account.key().as_ref(),
            
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,


    //创建mint账户
    #[account(
        //帮助创建mint_account
        init_if_needed,
        payer = authority,
        seeds = [
            b"mint_v9",
        ],
        bump,
        mint::decimals = 100,
        //谁有权限Mint这个token
        //按当前账户设置来看谁创建了这个token谁就有权限mint
        //后面点赞的时候，合约要给作者进行mint，所以要让合约具有mint的权限
        // mint::authority = authority,弃用
        //使用合约的地址进行mint，合约自身就有权限mint
        mint::authority = mint_account.key(),
    )]
    pub mint_account:Box<Account<'info,Mint>>,
    
    #[account(mut)]
    pub authority:Signer<'info>,

    pub system_program:Program<'info,System>,
    pub token_program: Program<'info,Token>,
    pub token_metadata_program:Program<'info,Metadata>,

    pub rent: Sysvar<'info,Rent>,
}

