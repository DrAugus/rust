use std::fmt;
use std::fmt::{Display};
use std::hash::Hash;


mod exception;
mod format_print;
mod genshin;
mod hello;
mod ip;
mod social_info;
mod str;
mod utils;
mod variables;
mod loops;
mod rookie;
mod implement;
mod file;
mod feature;


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

fn main() {
    social_info::random_use();
    feature::show_screen();




    ip::ip_switch(&"127.0.0.1".to_string(), true);


    hello::greet_world();
    small_scale_chopper();

    str::str();
  

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

    variables::variables();

    exception::exception();

}
