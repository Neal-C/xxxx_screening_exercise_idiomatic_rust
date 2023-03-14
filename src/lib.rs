#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_return)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::hash_map::{Keys, Values};
use std::collections::{HashMap, HashSet};
use std::collections::btree_set::BTreeSet;
use std::rc::{Rc};

#[derive(Debug, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
struct ChessPlayer {
    ratio: u16,
    age: u16,
    name: String
}

impl ChessPlayer {
    const fn new<'player>(name: String, ratio: u16, age: u16) -> Self {
        return Self {
            ratio,
            name,
            age
        };
    }
}

impl ChessPlayer {

    fn match_up<'player>(self, other_player: Rc<ChessPlayer>) -> Rc<Self>{

        if self.ratio > other_player.ratio {
            return Rc::new(self);
        } else {
            return other_player;
        }
    }

    const fn is_draw_against_player<'player>(&'player self, other_player: &Self) -> bool {
        return self.ratio == other_player.ratio;
    }

    const fn is_eliminated_by<'player>(&'player self, other_player : &Self) -> bool {
        return (self.age >= other_player.age && self.ratio < other_player.ratio) || (self.age >=
            other_player.age && self.ratio == other_player.ratio)
    }
}


fn get_champions(participants: Vec<ChessPlayer>) -> Vec<Rc<ChessPlayer>> {
    if participants.is_empty() {
        return vec![];
    }


    let mut record  :HashMap<u16,Rc<ChessPlayer>> = HashMap::new();
    let mut draws = Vec::new();

    for player in participants {
        let category = player.age;
        let current_champion = record
            .entry(category)
            .or_insert_with(|| Rc::new(ChessPlayer::default
                ()));

        if player.is_draw_against_player(current_champion){
            draws.push(Rc::clone(current_champion));
            draws.push(Rc::new(player));
            continue;
        }
        let strongest_at_category_age = player.match_up(Rc::clone(current_champion));
        record.insert(category, strongest_at_category_age);
    }

    let mut bests_by_age= record.values().collect::<Vec<_>>();
    for player in &draws {
        bests_by_age.push(player);
    }


    let mut bests_by_age = Vec::from_iter(BTreeSet::<_>::from_iter(bests_by_age));

    bests_by_age.sort_by_key(|element| element.age);

    let mut ascending_ordered_ages= record.keys().collect::<Vec<_>>();
    ascending_ordered_ages.sort();
    // let ascending_ordered_ages = ascending_ordered_ages.iter();
    let mut champions_list = Vec::new();

    for player in &bests_by_age {

        for age_category in &ascending_ordered_ages {
            if(**age_category == player.age) {
                champions_list.push(Rc::clone(player));
                break;
            }
            if(player.is_eliminated_by(record.get(age_category).unwrap())){
                break;
            }
        }

    }

    return champions_list;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mary_and_peter_are_champions(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Jean"), 1000, 10),
            ChessPlayer::new(String::from("mary"), 1100, 9),
            ChessPlayer::new(String::from("peter"), 1200, 11)
        ];
        let karl_and_lebron= vec![
            Rc::new(ChessPlayer::new(String::from("mary"), 1100, 9)),
            Rc::new(ChessPlayer::new(String::from("peter"), 1200, 11))
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, karl_and_lebron);
    }

    #[test]
    fn a_non_empty_vec_must_return_champions(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Sherlock"), 2000, 30),
            ChessPlayer::new(String::from("Magnus"), 3100, 30),
            ChessPlayer::new(String::from("Francis"), 2200, 10)
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert!(!result.is_empty());
    }

    #[test]
    fn return_goats(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Sherlock"), 38_000, 30),
            ChessPlayer::new(String::from("Magnus"), 42_000, 75),
            ChessPlayer::new(String::from("Francis"), 37_000, 18),
            ChessPlayer::new(String::from("James Moriarty"), 32_000, 31),
            ChessPlayer::new(String::from("Erik Lehnsherr"), 31_000, 32),
            ChessPlayer::new(String::from("Daniil Dubov"), 33_000, 33),
        ];
        let goats= vec![
            Rc::new(ChessPlayer::new(String::from("Francis"), 37_000, 18)),
            Rc::new(ChessPlayer::new(String::from("Karl"), 38_000, 30)),
            Rc::new(ChessPlayer::new(String::from("Magnus"), 42_000, 75)),
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, goats)
    }

    #[test]
    fn account_for_draws(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Sherlock"), 40_000, 30),
            ChessPlayer::new(String::from("Magnus"), 42_000, 75),
            ChessPlayer::new(String::from("Francis"), 37_000, 18),
            ChessPlayer::new(String::from("Moriary"), 40_000, 30),
            ChessPlayer::new(String::from("Daniil"), 35_000, 40),
            ChessPlayer::new(String::from("Erik"), 32_000, 31),
        ];
        let goats= vec![
            Rc::new(ChessPlayer::new(String::from("Francis"), 37_000, 18)),
            Rc::new(ChessPlayer::new(String::from("Sherlock"), 40_000, 30)),
            Rc::new(ChessPlayer::new(String::from("Moriarty"), 40_000, 30)),
            Rc::new(ChessPlayer::new(String::from("Magnus"), 42_000, 75))
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, goats)
    }

}