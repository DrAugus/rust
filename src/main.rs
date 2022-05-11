const MAX_DAMAGE: u64 = 21_474_836_473;
const MAX_LEVEL: u8 = 90;

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好！";
    let english = "World, hello!";
    let italian = "mondo, Ciao!";
    let french = "Le monde, Bonjour!";
    let russian = "мир, Привет!";
    let korean = "세계, 안녕하세요!";
    let german = "Welt, Hallo!";
    let regions = [
        southern_germany,
        chinese,
        english,
        italian,
        french,
        russian,
        korean,
        german,
    ];
    for region in regions {
        println!("{}", &region);
    }
}

fn small_scale_chopper() {
    let penguin_data = "\
common name,length (cm)
Little penguin,33
Yellow-eyed penguin,65
Fiordland penguin,60
Invalid,data
";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length)
        }
    }
}

fn variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // let y = 3; // 未使用未以_开头 会警告
    let _z = 44; // 下划线作为变量名的开头 rust不会警告其未使用

    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    println!("{}", 13.14_f32.round());

    // 堆上的 String
    let s1 = String::from("darling");
    let s2 = s1;
    println!("s1 = {:?}, s2 = {:?}", "null", s2);

    let s3: &str = "hello";
    let s4 = s3;
    println!("s4 = {:?},  s3 = {:?}", s4, s3);

    let s5 = String::from("ky");
    let s6 = s5.clone(); // 使用 clone 会极大的降低程序性能，需要小心使用
    println!("s5 = {:?}, s6 = {:?}", s5, s6);

    // 同一作用域，特定数据只能有一个可变引用；
    let mut str_mut_1 = String::from("hi");
    {
        let temp_str_mut = &mut str_mut_1;
        println!("temp_m_s = {:?}", temp_str_mut);
    }
    let str_mut_r1 = &mut str_mut_1;
    // let str_mut_r2 = &mut str_mut_1;
    println!(
        "str_mut_r1: {}, str_mut_r2: {}",
        str_mut_r1, "str_mut_r2 has borrowed"
    );

    // 或者任意多个不可变引用
    let mut str_mut_2 = String::from("Bonne");
    let str_r1 = &str_mut_2;
    let str_r2 = &str_mut_2;
    // let str_r3 = &mut str_mut_2;
    println!(
        "str_r1: {}, str_r2: {}, str_r3 :{}",
        str_r1, str_r2, "str_r3 cannot borrow"
    );
    let str_r4 = &mut str_mut_2;
    println!("str_r4: {}, can borrow here", str_r4);

    let i1 = 5;
    let i2 = &i1;
    assert_eq!(i1, *i2);

    let str_plus1 = String::from("hi, ");
    let str_plus2 = "r u ready? ";
    let str_res_plus = str_plus1 + str_plus2;
    dbg!(str_res_plus);
    let str_plus3 = "hi, ";
    let str_plus4 = String::from("r u ready? ");
    let str_res_plus = format!("{}{}WOW", str_plus3, str_plus4);
    dbg!(str_res_plus);
    let str_plus5 = String::from("hi, ");
    let str_plus6 = String::from("r u ready? ");
    let str_res_plus = str_plus5 + &str_plus6;
    dbg!(str_res_plus);
}

fn str_slice() {
    println!("str_slice");
    let s = String::from("Bonne nuit");
    let len = s.len();
    let slice_begin_1 = &s[0..5];
    let slice_begin_2 = &s[..5];
    let slice_end_1 = &s[6..len];
    let slice_end_2 = &s[6..];
    let slice_full_1 = &s[0..len];
    let slice_full_2 = &s[..];
    println!("s: {}, slice_begin_1: {}, slice_begin_2 : {}, slice_end_1 :{}, slice_end_2: {}, slice_full_1: {}, slice_full_2: {}",
             s, slice_begin_1, slice_begin_2, slice_end_1, slice_end_2, slice_full_1, slice_full_2);

    let first_word = get_first_word(&s);
    println!("the first word: {}", first_word);

    let chinois = "中国人";
    let one = &chinois[0..3];
    println!("注意汉字切片 chinois: {}, one: {}", chinois, one);

    println!("切，什么都可以切！");
    let a = [1, 2, 3, 4, 5];
    let slice_a = &a[1..3];
    assert_eq!(slice_a, &[2, 3]);
}

fn str2string(s: &str) -> String {
    String::from(s)
    // s.to_string()
}

fn string2str(s: String) {
    let str1 = &s;
    let str2 = &s[..];
    let str3 = s.as_str();
    println!("str1: {}, str2: {}, str3: {}", str1, str2, str3);
}

fn get_first_word(s: &String) -> &str { &s[..1] }

fn str_gone() {
    let s = String::from("dar");
    takes_str_ownership(s);
    let x = 5;
    makes_int_copy(x);
    let t = String::from("love");
    let new_t = keep_str_ownership(t);
    println!(
        "s: {}, x: {}, t: {}, new_t: {}",
        "s is gone", x, "t is gone", new_t
    );

    let alive_s = String::from("love");
    let len = cal_length(&alive_s).1;
    println!("The length of '{}' is {}", alive_s, len);

    let mut alive_mut_s = String::from("love");
    str_add(&mut alive_mut_s);
}

fn cal_length(s: &String) -> (&String, usize) {
    (s, s.len())
}

fn str_add(str: &mut String) {
    println!("str_add source: {}", str);
    str.push_str(" Bonne nuit");
    println!("push_str ' Bonne nuit' now: {}", str);
    str.push('.');
    println!("push '.' now: {}", str);

    str.insert(str.len(), ' ');
    println!("insert ' ' now: {}", str);
    str.insert_str(str.len(), "WOW");
    println!("insert_str 'WOW' now: {}", str);
}

fn str_replace() {
    let s = String::from("I like u. u r my sunshine");
    let str = "I like u. u r my sunshine";
    let new_s = s.replace("u", "you");
    let new_str = str.replace("u", "you");
    dbg!(new_s,new_str);
    let new_s_once = s.replacen("u", "you", 1);
    let new_str_once = str.replacen("u", "you", 1);
    dbg!(new_s_once,new_str_once);
    let mut only_string = String::from("I like u. u r my sunshine");
    only_string.replace_range(2..only_string.len(), "...");
    dbg!(only_string);
}

fn str_delete() {
    // all of these only for string
    let mut str_pop = String::from("I like u. u r my sunshine.人.s");
    let p1 = str_pop.pop();
    let p2 = str_pop.pop();
    let p3 = str_pop.pop();
    dbg!(p1,p2,p3,str_pop);
    let mut str_del = String::from("I like u. u r my sunshine.人.s");
    let d1 = str_del.remove(str_del.len() - 1);
    let d2 = str_del.truncate(6);
    dbg!(d1,d2,str_del);
}

fn takes_str_ownership(str: String) {
    println!("takes_str_ownership {}", str);
}

fn keep_str_ownership(str: String) -> String {
    println!("keep_str_ownership {}", str);
    str
}

fn makes_int_copy(int: i32) {
    println!("makes_int_copy {}", int);
}

fn add<T: std::ops::Add<Output=T>>(i: T, j: T) -> T {
    i + j
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式 表达式不能包含分号 加分号就变成语句了
}

fn pattern_matching() {}

fn loops() {
    for i in 0..10 {
        print!("{}", i);
    }
    println!();
    for i in 0..=10 {
        print!("{}", i);
    }
    println!();
    for i in 'a'..='z' {
        print!("{}", i);
    }
    println!();
}

use num::complex::Complex;
use crate::Weapons::Sword;

fn num_use() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let res = a + b;
    println!("{} + {}i", res.re, res.im);
}

fn tuple_use() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("tup0: {}, tup1: {}, tup2: {}", tup.0, tup.1, tup.2);
}

enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

#[derive(Debug)]
enum Weapons {
    Sword,
    Bow,
    Polearm,
    Claymore,
    Catalyst,
}

#[derive(Debug)]
struct Stats {
    hp: i32,
    atk: i32,
    def: i32,
}


enum EnumCharacter {
    ID(i32),
    Name(i32),
    Rarity(i32),
    Weapon(Weapons),
    Stats(Stats),
}

#[derive(Debug)]
// Rust 不支持将某个结构体某个字段标记为可变。
struct Character {
    id: i32,
    name: i32,
    rarity: i32,
    weapon: Weapons,
    stats: Stats,
}

fn build_character(
    id: i32,
    name: i32,
    rarity: i32,
    weapon: Weapons,
    stats: Stats) -> Character {
    Character {
        id,
        name,
        rarity,
        weapon,
        stats,
    }
}

fn character_test() {
    let stats = Stats {
        hp: 999,
        atk: 999,
        def: 999,
    };
    let character1 = build_character(1, 1, 1, Sword, stats);
    let character2 = Character {
        id: 3,
        name: character1.name,
        rarity: character1.rarity,
        weapon: character1.weapon,
        stats: character1.stats,
    };// character1 无法再被使用
    let character3 = Character {
        id: 2,
        ..character2
    };// character2 无法再被使用

    println!("character3 {:#?}", character3);
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn array() {
    let a = [3; 5];
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    for i in a {
        print!("{} ", i);
    }
    println!();
    for i in &a {
        print!("{} ", i);
    }
    println!();
    let a = a[1];
    println!("a: {}", a);

    let mut a: [i32; 5] = [1, 2, 3, 4, 4];
    for mut i in a {
        if i % 2 == 0 { i = i + 1 };
        print!("{} ", i);
    }
    dbg!(a);

    for (i, v) in a.iter().enumerate() {
        print!("{}: {}, ", i, v);
    }
    println!();

    for _ in 0..5 {
        println!("this is China!")
    }
}

enum IpAddress {
    Ipv4Address(String),
    Ipv6Address(String),
}

// TODO 地址校验
fn ip_switch(address: &String, standard: bool) -> String {
    let mut is_ipv4 = false;
    if address.contains('.') { is_ipv4 = true }

    let mut res: String = str2string("");

    if is_ipv4 == true {
        if standard == false {
            res = str2string("::") + address;
            dbg!(&res);
        } else {
            let mut temp = address;
            let mut index_arr = [0; 3];
            let mut index = 0;
            for (i, char) in temp.chars().enumerate() {
                if char == '.' {
                    index_arr[index] = i;
                    index = index + 1;
                }
            }
            dbg!(index_arr);
            let mut value_ipv4 = [
                temp[0..index_arr[0]].parse::<u8>().unwrap(),
                temp[index_arr[0] + 1..index_arr[1]].parse::<u8>().unwrap(),
                temp[index_arr[1] + 1..index_arr[2]].parse::<u8>().unwrap(),
                temp[index_arr[2] + 1..temp.len()].parse::<u8>().unwrap(),
            ];
            dbg!(value_ipv4);

            for mut i in value_ipv4 {
                res += &*format!("{:02X}", i);
            }
            res.insert(4, ':');
            res.insert_str(0, "::");
            dbg!(&res);
        }
    } else {
        dbg!("ipv6 waiting");
    }


    return res;
}


fn main() {
    println!(
        "Genshin MAX_DAMAGE: {} MAX_LEVEL: {}",
        MAX_DAMAGE, MAX_LEVEL
    );

    ip_switch(&str2string("127.0.0.1"), true);

    println!("add 2 + 3: {}", add(2, 3));
    println!("add_with_extra 2 + 3: {}", add_with_extra(2, 3));

    greet_world();
    small_scale_chopper();
    variables();
    pattern_matching();
    loops();
    num_use();
    str_gone();
    str_slice();
    str_replace();
    str_delete();
    tuple_use();
    array();

    character_test();
}
