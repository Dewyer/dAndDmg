mod character;
mod roller;
mod simulator;

use crate::character::{Character,StatBlock};

fn main() {
    println!("Hello, dnd!");

    let me = Character::new(
        "Barna",
        20, 
        17,
        StatBlock::new(20, 18, 14, 10, 12, 10),
        2,
        vec![]
    );

    let goblin = Character::new(
        "Goblin",
        7,
        12,
        StatBlock::new(10, 10, 11, 7, 7, 7),
        2,
        vec![ ]
    );

    let players = vec![me];
    let monsters =  
}
