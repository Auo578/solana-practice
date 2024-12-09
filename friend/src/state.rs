//存的数据写到state里
use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize,BorshSerialize};


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
    pub timestamp :u64,
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

    pub fn unfollow(&mut self,user_to_unfollow:Pubkey) {
        self.follows.retain(|&x| x != user_to_unfollow);
        self.data_len = self.follows.len() as u16;
    }
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

impl Post {
    pub fn new(content: String, timestamp:u64) -> Self {
        Self { content, timestamp }
    }
}


// //等同于id增长
// user_post.add_post();

// //回写user_post账户数据
// user_post.serialize(& mut *pda_account.try_borrow_mut_data()?)?;

// //获取最新id
// let count = user_post.get_count();

// //创建pda账户
// let (pda,bump_seed) = Pubkey::find_program_address(
//     &[user_account.key.as_ref(),"post".as_bytes(),&[count as u8]], 
//     program_id,
// );

// //创建帖子数据
// let post = Post::new(content,timestamp);