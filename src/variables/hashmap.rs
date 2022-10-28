use std::collections::HashMap;

#[derive(Debug)]
// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // update score
        scores.insert(team_1_name.clone(), Team {
            name: team_1_name.to_string(),
            goals_scored: team_1_score + if scores.contains_key(&team_1_name) { scores[&team_1_name].goals_scored } else { 0 },
            goals_conceded: team_2_score + if scores.contains_key(&team_1_name) { scores[&team_1_name].goals_conceded } else { 0 },
        });
        scores.insert(team_2_name.clone(), Team {
            name: team_2_name.to_string(),
            goals_scored: team_2_score + if scores.contains_key(&team_2_name) { scores[&team_2_name].goals_scored } else { 0 },
            goals_conceded: team_1_score + if scores.contains_key(&team_2_name) { scores[&team_2_name].goals_conceded } else { 0 },
        });
    }
    scores
}

fn get_results() -> String {
    let results = "".to_string()
        + "England,France,4,2\n"
        + "France,Italy,3,1\n"
        + "Poland,Spain,2,0\n"
        + "Germany,England,2,1\n";
    results
}

fn build_scores() {
    let scores = build_scores_table(get_results());
    for (k, v) in scores {
        println!("{} {:#?}", k, v);
        assert_eq!(k, v.name);
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        if !basket.contains_key(&fruit) {
            basket.insert(fruit, 1);
        };
    }
}

fn get_fruit_basket() -> HashMap<Fruit, u32> {
    let mut basket = HashMap::<Fruit, u32>::new();
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lychee, 5);

    basket
}

pub(crate) fn use_hashmap() {
    // 若预先知道大小可以使用 with_capacity 避免频繁的内存分配和拷贝，提升性能
    let _hm: HashMap<String, i32> = HashMap::with_capacity(3);

    let mut gems = HashMap::new();
    gems.insert("red gems", 1);
    gems.insert("green gems", 2);
    gems.insert("blue gems", 3);

    dbg!(gems.len() >= 3);
    dbg!(gems.values().sum::<u32>() > 5);

    let lang_list = vec![
        ("English".to_string(), 1),
        ("French".to_string(), 2),
        ("German".to_string(), 3),
    ];
    let mut lang_map_normal = HashMap::new();
    for t in &lang_list {
        lang_map_normal.insert(&t.0, &t.1);
    }
    dbg!(lang_map_normal);

    let lang_map: HashMap<_, _> = lang_list.into_iter().collect();
    dbg!(&lang_map);

    let lan_name = "English".to_string();
    let num: Option<&i32> = lang_map.get(&lan_name);
    dbg!(num);

    println!("Traversal lang_map");
    for (k, v) in &lang_map {
        println!("{}:{} ", k, v);
    }

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5
    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入

    println!("Traversal scores");
    for (k, v) in scores {
        println!("{}:{} ", k, v);
    }

    // 在已有值的基础上更新
    // 统计文本中词语出现的次数
    let text_example = "hello my name is makabaka hello her name is wuxidixi";
    let mut map = HashMap::new();
    for word in text_example.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    dbg!(map);
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    dbg!(basket);

    build_scores();
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
