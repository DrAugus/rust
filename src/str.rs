pub(crate) fn str_slice() {
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

pub(crate) fn str_gone() {
    let s = String::from("dar");
    takes_str_ownership(s);
    let x = 5;
    let makes_int_copy = |int: i32| { println!("makes_int_copy {}", int); };
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

pub(crate) fn str_replace() {
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

pub(crate) fn str_delete() {
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