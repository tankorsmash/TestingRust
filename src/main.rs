#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use serde::{Deserialize, Serialize};
use serde_json::Result;

use rand::Rng;

use std::fmt;
use std::io;

use std::cell::Cell;
use std::rc::{Rc, Weak};
use std::cell::RefCell;



// #[derive(Serialize, Deserialize, Debug)]
// #[derive(Debug)]
pub trait Behaviour {
    fn do_behaviour(&self) -> bool { return true;}
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
struct Actor<'a> {
    name: String,
    hp: i32,
    damage: i32,

    behaviours: Vec<Box<dyn Behaviour>>,

    backpack: Backpack<'a>
}

struct Backpack<'a> {
    food: String,
    // actor: *mut Actor,
    actor: Option<&'a Actor<'a>>

}

struct BehaveEat<'a> {
    // actor: *mut Actor,
    actor: Option<&'a Actor<'a>>
}

impl Behaviour for BehaveEat<'_> {
    fn do_behaviour(&self) -> bool {
        match self.actor {
            Some(actor) => {
                return actor.backpack.food == "orange"
                // return true
            },
            None => { return false }
        }
        // return self.actor.backpack.food == "ss"
        // return false;
    }
}

pub trait Battler {
    fn do_battle(&self, other: &mut impl Battler) -> bool;
}


impl Battler for &Actor<'_> {
    fn do_battle(&self, other: &mut impl Battler) -> bool {
        return true;
    }
}

impl std::fmt::Display for Actor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Actor: \"{}\", HP: {}, DMG: {}", self.name, self.hp, self.damage)
    }
}

impl std::fmt::Display for dyn Behaviour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BEHAVE!")
    }
}

impl std::fmt::Debug for dyn Behaviour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BEHAVE debug!")
    }
}

impl std::fmt::Display for Backpack<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BEHAVE!")
    }
}

impl std::fmt::Debug for Backpack<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BEHAVE debug!")
    }
}


fn build_actor<'a>(name: String) -> Actor<'a> {
    let mut rng = rand::thread_rng();
    let hp =  rng.gen_range(5, 15);
    let damage =  rng.gen_range(1, 5);


    let actor = Actor {
        name: name,
        hp: hp,
        damage: damage,
        behaviours: vec!(),
        backpack: Backpack {
            food: "apple".to_string(),
            actor: None
        },
    };

    return actor;
}

fn serialize() -> Result<String> {

    let names = vec!["Josh", "Matt"];

    let mut actores: Vec<Actor> = Vec::new();

    for name in names {
        let actor = build_actor(String::from(name));
        // println!("{:#?}", actor);

        actores.push(actor);
    }


    let josh : &Actor = &actores[0];
    let mut matt: &Actor = &actores[1];
    josh.do_battle(&mut matt);

    // let data = serde_json::to_string(&josh)?;
    // println!("result:\n{}", data);
    // return Ok(data);

    return Ok(String::from("success"));
}

fn behave_testing()
{
    // let res = serialize();
    let actor = build_actor(String::from("Hero"));
    // actor.behaviours.push(Box::new(BehaveEat{actor: Some(&mut actor)}));
    for behaviour in actor.behaviours {
        let behave_result = behaviour.do_behaviour();
        println!("Behaviour {} resulted: '{}'", behaviour, behave_result);
    }
}

struct Parent
{
    child: Child,
}

struct Child
{
    parent: Weak<RefCell<Parent>>,
}


fn main(){
    println!("Running...");

    let mut parent = Rc::new(RefCell::new(Parent {
        child: Child{ parent: Weak::new() }
    }));
    parent.borrow_mut().child.parent = Rc::downgrade(&parent);

    println!("Done!");
}
