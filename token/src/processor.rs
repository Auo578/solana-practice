use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program::{invoke, invoke_signed}, program_pack::Pack, pubkey::Pubkey, system_instruction, sysvar::{rent::Rent, Sysvar}
};

use spl_token::{instruction::{initialize_mint, mint_to}, state::Mint};

use crate::instruction::TokenInstruction;


pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult{
        //读取instruction_data是什么,定义TokenInstruction结构体
        let instructon = TokenInstruction::try_from_slice(instruction_data)?;
        match instructon {
            TokenInstruction::CreateToken { decimals } => Self::create_token(accounts,decimals),
            TokenInstruction::Mint { amount } => Self::mint_token(accounts, amount),
        }
    }

    fn create_token(accounts: &[AccountInfo],decimals:u8) -> ProgramResult{
        //生成account mintaccount
        //初始化account，将它变成mintaccount
        let account_iter = &mut accounts.iter();
        //依照传入的account顺序依次读取
        //这个指令的第一个参数，希望传的mint_account
        //希望传6个地址过来
        let mint_account = next_account_info(account_iter)?;
        let mint_authority = next_account_info(account_iter)?;
        let payer = next_account_info(account_iter)?;
        let rent_sysvar = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;
        
        //在后面启动时有一个solanalks？的命令看到
        msg!("Creating mint account...");
        msg!("Mint account is {}",mint_account.key);
        //写合约，拿到6个账户之后，创建token

        //创建account，因为solana有计算，在链上写入要调用链上指令，调用系统指令
        //调用invoke去crate_account结束
        //通过这段创建一个账户
        let create_account_ix =             &system_instruction::create_account(
            payer.key, 
            mint_account.key, 
            Rent::get()?.minimum_balance(Mint::LEN), 
            Mint::LEN as u64, 
            token_program.key,
        );
        //另一段也可以用变量去截代码
        invoke(
            create_account_ix,
            &[
                mint_account.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
                ],
        )?;

        //创建完账户后，使其初始化变成mint_account
        //指令
        let mint_init_ix = &initialize_mint(
            token_program.key, 
            mint_account.key, 
            mint_authority.key, 
            None, 
            decimals
        )?;
        //调用
        msg!("initialize_mint account...");
        //创建mint账户是不需要种子的
        invoke_signed(
            mint_init_ix, 
            &[
                mint_account.clone(),
                rent_sysvar.clone(),
                token_program.clone(),
                mint_authority.clone(),
                ], 
            &[],
        )?;

        msg!("SPL Token Mint create success!");
            Ok(())
    }

    fn mint_token(accounts: &[AccountInfo],amount:u64)-> ProgramResult{
        let account_iter = &mut accounts.iter();
        //调用其他合约，其他合约的地址的一个必要参数
        //这个一共需要7个参数,不全也行，不全在合约里直接拿
        let mint_account = next_account_info(account_iter)?;
        let associated_token_account = next_account_info(account_iter)?;
        let rent_sysvar = next_account_info(account_iter)?;
        let payer = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;
        let associated_token_program = next_account_info(account_iter)?;
        
        msg!("ATA is : {:?}",associated_token_account);
        //判断ATA账户有没有生成，根据余额判断
        if associated_token_account.lamports() == 0{
            msg!("Creating assocaited token account....");

            //创建ATA，首先创建一个指令
            let create_ata_ix = &spl_associated_token_account::instruction::create_associated_token_account(
                payer.key, 
                payer.key, 
                mint_account.key, 
                token_program.key,
            );

            //调用invoke创建ATA账户
            //account_infos调用的参数相对==相当于参数里指令的所有参数，create_associated_token_account
            invoke(
                create_ata_ix, 
                &[
                    payer.clone(),
                    associated_token_account.clone(),
                    mint_account.clone(),
                    //系统程序是 Solana 中创建账户的基础程序，用于处理创建和分配新账户的操作。
                    system_program.clone(),
                    token_program.clone(),
                    //rent_sysvar 是一个系统变量，指示 Solana 网络的当前租金费用。
                    rent_sysvar.clone(),
                    //associated_token_program 是专门用于处理关联代币账户（ATA）创建的程序。
                    associated_token_program.clone(),
                ],
            )?;
            
        };

        msg!("Minting {} tokens to ata...",amount);
        //这里调用invoke去真正的mint_token
        
        let mint_ix = &mint_to(
            token_program.key, 
            mint_account.key, 
            //要给哪个账户去mint,ata账户
            associated_token_account.key, 
            //负责支付交易费用的账户，它通常也是执行交易操作的账户
            payer.key, 
            //需要签名的，传的是一个切片
            &[payer.key], 
            amount,
        )?;
        invoke(
            mint_ix, 
            &[
                mint_account.clone(),
                payer.clone(),
                associated_token_account.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("Tokens Minted tokens to ata success");
        Ok(())
    }

}