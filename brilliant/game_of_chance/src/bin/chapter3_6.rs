extern crate rand;

fn random_piston(count: usize) -> Vec<bool> {
    use rand::prelude::*;
    let mut rng = thread_rng();
    let mut piston = vec![false; count];
    let idx = rng.gen_range(0, count);
    for idx in [idx, idx + 1].into_iter() {
        piston[idx % count] = true;
    }
    piston
}
fn main() {
    let tries = 100000;
    let random_piston_first_player_not_dead = |count| {
        (0..)
            .map(|_| random_piston(count))
            .filter(|piston| !piston[0])
            .nth(0)
            .unwrap()
    };
    let start_a_death_count = (0..tries)
        .map(|_| {
            let piston = random_piston_first_player_not_dead(6);
            if piston[1] {
                1
            } else {
                0
            }
        }).sum::<usize>();

    let start_b_death_count = (0..tries)
        .map(|_| {
            let piston = random_piston(6);
            if piston[1] {
                1
            } else {
                0
            }
        }).sum::<usize>();

    let death_odds_a = start_a_death_count as f32 / tries as f32;
    let death_odds_b = start_b_death_count as f32 / tries as f32;
    println!("Chance of death");
    println!("Strat a {:?}", death_odds_a);
    println!("Strat b {:?}", death_odds_b);
}
