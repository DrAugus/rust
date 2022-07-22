use num::Complex;

pub(crate) fn variables() {
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


pub(crate) fn num_use() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let res = a + b;
    println!("{} + {}i", res.re, res.im);
}

pub(crate) fn tuple_use() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("tup0: {}, tup1: {}, tup2: {}", tup.0, tup.1, tup.2);
}

pub(crate) fn array() {
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