use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, clock::Clock, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar};
use crate::instruction::*;
use crate::state::*;

const PUBKEY_SIZE: usize=32;
const U16_SIZE:usize=2;
const USER_PROFILE_SIZE: usize=6;

//一个PDA账户最多能关注200个
const MAX_FOLLOWER_COUNT :usize=200;

const USER_POST_SIZE: usize = 8;
pub struct  Processor;

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult{
        let instruction = SocialInstruction::try_from_slice(instruction_data)?;
        match instruction {
            SocialInstruction::InitializeUser { seed_type } =>{
                Self::initialize_user(program_id, accounts, seed_type)
            }

            SocialInstruction::FollowUser { user_to_follow } => {
                Self::follow_user(accounts,user_to_follow)
            }
            //  查询可以不写，因为其可以通过账户去查，不需要通过合约去查
            SocialInstruction::QueryFollow => {
                Self::query_follows(accounts)
            }
            //写完上面三个方法了，写客户端
            SocialInstruction::UnfollowUser { user_to_unfollow } => {
                Self::unfollow_user(accounts,user_to_unfollow)
            }

            SocialInstruction::PostContent { content } => {
                Self::post_content(program_id,accounts,content)
            }

            SocialInstruction::QueryPosts => {
                Self::query_post(accounts)
            }
        }
    }
    //先写follow，先生成一个PDA，生成USERfolloerfile去存储关注的人
    //通过seed生成PDA，通过PDA存储关注的人
    //初始化账户
    fn initialize_user(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        //solana里的原则是一个账户存一个信息
        //可以生成多个PDA账户
        //通过生成profile ，post根据seed_type去区分生成profile还是post
        seed_type: String
    ) -> ProgramResult{

        let account_info_iter = &mut accounts.iter();
        let user_account = next_account_info(account_info_iter)?;
        let pda_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        

        //根据不同的seed去初始化不同的PDA账户
        let seed = match seed_type.as_str() {
            "profile" => "profile",
            "post" => "post",
            _ => return Err(ProgramError::InvalidArgument) ,
        };

        msg!("seed:{:?}",seed);
        //拿到种子之后生成PDA账户
        let (pda,bump_seed) = Pubkey::find_program_address(&[user_account.key.as_ref(),seed.as_bytes()], program_id);

        msg!("pda:{:?}",pda);

        if pda != pda_account.key.clone(){
            return Err(ProgramError::InvalidArgument);
        }
        //租金
        let rent = Rent::get()?; 
        //计算租金的大小，空间大小
        let space = match seed_type.as_str() {
            "profile" => computer_profile_space(MAX_FOLLOWER_COUNT),
            "post" => USER_POST_SIZE,
            _ => return Err(ProgramError::InvalidArgument) ,
        };
        //计算租金
        let lamports = rent.minimum_balance(space);
        //创建账户指令
        let creat_account_ix = system_instruction::create_account(
            user_account.key, 
            &pda, 
            lamports, 
            space as u64, 
            program_id,
        );

        invoke_signed(
            &creat_account_ix, 
            &[user_account.clone(),pda_account.clone(),system_program.clone()], 
            &[&[user_account.key.as_ref(),seed.as_bytes(),&[bump_seed]]],
        )?;

        //创建账户后根据type初始化账户里的data
        //拿到data后塞到user_profile里
        match seed_type.as_str() {
            "profile" => {
                let user_profile = UserProfile::new();
                //把账户写进去了
                user_profile.serialize(&mut *pda_account.try_borrow_mut_data()?)?
            
            
            },
            "post" => {
                //给user_post这个seed生成一个pda账户
                let user_post = UserPost::new();
                //把数据存进去
                user_post.serialize(&mut *pda_account.try_borrow_mut_data()?)?
            },
            _ => return Err(ProgramError::InvalidArgument) ,
        };

        msg!("User init success!");

        Ok(())

    }


    fn follow_user(accounts:&[AccountInfo],user_to_follow:Pubkey) -> ProgramResult{
        let account_info_iter = &mut accounts.iter();
        let pda_account = next_account_info(account_info_iter)?;

        //size = USERprofile的size，里的data_len
        //根据这个截断，之后才能被序列化，直接序列化会报错
        //space里会用0占位，反序列的时候会用0报错，显示数据不完整
        //这个就是计算要拿到多少长度的
        let mut  size: usize = 0;
        {
            //拿括号是因为follow会更改，借用和可变借用不能同时存在
            //拿data的数据
            let data = &pda_account.data.borrow();
        
            let len = &data[..U16_SIZE];

            let pubkey_count = bytes_to_u16(len).unwrap();
            size = computer_profile_space(pubkey_count as usize);
            msg!("size is {:?}",size);
        
        }

        //拿原来的数据反序列化
        //第一种，序列化从0开始到哪一个阶段为止
        //这样写一定会报错
        // let mut user_profile = UserProfile::try_from_slice(&user_account.data.borrow());
        //要截断到它存的最后一位
        //比较麻烦
        let mut user_profile = UserProfile::try_from_slice(&pda_account.data.borrow()[..size])?;
        
        msg!("user_profile is {:?}",user_profile);

        //封装一个方法

        user_profile.follow(user_to_follow);

        //序列化回来
        user_profile.serialize(&mut *pda_account.try_borrow_mut_data()?)?;
        Ok(())
    }

    fn query_follows(accounts: &[AccountInfo]) -> ProgramResult{
        let account_info_iter = &mut accounts.iter();
        let pda_account = next_account_info(account_info_iter)?;
        
        //第二种更好更简单的写法
        //这个方法是个宏
        //官方：不检查，上一个安全性更高
        let user_profile = try_from_slice_unchecked::<UserProfile>(&pda_account.data.borrow()).unwrap();

        msg!("user_profile is {:?}",user_profile);
        
        Ok(())
    }

    fn unfollow_user(accounts: &[AccountInfo],user_to_unfollow:Pubkey) -> ProgramResult{
        let account_info_iter = &mut accounts.iter();
        let pda_account = next_account_info(account_info_iter)?;
        


        //不加[..size]报错
        // let mut user_profile = UserProfile::try_from_slice(&pda_account.data.borrow()[..size])?;
        let mut user_profile = try_from_slice_unchecked::<UserProfile>(&pda_account.data.borrow()).unwrap();

        user_profile.unfollow(user_to_unfollow);


        //另一种写法其实本质和其一样
        //let user_profile = try_from_slice_unchecked::<UserProfile>(&pda_account.data.borrow()).unwrap();
        user_profile.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;



        Ok(())
    }

    fn post_content(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            content: String
        ) -> ProgramResult{
            let account_info_iter = &mut accounts.iter();
            let user_account = next_account_info(account_info_iter)?;
            let pda_account = next_account_info(account_info_iter)?;
            let post_pda_account = next_account_info(account_info_iter)?;
            let system_program = next_account_info(account_info_iter)?;
            
            //从区块链上拿时间
            let clock = Clock::get()?;
            let timestamp = clock.unix_timestamp as u64;
            
            //
            let mut user_post = try_from_slice_unchecked::<UserPost>(&pda_account.data.borrow())?;
            
            //等同于id增长
            user_post.add_post();

            //回写user_post账户数据
            user_post.serialize(& mut *pda_account.try_borrow_mut_data()?)?;

            //获取最新id
            let count = user_post.get_count();

            //创建pda账户
            let (pda,bump_seed) = Pubkey::find_program_address(
                &[user_account.key.as_ref(),"post".as_bytes(),&[count as u8]], 
                program_id,
            );

            //创建帖子数据
            let post = Post::new(content,timestamp);

            let rent = Rent::get()?;

            //创建账户需要space,巨大化

            let space = borsh::to_vec(&post).unwrap().len();
            //拿到最小的租金
            let lamports = rent.minimum_balance(space);
            
            //创建账户指令
            let creat_account_ix = system_instruction::create_account(
                user_account.key, 
                &pda, 
                lamports, 
                space as u64, 
                program_id,
            );

            invoke_signed(
                &creat_account_ix, 
                &[user_account.clone(),post_pda_account.clone(),system_program.clone()], 
                &[&[user_account.key.as_ref(),"post".as_bytes(),&[count as u8],&[bump_seed]]],
            )?;
            //将帖子数据写入账户
            post.serialize(&mut *post_pda_account.try_borrow_mut_data()?)?;
            
            Ok(())
        }


      fn query_post(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pda_account = next_account_info(account_info_iter)?;
        let pda_post_account = next_account_info(account_info_iter)?;

        let user_post = try_from_slice_unchecked::<UserProfile>(&pda_account.data.borrow()).unwrap();
        
        msg!("user_post:{:?}",user_post);

        let post = try_from_slice_unchecked::<Post>(&pda_post_account.data.borrow()).unwrap();
        msg!("post_content:{:?} at {:?}",post.content,post.timestamp);
        Ok(())
      }  
    
}

fn computer_profile_space(pubkey_count: usize) -> usize{
    //最终的总内存大小是将用户配置文件的大小和存储所有公钥所需要的内存加在一起。
    //pubkey_count 表示公钥的数量，PUBKEY_SIZE 表示每个公钥的字节数。
    return USER_PROFILE_SIZE+pubkey_count  * PUBKEY_SIZE
}

fn bytes_to_u16(bytes: &[u8]) -> Option<u16>{
    if bytes.len() != 2{
        return None;//确保输入是16个字节
    }

    let mut array = [0u8;2];
    array.copy_from_slice(bytes);
    Some(u16::from_be_bytes(array))
}