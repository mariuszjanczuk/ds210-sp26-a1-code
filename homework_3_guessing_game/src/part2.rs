use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        let mid = min + (max - min) / 2;
        let mut comparison = player.ask_to_compare(mid);
        if comparison < 0{
            return Self::guess_the_number(player, min, mid);
        } else if comparison == 0 {
            return mid
        } else if comparison > 0{
            return Self::guess_the_number(player, mid+1, max);
        } else {
            return 1000;
        }
    }
}
