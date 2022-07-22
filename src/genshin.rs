const MAX_DAMAGE: u64 = 21_474_836_473;
const MAX_LEVEL: u8 = 90;

enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

#[derive(Debug)]
enum Weapons {
    Sword,
    Bow,
    Polearm,
    Claymore,
    Catalyst,
}

#[derive(Debug)]
struct Stats {
    hp: i32,
    atk: i32,
    def: i32,
}

#[derive(Debug)]
// Rust 不支持将某个结构体某个字段标记为可变。
struct Character {
    id: i32,
    name: i32,
    rarity: i32,
    weapon: Weapons,
    stats: Stats,
}

impl Character {
    fn new(id: i32, name: i32, rarity: i32, weapon: Weapons, stats: Stats) -> Character {
        Character {
            id,
            name,
            rarity,
            weapon,
            stats,
        }
    }
}

pub(crate) fn character_test() {
    println!(
        "fn character_test() Genshin MAX_DAMAGE: {} MAX_LEVEL: {}",
        MAX_DAMAGE, MAX_LEVEL
    );

    let stats = Stats {
        hp: 999,
        atk: 999,
        def: 999,
    };
    let character1 = Character::new(1, 1, 1, Weapons::Sword, stats);
    let character2 = Character {
        id: 3,
        name: character1.name,
        rarity: character1.rarity,
        weapon: character1.weapon,
        stats: character1.stats,
    };// character1 无法再被使用
    let character3 = Character {
        id: 2,
        ..character2
    };// character2 无法再被使用

    println!("character3 {:#?}", character3);
}
