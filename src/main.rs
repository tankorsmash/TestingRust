#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use serde::{Deserialize, Serialize};
use serde_json::Result;

use rand::Rng;

use std::fmt;
use std::io;

#[derive(Serialize, Deserialize)]
struct Hero {
    name: String,
    hp: i32,
    damage: i32,
}

pub trait Battler {
    fn do_battle(&self, other: &mut impl Battler) -> bool;
}


impl Battler for &Hero {
    fn do_battle(&self, other: &mut impl Battler) -> bool {
        return true;
    }
}

impl std::fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hero: \"{}\", HP: {}, DMG: {}", self.name, self.hp, self.damage)
    }
}


fn build_hero(name: String) -> Hero {
    let mut rng = rand::thread_rng();
    let hp =  rng.gen_range(5, 15);
    let damage =  rng.gen_range(1, 5);


    let hero = Hero {
        name: name,
        hp: hp,
        damage: damage
    };

    return hero;
}

fn serialize() -> Result<String> {

    let names = vec!["Josh", "Matt"];

    let mut heroes: Vec<Hero> = Vec::new();

    for name in names {
        let hero = build_hero(String::from(name));
        println!("{}", hero);

        heroes.push(hero);
    }


    let josh : &Hero = &heroes[0];
    let mut matt: &Hero = &heroes[1];
    josh.do_battle(&mut matt);

    let data = serde_json::to_string(&josh)?;
    println!("result:\n{}", data);

    return Ok(data);
}

fn main(){
    // //wtf no default constructors or anything. Rust sucks man. Josh Dec 20, 2018
    // let josh = Hero{name: Name{_name:"".to_string()}};

    let res = serialize();

}
