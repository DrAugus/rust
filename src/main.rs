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
    let regions = [southern_germany, chinese, english, italian, french, russian, korean, german];
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
        if i == 0 || record.trim().len() == 0 { continue; }
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) { eprintln!("debug: {:?} -> {:?}", record, fields); }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() { println!("{}, {}cm", name, length) }
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

    // 同一作用域，特定数据只能有一个可变引用：
    let mut m_s = String::from("hi");
    let m_s_1 = &mut m_s;
    // let m_s_2 = &mut m_s;
    println!("m_s_1: {} m_s_2: {}", m_s_1, "m_s_2 has borrowed");


    let i1 = 5;
    let i2 = &i1;
    assert_eq!(i1, *i2);
}

fn str_gone() {
    let s = String::from("dar");
    takes_str_ownership(s);
    let x = 5;
    makes_int_copy(x);
    let t = String::from("love");
    let new_t = keep_str_ownership(t);
    println!("s: {} x: {} t: {} new_t: {}", "s is gone", x, "t is gone", new_t);

    let alive_s = String::from("love");
    let len = cal_length(&alive_s);
    println!("The length of '{}' is {}", alive_s, len);

    let mut alive_mut_s = String::from("love");
    change_str(&mut alive_mut_s);
}

fn cal_length(s: &String) -> usize { s.len() }

fn change_str(str: &mut String) { str.push_str(" Bonne nuit") }

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

fn add(i: i32, j: i32) -> i32 { i + j }

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式 表达式不能包含分号 加分号就变成语句了
}

fn pattern_matching() {}

fn loops() {
    for i in 0..10 { print!("{}", i); }
    println!();
    for i in 0..=10 { print!("{}", i); }
    println!();
    for i in 'a'..='z' { print!("{}", i); }
    println!();
}

use num::complex::Complex;

fn num_use() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let res = a + b;
    println!("{} + {}i", res.re, res.im);
}

fn main() {
    println!("Genshin MAX_DAMAGE: {} MAX_LEVEL: {}", MAX_DAMAGE, MAX_LEVEL);

    println!("add 2 + 3: {}", add(2, 3));
    println!("add_with_extra 2 + 3: {}", add_with_extra(2, 3));

    greet_world();
    small_scale_chopper();
    variables();
    pattern_matching();
    loops();
    num_use();
    str_gone();
}
