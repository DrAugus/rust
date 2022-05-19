use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, format};
use std::hash::Hash;
use std::process::Output;
use num::complex::Complex;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::Add;

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

#[derive(Debug)]
// Rust 不支持将某个结构体某个字段标记为可变。
struct Character {
    id: i32,
    name: i32,
    rarity: i32,
    weapon: Weapons,
    stats: Stats,
}

impl Character {
    fn new(id: i32, name: i32, rarity: i32, weapon: Weapons, stats: Stats) -> Character {
        Character {
            id,
            name,
            rarity,
            weapon,
            stats,
        }
    }
}

fn character_test() {
    let stats = Stats {
        hp: 999,
        atk: 999,
        def: 999,
    };
    let character1 = Character::new(1, 1, 1, Weapons::Sword, stats);
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

struct Color(u8, u8, u8);

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output=T>> {
    x: T,
    y: T,
    z: T,
}

impl<T: Add<T, Output=T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: add(self.x, p.x),
            y: add(self.y, p.y),
            z: add(self.z, p.z),
        }
    }
}

enum IpAddress {
    Ipv4Address(String),
    Ipv6Address(String),
}

fn check_ipv4_address(address: &String) -> ([u8; 4], bool) {
    let mut is_ipv4 = false;
    if address.contains('.') { is_ipv4 = true }
    if !is_ipv4 { return ([0, 0, 0, 0], false); }

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
        temp[0..index_arr[0]].parse().unwrap(),
        temp[index_arr[0] + 1..index_arr[1]].parse().unwrap(),
        temp[index_arr[1] + 1..index_arr[2]].parse().unwrap(),
        temp[index_arr[2] + 1..temp.len()].parse().unwrap(),
    ];
    dbg!(value_ipv4);
    for i in value_ipv4 {
        if i > 255 || i < 1 {
            ([0, 0, 0, 0], false);
        }
    }
    (value_ipv4, true)
}

fn ip_switch(address: &String, standard: bool) -> String {
    let check_ipv4 = check_ipv4_address(address);
    let mut is_ipv4 = check_ipv4.1;
    let ipv4_value = check_ipv4.0;

    let mut res: String = str2string("");

    if is_ipv4 == true {
        if standard == false {
            res = str2string("::") + address;
            dbg!(&res);
        } else {
            for mut i in ipv4_value {
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

enum Direction {
    East,
    West,
    North,
    South,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}

fn show_direction(d: Direction) -> &'static str {
    match d {
        Direction::East => "East",
        Direction::SouthEast | Direction::SouthWest => "South",
        Direction::North => {
            println!("厚礼蟹");
            "North"
        }
        _ => "other",
    }
}

enum Action {
    Say(String),
    MoveTo(Point<f32>, Point<f32>),
    ChangeColor(Color),
    Loading(bool),
}

fn op_action(op: Action) {
    match op {
        Action::Say(s) => println!("{}", s),
        Action::MoveTo(x, y) => {
            println!("point from ({:?}) move to ({:?})", x, y);
        }
        Action::ChangeColor(c) => {
            println!("change color into (r:{}, g:{}, b:{}))",
                     c.0, c.1, c.2);
        }
        _ => {
            println!("loading");
        }
    }
}

// In addition to number, you can also match char
fn match_number(x: i32) {
    match x {
        0 => println!("zero"),
        1..=9 if x < 5 => println!("one through 4"),
        1..=9 => println!("one through 9"),
        11 | 22 | 33 => println!("{}", x),
        _ => println!("else: {}", x)
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// Self 指代被实现方法的结构体类型，self 指代此类型的实例
impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    fn area(&self) -> f64 { std::f64::consts::PI * (self.radius * self.radius) }
}

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

fn random_use() {
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

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: String) -> File {
        File {
            name,
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

pub struct Screen {
    // 动态数组，类型是 Draw 特征对象：Box<dyn Draw>，
    // 任何实现了 Draw 特征的类型，都可以存放其中。
    // 当使用特征对象时，Rust 必须使用动态分发
    // 关于【特征对象的动态分发】还得仔细研究
    // https://course.rs/basic/trait/trait-object.html#%E7%89%B9%E5%BE%81%E5%AF%B9%E8%B1%A1%E7%9A%84%E5%8A%A8%E6%80%81%E5%88%86%E5%8F%91
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 泛型实现
pub struct GenericScreen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> GenericScreen<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl Draw for u8 {
    fn draw(&self) {
        println!("u8 {}", *self);
    }
}

impl Draw for f64 {
    fn draw(&self) {
        println!("f64 {}", *self);
    }
}

fn draw1(x: Box<dyn Draw>) { x.draw(); }

fn draw2(x: &dyn Draw) { x.draw(); }


pub trait CacheableItem: Clone + Default + fmt::Debug {
    type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
    fn is_null(&self) -> bool;
}

trait Container<A, B> {
    fn contains(&self, a: A, b: B) -> bool;
}

fn difference<A, B, C>(container: &C) -> i32 where C: Container<A, B> { 1 }

//Generic refactor
trait ContainerGeneric {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

fn difference_generic<C: ContainerGeneric>(container: &C) {}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        dbg!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        dbg!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        dbg!("*waving arms furiously*");
    }
}

trait Animal {
    fn babe_name() -> String;
}

struct Cat;

impl Cat {
    fn babe_name() -> String {
        "babe".to_string()
    }
}

impl Animal for Cat {
    fn babe_name() -> String {
        "meow".to_string()
    }
}

fn read_change_vec() {
    // 若预先知道大小可以使用 with_capacity 避免频繁的内存分配和拷贝，提升性能
    let _v: Vec<i32> = Vec::with_capacity(3);

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let forth: &i32 = &v[3];
    println!("forth value is {}", forth);

    // 对于get 需要用 match 匹配
    match v.get(5) {
        Some(sixth) => println!("sixth value is {}", sixth),
        None => println!("no sixth value"),
    }
    match v.get(11) {
        Some(eleventh) => println!("eleventh value is {}", eleventh),
        None => println!("no eleventh value"),
    }

    let mut v2 = v;
    for i in &mut v2 {
        *i += 10
    }
    dbg!(v2);
}

fn use_hashmap() {
    // 若预先知道大小可以使用 with_capacity 避免频繁的内存分配和拷贝，提升性能
    let _hm: HashMap<String, i32> = HashMap::with_capacity(3);

    let mut gems = HashMap::new();
    gems.insert("red gems", 1);
    gems.insert("green gems", 2);
    gems.insert("blue gems", 3);

    let lang_list = vec![
        ("English".to_string(), 1),
        ("French".to_string(), 2),
        ("German".to_string(), 3),
    ];
    let lang_map: HashMap<_, _> = lang_list.into_iter().collect();
    dbg!(&lang_map);

    let lan_name = "English".to_string();
    let num: Option<&i32> = lang_map.get(&lan_name);
    dbg!(num);

    for (k, v) in &lang_map {
        println!("{}:{} ", k, v);
    }

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}

fn main() {
    println!(
        "Genshin MAX_DAMAGE: {} MAX_LEVEL: {}",
        MAX_DAMAGE, MAX_LEVEL
    );
    random_use();

    let draw_x = 1.1f64;
    let draw_y = 8u8;
    draw1(Box::new(draw_x));
    draw1(Box::new(draw_y));
    draw2(&draw_x);
    draw2(&draw_y);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let file1 = File::new("file1.txt".to_string());
    println!("{:?}", file1);
    println!("{}", file1);

    let say_it = Action::Say("unwrap".to_string());
    // 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
    if let Action::Say(s) = say_it {
        println!("hit! {}", s);
    }
    // matches 暂不展开

    let actions = [
        Action::Say("hi".to_string()),
        Action::MoveTo(Point { x: 2.0, y: 3.0, z: 1.0 },
                       Point { x: 2.0, y: 3.0, z: 1.0 }),
        Action::ChangeColor(Color(2, 3, 4)),
    ];
    for action in actions {
        op_action(action);
    }

    ip_switch(&str2string("127.0.0.1"), true);

    println!("add 2 + 3: {}", add(2, 3));
    println!("add_with_extra 2 + 3: {}", add_with_extra(2, 3));

    greet_world();
    small_scale_chopper();
    variables();
    str_slice();
    str_gone();
    str_replace();
    str_delete();
    pattern_matching();
    loops();
    num_use();
    tuple_use();
    array();

    character_test();

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

    let arr_find_largest = [1, 2, 3, 4, 9, 6, 1];
    dbg!(largest(&arr_find_largest));

    let person_fly = Human;
    Pilot::fly(&person_fly);
    Wizard::fly(&person_fly);
    person_fly.fly();

    // 完全限定语法 定义  <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A babe cat is called a {}", <Cat as Animal>::babe_name());

    read_change_vec();
    use_hashmap();
}
