use std::collections::HashMap;
use num::Complex;

fn basics() {
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

    let a: [i32; 5] = [1, 2, 3, 4, 4];
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

    let mut v3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v4: Vec<i32> = v3.iter().map(|num|{
        num * 2
    }).collect();
    dbg!(v4);
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
    let mut lang_map_normal = HashMap::new();
    for t in &lang_list {
        lang_map_normal.insert(&t.0, &t.1);
    }
    dbg!(lang_map_normal);

    let lang_map: HashMap<_, _> = lang_list.into_iter().collect();
    dbg!(&lang_map);

    let lan_name = "English".to_string();
    let num: Option<&i32> = lang_map.get(&lan_name);
    dbg!(num);

    println!("Traversal lang_map");
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

    println!("Traversal scores");
    for (k, v) in scores {
        println!("{}:{} ", k, v);
    }

    // 在已有值的基础上更新
    // 统计文本中词语出现的次数
    let text_example = "hello my name is makabaka hello her name is wuxidixi";
    let mut map = HashMap::new();
    for word in text_example.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    dbg!(map);
}

fn type_conversion() {
    let i8max = i8::MAX;
    println!("i8 max {}", i8max);
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{} {} {}", a, b, c);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    println!("{} {} {:?}", first_address, second_address, values);
    assert_eq!(values[1], 3);

    // 其他类型转换暂不考虑
    // https://course.rs/basic/converse.html#%E9%80%9A%E7%94%A8%E7%B1%BB%E5%9E%8B%E8%BD%AC%E6%8D%A2
}


pub(crate) fn variables() {
    println!("=== variables.rs beg ===");
    basics();
    num_use();
    tuple_use();
    array();
    read_change_vec();
    use_hashmap();
    type_conversion();
    println!("=== variables.rs end ===");
}