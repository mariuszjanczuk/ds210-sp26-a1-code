use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part1 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part1 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE. 
       for number in min..max {
    if player.ask_if_equal(number) {
        return number;
    }
}
return 0;



        
//For part 1, you are tasked to implement the following strategy:
// Iterate/loop over numbers between min (inclusive) and max (exclusive).
// For each of these possibilities, ask the player if the possibility is equal to their number.
// If it is, congratulations, you found the answer! return it.
// If it is not, continue iterating to the next guess.
    }
}
