use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
//存的数据写到state里
use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize,BorshSerialize};
use solana_sdk::{instruction::{AccountMeta, Instruction}, signature::{read_keypair_file, Keypair}, signer::Signer, transaction::Transaction};


#[derive(Debug,BorshDeserialize,BorshSerialize)]

//关注只生成一个账户去关注
pub struct UserProfile{
    //可加可不加
    //在后面反序列化可用可不用
    pub data_len: u16,
    //本质要follows
    pub follows: Vec<Pubkey>,
}

//发帖用另一种方式，发一个帖子生成一个PDA账户
//用另一种方式存储，就是发一个用一个账户
//生成PDA账户需要一个seed，模拟出来一个mansoco的zhizunid？
#[derive(Debug,BorshDeserialize,BorshSerialize)]
pub struct UserPost{
    pub post_count: u64,
}


#[derive(Debug,BorshDeserialize,BorshSerialize)]
pub struct Post{
    pub content: String,
    pub timesrtamp :u64,
}

//存储的数据都在以上了

impl UserProfile{
    pub fn new() -> Self{
        Self{
            data_len:0,
            follows: Vec::new(),
        }
    }

    pub fn follow(&mut self,user:Pubkey){
        self.follows.push(user);
        //重新计算长度给data_len加过去
        self.data_len = self.follows.len() as u16;
    }
}




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



const  USER_PROFILE_SEED:&str = "profile"; 
const USER_POST_SEED:&str = "post";

pub struct  SocialClient{
    rpc_client: RpcClient,
    program_id: Pubkey,
}

impl  SocialClient {
    pub fn new(rpc_url:&str,program_id: Pubkey) -> Self{
        let rpc_client = RpcClient::new(rpc_url.to_string());

        Self { rpc_client, program_id }
    }

    //创建账号
    pub fn initialize_user(&self,user_keypair:&Keypair,seed_type:&str) -> Result<(),Box<dyn std::error::Error>>{
        let pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),seed_type.as_bytes()]);
        
        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::InitializeUser { seed_type: seed_type.to_string() }, 
            vec![
                AccountMeta::new(user_keypair.pubkey(), true),
                //可写账户
                AccountMeta::new(pda, false),
                //只读账户
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
            // let user_account = next_account_info(account_info_iter)?;
            // let pda_account = next_account_info(account_info_iter)?;
            // let system_program = next_account_info(account_info_iter)?;
        );
        //  发送命令
        self.seed_instruction(user_keypair, vec![initialize_user_instruction])?;

        Ok(())
    }

    //关注

    pub fn follow_user(&self,user_keypair:&Keypair,follow_user: Pubkey) -> Result<(),Box<dyn std::error::Error>>{
        let pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),USER_PROFILE_SEED.as_bytes()]);
        
        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::FollowUser { user_to_follow: follow_user} , 
            vec![
                AccountMeta::new(pda, false),
            ],
        );
        //  发送命令
        self.seed_instruction(user_keypair, vec![initialize_user_instruction])?;
        
        
        Ok(())
    }

    //查询账户信息（关注数据）

    pub fn query_followers(&self,user_keypair:&Keypair) -> Result<(),Box<dyn std::error::Error>> {
        let pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),USER_PROFILE_SEED.as_bytes()]);
        
        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::QueryFollow  , 
            vec![
                AccountMeta::new(pda, false),
            ],
        );
        //  发送命令
        self.seed_instruction(user_keypair, vec![initialize_user_instruction])?;
        
        
        Ok(())
    }

    //取消关注
    pub fn unfollow_user(
        &self,
        user_keypair:&Keypair,
        unfollow_user:Pubkey,
    ) -> Result<(),Box<dyn std::error::Error>> {
        
        let pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),USER_PROFILE_SEED.as_bytes()]);
        
        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::UnfollowUser { user_to_unfollow: unfollow_user } , 
            vec![
                AccountMeta::new(pda, false),
            ],
        );
        //  发送命令
        self.seed_instruction(user_keypair, vec![initialize_user_instruction])?;
        
        Ok(())
    }


    //创建帖子
    pub fn post_content(&self,user_keypair:&Keypair,content:String,id:u64) -> Result<(),Box<dyn std::error::Error>> {
        
        let pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),USER_POST_SEED.as_bytes()]);
        let post_pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),USER_POST_SEED.as_bytes(),&[id as u8]]);
        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::PostContent { content: content }  , 
            vec![
                //要付钱，要授权pda
                AccountMeta::new(user_keypair.pubkey(), true),
                AccountMeta::new(pda, false),
                AccountMeta::new(post_pda, false),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ],
        );
        //  发送命令
        self.seed_instruction(user_keypair, vec![initialize_user_instruction])?;


        Ok(())
    }

    //查询帖子
    pub fn query_post(&self,user_keypair:&Keypair,id:u64) -> Result<(),Box<dyn std::error::Error>>{
        
        
        let pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),USER_POST_SEED.as_bytes()]);
        let post_pda = get_pda(&self.program_id, &[user_keypair.pubkey().as_ref(),USER_POST_SEED.as_bytes(),&[id as u8]]);
        let initialize_user_instruction = Instruction::new_with_borsh(
            self.program_id,
            &SocialInstruction::QueryPosts, 
            vec![
                //要付钱，要授权pda
                AccountMeta::new(pda, false),
                AccountMeta::new(post_pda, false),
            ],
        );
        //  发送命令
        self.seed_instruction(user_keypair, vec![initialize_user_instruction])?;
        
        Ok(())
    }

    //发送命令
    pub fn seed_instruction(
        &self,
        payer:&Keypair,
        instruction: Vec<Instruction>,
    ) -> Result<(),Box<dyn std::error::Error>>{
        let last_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instruction, 
            Some(&payer.pubkey()), 
            &[payer], 
            last_blockhash,
        );
        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        
        println!("send_and_confirm_transaction success {}",signature);
        Ok(())
    }


}

fn get_pda(program_id: &Pubkey,seed: &[&[u8]]) -> Pubkey{
    //program_id已经引用了，还要加一个&号？
    let (pda,_bump) = Pubkey::find_program_address(seed, &program_id);
    println!("pda:{:?}",pda);
    pda

}

impl UserPost {
    pub fn new() -> Self{
        Self{
            post_count:0,
        }
    }

    pub fn add_post(&mut self) {
        self.post_count += 1;
    }

    pub fn get_count(&mut self) -> u64 {
        self.post_count 
    }
}



fn main() -> Result<(),Box<dyn std::error::Error>>{
    // let user_profile = UserProfile::new();
    //序列化完之后看长度
    //可以看到user_profile序列完之后是6
    // println!("user_profile len is {:?}",borsh::to_vec(&user_profile).unwrap().len());

    //部署合约之后的Pubkey,合约地址             
    let program_id = Pubkey::from_str("93qgSG4QkVn8vcVg4x7PHsBLSWrY8HTHzYV8i4ExjN6H")?;

    let user_keypair = read_keypair_file("/root/.config/solana/t1.json").expect("failed");

    let client = SocialClient::new("http://127.0.0.1:8899 ", program_id);
    //测试post数据大小
    // let user_post = UserPost::new();
    // println!("user_post len is {:?}",borsh::to_vec(&user_post).unwrap().len());

    // //创建初始化user_profile账户
    // client.initialize_user(&user_keypair,USER_PROFILE_SEED)?;

    // //关注用户
    // // LAPTOP-6M4RE4LA# solana address -k /root/.config/solana/t2.json 
    // // DST3zCqEgzmHXZU3fyksVuPeSdpzpjBfkds58KrbKtKC
    // //关注的用户
    // let follow_user = Pubkey::from_str("DST3zCqEgzmHXZU3fyksVuPeSdpzpjBfkds58KrbKtKC")?;
    // //调用关注的方法
    // client.follow_user(&user_keypair,follow_user)?;

    // //查询账户信息（关注数据）
    // client.query_followers(&user_keypair)?;

    // //取消关注
    // client.unfollow_user(&user_keypair,follow_user)?;

    // //查询账户信息（是否取消关注数据）
    // client.query_followers(&user_keypair)?;

    // 创建userpost账户
    client.initialize_user(&user_keypair,USER_POST_SEED)?;

    //发送帖子
    let mut content = "hello solana, id:1".to_string();
    let mut id = 1;
    client.post_content(&user_keypair,content,id)?;
    //查询帖子
    client.query_post(&user_keypair,id)?;
    //再发送一个
    content = "hello solana, id:2".to_string();
    id = 2;
    client.post_content(&user_keypair,content,id)?;

    //查询帖子
    client.query_post(&user_keypair,id)?;


    // 获取合约中的所有账户信息
    // let accounts = client.rpc_client.get_program_accounts(&program_id).unwrap();
    // for (pubkey,account) in  accounts.iter(){
    //     println!("pubkey:{:?}",pubkey);

    //     println!("account: {:?}", account);

    //     println!("--------------");
    // }

    Ok(())
}
