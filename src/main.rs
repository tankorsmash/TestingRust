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
struct Actor {
    name: String,
    hp: i32,
    damage: i32,

    behaviours: Vec<Box<dyn Behaviour>>,

    backpack: Backpack
}

struct Backpack {
    food: String,
    // actor: *mut Actor,
    actor: Weak<RefCell<Actor>>

}

struct BehaveEat {
    // actor: *mut Actor,
    actor: Weak<RefCell<Actor>>
}

impl Behaviour for BehaveEat {
    fn do_behaviour(&self) -> bool {
        let actor =  self.actor.upgrade();
        match actor {
            Some(actor) => { return actor.borrow_mut().backpack.food == "orange"}
            None => { return false; }
        }
    }
}

pub trait Battler {
    fn do_battle(&self, other: &mut impl Battler) -> bool;
}


impl Battler for Actor {
    fn do_battle(&self, other: &mut impl Battler) -> bool {
        println!("battling!");
        return true;
    }
}

impl std::fmt::Display for Actor {
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

impl std::fmt::Display for Backpack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BEHAVE!")
    }
}

impl std::fmt::Debug for Backpack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BEHAVE debug!")
    }
}


fn build_actor(name: String) -> std::rc::Rc<std::cell::RefCell<Actor>> {
    let mut rng = rand::thread_rng();
    let hp =  rng.gen_range(5, 15);
    let damage =  rng.gen_range(1, 5);


    let actor = Rc::new(RefCell::new(Actor {
        name: name,
        hp: hp,
        damage: damage,
        behaviours: vec!(),
        backpack: Backpack {
            food: "apple".to_string(),
            actor: Weak::new()
        },
    }));
    actor.borrow_mut().backpack.actor = Rc::downgrade(&actor);

    return actor;
}

fn serialize() -> Result<String> {
    println!("begin serialize");

    let names = vec!["Josh", "Matt"];

    let mut actores = Vec::new();

    for name in names {
        let actor = build_actor(String::from(name));
        // println!("{:#?}", actor);

        actores.push(actor);
    }


    let josh = &actores[0];
    let mut matt = &actores[1];
    josh.borrow_mut().do_battle(&mut (*matt.borrow_mut()));

    // let data = serde_json::to_string(&josh)?;
    // println!("result:\n{}", data);
    // return Ok(data);

    return Ok(String::from("success"));
}

fn behave_testing()
{
    let res = serialize();
    let actor = build_actor(String::from("Hero"));
    // actor.behaviours.push(Box::new(BehaveEat{actor: Some(&mut actor)}));
    // for behaviour in actor.behaviours {
    //     let behave_result = behaviour.do_behaviour();
    //     println!("Behaviour {} resulted: '{}'", behaviour, behave_result);
    // }
}

struct Parent
{
    child: Child,
}

struct Child
{
    parent: Weak<RefCell<Parent>>,
}

pub trait Testing {
    fn testing(&self) -> ();
}

impl Testing for Parent {
    fn testing(&self) -> () {
        println!("testing");
    }
}

enum Direction {
    LEFT, RIGHT
}

fn main(){
    println!("Running...");
    serialize();

    // let mut parent = Rc::new(RefCell::new(Parent {
    //     child: Child{ parent: Weak::new() }
    // }));
    // parent.borrow_mut().child.parent = Rc::downgrade(&parent);
    // parent.borrow_mut().testing();

    println!("Done!");
}
