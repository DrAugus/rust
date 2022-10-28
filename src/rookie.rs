fn _add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式 表达式不能包含分号 加分号就变成语句了
}

// In addition to number, you can also match char
fn _match_number(x: i32) {
    match x {
        0 => println!("zero"),
        1..=9 if x < 5 => println!("one through 4"),
        1..=9 => println!("one through 9"),
        11 | 22 | 33 => println!("{}", x),
        _ => println!("else: {}", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn rookie() {
        print!("222");
    }
}
