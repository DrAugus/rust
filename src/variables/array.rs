#[allow(dead_code)]
fn array() {
    let a = [3; 5];
    let _vec_a = Vec::from(a);
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
#[allow(dead_code)]
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

    let v3 = Vec::new();
    let v3 = fill_vec(&v3);
    let v3 = fill_vec_2(v3);
    let v4: Vec<i32> = v3.iter().map(|num|{
        num * 2
    }).collect();
    dbg!(v4);
}
#[allow(dead_code)]
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
#[allow(dead_code)]
fn fill_vec_2(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        array();
        read_change_vec();
    }
}