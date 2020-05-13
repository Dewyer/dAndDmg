use crate::character::{Character,Ability};
use crate::roller::Roll;

pub struct Simulation
{
    teams:Vec<Team>
}

#[derive(Clone)]
pub struct Team
{
    name:String,
    characters:Vec<Character>
}

pub struct Outcome
{
    team_index_won:i32
}

impl Simulation
{
    //Todo : Add non random seeds later
    /*
    pub fn new(characters:&Vec<Character>,seed:i32) -> Self
    {

    }
    */

    pub fn new(teams:Vec<Team>) -> Self
    {
        Simulation
        {
            teams:teams.clone()
        }
    }

    pub fn get_flat_characters(&self) -> Vec<&Character>
    {
        let mut all_chars:Vec<&Character> = Vec::new();

        for team in self.teams.iter()
        {
            for cc in team.characters.iter()
            {
                all_chars.push(&cc);
            }
        }
        all_chars
    }

    pub fn get_initiative_order(&self) -> Vec<&Character>
    {
        let mut initiative_ord:Vec<(i32,&Character)> = Vec::new();           
        let all_chars = self.get_flat_characters();

        for cc in all_chars
        {
            let initiative = Roll::roll_dice(20) + cc.stats.get_mod(Ability::Dex);
            initiative_ord.push((initiative,cc))
        }
        initiative_ord.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());

        initiative_ord.into_iter().map(|e| e.1).rev().collect()
    }

    pub fn simulate(&mut self) -> Outcome
    {
        let initiatve_order = self.get_initiative_order();
        println!("Initivate order got : {:?}",initiatve_order);

        Outcome
        {
            team_index_won:0
        }
    }
}