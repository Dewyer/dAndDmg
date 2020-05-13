use std::collections::HashMap;
use crate::roller::Roll;

#[derive(Clone,Debug)]
pub struct Character
{
    pub name:String,
    pub health:i32,
    pub ac:i32,
    pub stats:StatBlock,
    pub proficiency_bonus:i32,
    pub attacks:Vec<Attack>
}

#[derive(Clone,Debug)]
pub struct Attack
{
    attack_roll:Roll,
    damage_roll:Roll
}

#[derive(Clone,PartialEq, Eq, Hash,Debug)]
pub enum Ability
{
    Dex,
    Int,
    Str,
    Chr,
    Wis,
    Con
}

#[derive(Clone,Debug)]
pub struct StatBlock
{
    values:HashMap<Ability,i32>    
}

impl StatBlock
{
    pub fn new(stre:i32,dex:i32,con:i32,int:i32,wis:i32,chr:i32) -> Self
    {
        let mut val:HashMap<Ability,i32> = HashMap::new();
        val.insert(Ability::Str,stre);
        val.insert(Ability::Dex,dex);
        val.insert(Ability::Con,con);
        val.insert(Ability::Int,int);
        val.insert(Ability::Wis,wis);
        val.insert(Ability::Chr,chr);

        StatBlock
        {
            values:val
        }
    }

    pub fn get_mod(&self,ab:Ability) -> i32
    {
        ((self.values.get(&ab).unwrap()-10) as f64 / 2.0).floor() as i32
    }
}

impl Character
{
    pub fn new(name:&str,health:i32,ac:i32,stats:StatBlock,prof:i32,attacks:Vec<Attack>) -> Self
    {
        Character
        {
            name:name.to_string(),
            health,
            ac,
            stats,
            proficiency_bonus:prof,
            attacks
        }
    }
}