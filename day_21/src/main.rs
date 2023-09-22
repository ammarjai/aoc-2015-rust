#![warn(clippy::pedantic)]

use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Character {
    reset_hit_points: u8,
    hit_points: u8,
    damage: u8,
    armor: u8,
}

impl Character {
    fn new() -> Self {
        Self {
            reset_hit_points: 100,
            hit_points: 100,
            damage: 0,
            armor: 0,
        }
    }

    fn from_txt(inputs: &str) -> Self {
        let mut new = Character::new();
        for (i, line) in inputs.lines().enumerate() {
            let value = line.split(' ').last().unwrap().parse().unwrap();
            match i {
                0 => {
                    new.reset_hit_points = value;
                    new.hit_points = value;
                }
                1 => new.damage = value,
                2 => new.armor = value,
                _ => (),
            }
        }
        new
    }

    fn reset(&mut self) {
        self.hit_points = self.reset_hit_points;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    name: String,
    cost: usize,
    type_: ItemType,
    damage: u8,
    armor: u8,
}

impl Item {
    fn from_txt(inputs: &str) -> Vec<Item> {
        let re = Regex::new(
            r"(?<name>\w+[ +\d]{0,3})[ ]{0,}(?<cost>\d+)[ ]{0,}(?<damage>\d+)[ ]{0,}(?<armor>\d+)",
        )
        .unwrap();
        let mut type_ = ItemType::Weapon;
        let mut output = Vec::new();
        for line in inputs.lines() {
            if line.contains("Weapons:") {
                type_ = ItemType::Weapon;
            } else if line.contains("Armor:") {
                type_ = ItemType::Armor;
            } else if line.contains("Rings:") {
                type_ = ItemType::Ring;
            } else if line.is_empty() {
                continue;
            } else {
                let capture = re.captures(line).unwrap();
                output.push(Item {
                    name: capture.name("name").unwrap().as_str().trim().to_string(),
                    cost: capture.name("cost").unwrap().as_str().parse().unwrap(),
                    type_,
                    damage: capture.name("damage").unwrap().as_str().parse().unwrap(),
                    armor: capture.name("armor").unwrap().as_str().parse().unwrap(),
                });
            }
        }
        output
    }
}

fn attack(player1: &Character, player2: &mut Character) {
    if player2.hit_points == 0 {
        return;
    }

    if player1.damage > player2.armor {
        let damage = player1.damage - player2.armor;
        match player2.hit_points {
            v if v >= damage => player2.hit_points -= damage,
            _ => player2.hit_points = 0,
        }
    } else {
        player2.hit_points -= 1;
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let items_file_path = env::args().nth(2).expect("Missing file path argument!");

    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");
    let mut boss_character = Character::from_txt(&inputs);
    let mut player_character = Character::new();

    let item_inputs = fs::read_to_string(items_file_path).expect("items.txt not found!");
    let items = Item::from_txt(&item_inputs);
    let weapons: Vec<&Item> = items
        .iter()
        .filter(|&f| f.type_ == ItemType::Weapon)
        .collect();
    let armors: Vec<&Item> = items
        .iter()
        .filter(|&f| f.type_ == ItemType::Armor)
        .collect();
    let rings: Vec<&Item> = items
        .iter()
        .filter(|&f| f.type_ == ItemType::Ring)
        .collect();

    let mut items_combinations: Vec<Vec<Option<&Item>>> = Vec::new();
    for &weapon in &weapons {
        for &armor in &armors {
            for &ring1 in &rings {
                for &ring2 in &rings {
                    if ring1 == ring2 {
                        continue;
                    }

                    items_combinations.push(vec![
                        Some(weapon),
                        Some(armor),
                        Some(ring1),
                        Some(ring2),
                    ]);

                    items_combinations.push(vec![Some(weapon), None, Some(ring1), Some(ring2)]);
                    items_combinations.push(vec![Some(weapon), None, None, Some(ring2)]);
                }

                items_combinations.push(vec![Some(weapon), Some(armor), Some(ring1), None]);
                items_combinations.push(vec![Some(weapon), None, Some(ring1), None]);
            }

            items_combinations.push(vec![Some(weapon), Some(armor), None, None]);
        }

        items_combinations.push(vec![Some(weapon), None, None, None]);
    }

    let empty_item = Item {
        name: String::new(),
        cost: 0,
        type_: ItemType::Armor,
        damage: 0,
        armor: 0,
    };
    let mut lowest_cost = 1000usize;
    let mut highest_cost = 0usize;
    for combination in &items_combinations {
        boss_character.reset();
        player_character.reset();

        let damage = combination
            .iter()
            .map(|f| f.unwrap_or(&empty_item).damage)
            .sum::<u8>();
        let armor = combination
            .iter()
            .map(|f| f.unwrap_or(&empty_item).armor)
            .sum::<u8>();
        let cost = combination
            .iter()
            .map(|f| f.unwrap_or(&empty_item).cost)
            .sum::<usize>();
        player_character.damage = damage;
        player_character.armor = armor;

        loop {
            attack(&player_character, &mut boss_character);
            if boss_character.hit_points == 0 {
                if cost < lowest_cost {
                    lowest_cost = cost;
                }
                break;
            }

            attack(&boss_character, &mut player_character);
            if player_character.hit_points == 0 {
                if cost > highest_cost {
                    highest_cost = cost;
                }
                break;
            }
        }
    }

    println!("Lowest gold spent to win: {lowest_cost}");
    println!("Highest gold spent to lose: {highest_cost}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn fight_test() {
        let inputs = fs::read_to_string("./test.txt").expect("Invalid file path argument!");
        let mut boss_character = Character::from_txt(&inputs);
        let mut player_character = Character::new();
        player_character.hit_points = 8;
        player_character.damage = 5;
        player_character.armor = 5;

        loop {
            attack(&player_character, &mut boss_character);
            if boss_character.hit_points == 0 {
                break;
            }

            attack(&boss_character, &mut player_character);
            if player_character.hit_points == 0 {
                break;
            }
        }

        println!("Player: {player_character:#?}");
        println!("Boss: {boss_character:#?}");
        assert_eq!(boss_character.hit_points, 0);
    }
}
