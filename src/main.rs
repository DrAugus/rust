use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, format};
use std::hash::Hash;
use std::process::Output;
use num::complex::Complex;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use std::ops::Add;


mod genshin;
mod hello;
mod ip;
mod social_info;
mod str;
mod utils;
mod variables;


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
            x: utils::add(self.x, p.x),
            y: utils::add(self.y, p.y),
            z: utils::add(self.z, p.z),
        }
    }
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
    social_info::random_use();

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

    ip::ip_switch(&"127.0.0.1".to_string(), true);

    println!("add 2 + 3: {}", utils::add(2, 3));
    println!("add_with_extra 2 + 3: {}", add_with_extra(2, 3));

    hello::greet_world();
    small_scale_chopper();
    variables::variables();
    variables::num_use();
    variables::tuple_use();
    variables::array();
    str::str_slice();
    str::str_gone();
    str::str_replace();
    str::str_delete();
    pattern_matching();
    loops();

    genshin::character_test();

    social_info::generate_id();


    let arr_find_largest = [1, 2, 3, 4, 9, 6, 1];
    dbg!(utils::largest(&arr_find_largest));

    let person_fly = Human;
    Pilot::fly(&person_fly);
    Wizard::fly(&person_fly);
    person_fly.fly();

    // 完全限定语法 定义  <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A babe cat is called a {}", <Cat as Animal>::babe_name());

    read_change_vec();
    use_hashmap();
}
