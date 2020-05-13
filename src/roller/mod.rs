use rand::distributions::{Distribution, Uniform};

#[derive(Clone,Debug)]
pub struct Roll
{
    dices:Vec<(i32,i32)>
}

impl Roll
{
    pub fn roll_dice(d:i32) -> i32
    {
        let mut rng = rand::thread_rng();
        Uniform::from(1..(d+1)).sample(&mut rng)
    }
}