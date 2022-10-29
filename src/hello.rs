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


