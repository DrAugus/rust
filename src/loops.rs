#[allow(dead_code)]
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
