#![allow(dead_code)]
#![allow(unused_variables)]

// extern crate rand;
use rand::Rng;

use std::fmt;

// use std::io;
// struct Name {
//     _name : String
// }
//
// impl Name {
//     fn pretty_name (&self) -> String {
//         return "shoot".to_string();
//     }
// }
//
// struct Hero {
//     name: Name
//
// }


struct Hero {
    name: String,
    hp: i32,
    damage: i32,
}


impl std::fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.hp, self.damage)
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

fn main(){
    // //wtf no default constructors or anything. Rust sucks man. Josh Dec 20, 2018
    // let josh = Hero{name: Name{_name:"".to_string()}};

    let names = vec!["Josh", "Matt"];
    for name in names {
        let hero = build_hero(String::from(name));
        println!("{}: {}", hero.name, hero);
    }





    // let mut my_vector: Vec<i32> = Vec::new();
    // my_vector.push(1);
    // my_vector.push(2);
    // my_vector.push(3);
    //
    // for i in my_vector {
    //     println!("Poop {}", i);
    // }

    // println!("guess a number dude");
    // let mut guess_result = String::new();
    // io::stdin().read_line(&mut guess_result)
    //     .expect("Failed to read line");

    // if guess_result.trim() == "y".to_string() {
    //     println!("woooo");
    // } else {
    //     println!("type y instead of {}", guess_result);
    // };
}
