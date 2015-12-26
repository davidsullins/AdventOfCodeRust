// advent21.rs
// RPG

#[macro_use] extern crate scan_fmt;

use std::io;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    let mut boss = Stats {hit_points: 0, damage: 0, armor: 0};

    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let hit_points = scan_fmt!(&input, "Hit Points: {}", i32);
    boss.hit_points = hit_points.unwrap();

    input.clear();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let damage = scan_fmt!(&input, "Damage: {}", i32);
    boss.damage = damage.unwrap();

    input.clear();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let armor = scan_fmt!(&input, "Armor: {}", i32);
    boss.armor = armor.unwrap();


    let cost = find_least_gold(&boss);
    println!("Least gold needed: {}", cost);

    let cost2 = find_most_gold(&boss);
    println!("Most gold needed: {}", cost2);

}

struct Stats {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

fn does_player_win(player: &Stats, boss: &Stats) -> bool {
    let mut player_hp = player.hit_points;
    let mut boss_hp = boss.hit_points;

    let player_damage = max(1, player.damage - boss.armor);
    let boss_damage = max(1, boss.damage - player.armor);

    loop {
        if player_hp <= 0 {
            return false;
        }
        boss_hp -= player_damage;

        if boss_hp <= 0 {
            return true;
        }

        player_hp -= boss_damage;
    }
}

#[derive(Clone)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

fn add_items(item1: &Item, item2: &Item) -> Item {
    Item {
        cost: item1.cost + item2.cost,
        damage: item1.damage + item2.damage,
        armor: item1.armor + item2.armor,
    }
}

fn find_item_combinations() -> Vec<Item> {
    let weapons = vec![
        Item {cost: 8, damage: 4, armor: 0},
        Item {cost: 10, damage: 5, armor: 0},
        Item {cost: 25, damage: 6, armor: 0},
        Item {cost: 40, damage: 7, armor: 0},
        Item {cost: 74, damage: 8, armor: 0},
        ];
    let armors = vec![
        Item {cost: 13, damage: 0, armor: 1},
        Item {cost: 31, damage: 0, armor: 2},
        Item {cost: 53, damage: 0, armor: 3},
        Item {cost: 75, damage: 0, armor: 4},
        Item {cost: 102, damage: 0, armor: 5},
        ];
    let rings = vec![
        Item {cost: 25, damage: 1, armor: 0},
        Item {cost: 50, damage: 2, armor: 0},
        Item {cost: 100, damage: 3, armor: 0},
        Item {cost: 20, damage: 0, armor: 1},
        Item {cost: 40, damage: 0, armor: 2},
        Item {cost: 80, damage: 0, armor: 3},
        ];

    let mut combos = Vec::new();

    // exactly 1 weapon
    // 0 or 1 armor
    // 0 - 2 rings
    for w in &weapons {
        // 0 armor, 0 rings
        combos.push(w.clone());

        // 0 armor, 0 - 2 rings
        enumerate_rings(&mut combos, &rings, &w);

        // 1 armor, 0 - 2 rings
        for a in &armors {
            let base = add_items(&w, &a);
            enumerate_rings(&mut combos, &rings, &base);
        }
    }
    
    combos
}

fn stats_from_items(item_total: &Item) -> Stats {
    Stats {hit_points: 100, damage: item_total.damage, armor: item_total.armor}
}

// Find least gold needed to defeat a given boss
fn find_least_gold(boss: &Stats) -> i32 {
    let combos = find_item_combinations();

    combos.iter()
        .filter(|combo| does_player_win(&stats_from_items(&combo), boss))
        .map(|combo| combo.cost)
        .min().unwrap()
}

fn enumerate_rings(combos: &mut Vec<Item>, rings: &[Item], base: &Item) {
    // 0 - 2 rings
    for bit_mask in 0..(1 << rings.len()) {
        if count_bits(bit_mask) <= 2 {
            let ring_sum = rings.iter().enumerate()
                .filter(|&(i, _)| bit_mask & (1 << i) != 0)
                .fold(base.clone(), |acc, (_, r)| add_items(&acc, r));
            combos.push(ring_sum);
        }
    }
}


// copied from Day 17
fn count_bits(x: u32) -> u32 {
    let mut total = 0;
    let mut val = x;

    while val > 0 {
        if (val & 1) != 0 {
            total += 1;
        }
        val >>= 1;
    }

    total
}

#[test]
fn test_does_player_win() {
    let player = Stats{hit_points: 8, damage: 5, armor: 5};
    let boss = Stats{hit_points: 12, damage: 7, armor: 2};
    let boss2 = Stats{hit_points: 12, damage: 7, armor: 3};


    assert!(does_player_win(&player, &boss));
    assert!(!does_player_win(&player, &boss2));
}

// part 2

// Find most gold you can spend and still lose to a given boss
fn find_most_gold(boss: &Stats) -> i32 {
    let combos = find_item_combinations();

    combos.iter()
        .filter(|combo| !does_player_win(&stats_from_items(&combo), boss))
        .map(|combo| combo.cost)
        .max().unwrap()
}


