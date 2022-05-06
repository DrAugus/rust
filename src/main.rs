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
    let y = 3;
    let _z = 44;
}

fn main() {
    greet_world();
    small_scale_chopper();
    variables();
}
