#[derive(Debug)]
#[allow(dead_code)]
// Rust 不支持将某个结构体某个字段标记为可变。
pub(crate) struct Character {
    id: String,
    name: String,
    prefix: String,
    star: u8,
    event_exclusive: bool,
    intro: String,
    city: City,
    ele: Element,
    weapon: Weapon,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum Weapon {
    Sword,
    Bow,
    Polearm,
    Claymore,
    Catalyst,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum City {
    Mondstadt,
    Liyue,
    Inazuma,
    Sumeru,
    Fontaine,
    Natlan,
    Snezhnaya,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum Element {
    Anemo,
    Pyro,
    Cryo,
    Electro,
    Hydro,
    Geo,
    Dendro,
}

#[allow(dead_code)]
pub(crate) fn character() -> Vec<Character> {
    let vec: Vec<Character> = vec![
        Character {
            id: "nilou".to_string(),
            name: "妮露".to_string(),
            prefix: "莲光落舞筵".to_string(),
            star: 5,
            event_exclusive: true,
            intro: "「祖拜尔剧场」的明星演员，舞姿娉婷，如睡莲初绽，一尘不染。但她绝非高傲清冷之人，即便只是匆匆的旅者，也会对她纯洁质朴的笑容过目不忘。".to_string(),
            city: City::Sumeru,
            ele: Element::Hydro,
            weapon: Weapon::Catalyst,
        },
    ];

    vec
}

impl Character {
    #[allow(dead_code)]
    pub(crate) fn new(id: String) -> Character {
        Character {
            id,
            name: "".to_string(),
            prefix: "".to_string(),
            star: 0,
            event_exclusive: false,
            intro: "".to_string(),
            city: City::Mondstadt,
            ele: Element::Anemo,
            weapon: Weapon::Sword,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn random() -> Character {
        Character {
            id: "".to_string(),
            name: "".to_string(),
            prefix: "".to_string(),
            star: 0,
            event_exclusive: false,
            intro: "".to_string(),
            city: City::Mondstadt,
            ele: Element::Anemo,
            weapon: Weapon::Sword,
        }
    }
}