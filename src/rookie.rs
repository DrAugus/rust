fn _small_scale_chopper() {
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

fn _greet_world() {
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

fn _add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式 表达式不能包含分号 加分号就变成语句了
}

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rookie() {
        match_number(2);
    }
}
