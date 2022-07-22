use std::fmt::Display;
use rand::{random, Rng, thread_rng};

pub trait SocialPlatform {
    fn social_info(&self) -> String;
    fn send_msg(&self) -> String {
        format!("holy shit")
    }
}

pub struct WeChat {
    pub nickname: String,
    pub wx_id: String,
    //wxid_ + 14位初始 小写字母与数字混合
    pub district: String,
    pub gender: String,
    pub about: String,
}

impl SocialPlatform for WeChat {
    fn social_info(&self) -> String {
        format!("nickname: {}, wx_id: {}, district: {}, gender: {}, about: {}",
                self.nickname, self.wx_id, self.district, self.gender, self.about)
    }
    fn send_msg(&self) -> String {
        format!("msg by {}", self.nickname)
    }
}

// 使用特征作为函数参数
pub fn show_msg(item: &impl SocialPlatform) {
    println!("NEW MESSAGE: {}", item.send_msg())
}

// 特征约束(trait bound) T: SocialPlatform
pub fn show_msg_all<T: SocialPlatform>(item1: &T, item2: &T) {
    println!("NEW MESSAGE: item1: {}, item2: {}", item1.send_msg(), item2.send_msg())
}

pub fn multi_bound_use1(item: &(impl SocialPlatform + Display)) {
    println!("NEW MESSAGE: {}", item.send_msg())
}

pub fn multi_bound_use2<T: SocialPlatform + Display>(item: &T) {
    println!("NEW MESSAGE: {}", item.send_msg())
}

// where 约束 不再展开

pub(crate) fn random_use() {
    let mut rng = thread_rng();

    // Arrays (up to 32 elements): each element is generated sequentially;
    // see also Rng::fill which supports arbitrary array length for integer types and
    // tends to be faster for u32 and smaller types.
    let mut arr2 = [0u8; 2];
    rng.fill(&mut arr2);

    dbg!(arr2);
}

// 仅包含数字0-9和字母a-z 长度36
fn random_string(len: usize) -> String {

    // 65-90 A-Z
    // 97-122 a-z
    // 纯字母是 3*len 的总长度, len为14 即总长42位
    // 纯数字是 1*len 的总长度, len为14 即总长14位

    let mut ascii_value: [u8; 36] = [0; 36];
    for i in 0..=9 {
        ascii_value[i] = i as u8;
    }
    for i in 10..=35 {
        ascii_value[i] = (i + 87) as u8;
    }

    let mut rng = thread_rng();

    let mut vec_index: Vec<_> = Vec::new();
    for _ in 0..=len {
        vec_index.push(rng.gen_range(0..36));
    }

    let mut res = "".to_string();
    for item in vec_index {
        res += &ascii2string(ascii_value[item]).to_string()
    }

    dbg!(&res);
    res.to_string()
}

fn ascii2string(v: u8) -> String {
    let supper_alpha = [
        "A", "B", "C", "D", "E", "F", "G",
        "H", "I", "J", "K", "L", "M", "N",
        "O", "P", "Q", "R", "S", "T",
        "U", "V", "W", "X", "Y", "Z"
    ];
    let lower_alpha = [
        "a", "b", "c", "d", "e", "f", "g",
        "h", "i", "j", "k", "l", "m", "n",
        "o", "p", "q", "r", "s", "t",
        "u", "v", "w", "x", "y", "z"
    ];

    match v {
        0..=9 => v.to_string(),
        65..=90 => supper_alpha[(v - 65) as usize].to_string(),
        97..=122 => lower_alpha[(v - 97) as usize].to_string(),
        _ => {
            println!("waiting");
            "".to_string()
        }
    }
}

impl WeChat {
    fn new(nickname: String, district: String, gender: String, about: String) -> WeChat {
        WeChat {
            nickname,
            wx_id: "wxid_".to_owned() + &random_string(14),
            district,
            gender,
            about,
        }
    }
}

pub struct QQ {
    pub nickname: String,
    pub qq_number: u32,
    pub district: String,
    pub gender: String,
    pub about: String,
    pub birthday: String,
}

impl SocialPlatform for QQ {
    fn social_info(&self) -> String {
        format!("nickname: {}, qq_number: {}, district: {}, gender: {}, about: {}, birthday: {}",
                self.nickname, self.qq_number, self.district, self.gender, self.about, self.birthday)
    }
}

impl QQ {
    fn new(nickname: String, district: String, gender: String, about: String, birthday: String) -> QQ {
        QQ {
            nickname,
            qq_number: random::<u32>(),
            district,
            gender,
            about,
            birthday,
        }
    }
}

pub(crate) fn generate_id() {
    let new_qq = QQ::new("holy".to_string(),
                         "UK".to_string(),
                         "male".to_string(),
                         "I am unstoppable".to_string(),
                         "3/2".to_string());
    let new_wx = WeChat::new("nick".to_string(),
                             "Italian".to_string(),
                             "female".to_string(),
                             "gette".to_string());

    println!("new_qq: {}", new_qq.social_info());
    println!("new_wx: {}", new_wx.social_info());

    show_msg(&new_qq);
    show_msg(&new_wx);

    show_msg_all(&new_qq, &new_qq);
}