mod characters;

#[allow(dead_code)]
const MAX_DAMAGE: u64 = 21_474_836_473;
#[allow(dead_code)]
const MAX_LEVEL: u8 = 90;

enum _PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_test() {
        println!(
            "fn character_test() Genshin MAX_DAMAGE: {} MAX_LEVEL: {}",
            MAX_DAMAGE, MAX_LEVEL
        );

        let character1 = characters::Character::new("jean".to_string());
        println!("character1 {:#?}", character1);
    }
}


mod genshin_info {
    mod character {
        #[allow(dead_code)]
        fn get() {
            super::super::characters::Character::random();
        }

        #[allow(dead_code)]
        fn name_info() {}
    }

    mod events {
        #[allow(dead_code)]
        fn banner() {}

        #[allow(dead_code)]
        fn trail_role() {}
    }
}