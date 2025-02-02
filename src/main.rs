use rand::{thread_rng, Rng};
use std::collections::HashSet;
use ucl_league_phase_draw::{filter_teams_by_pot, get_teams, Fixture, Pot};
fn main() {
    let mut teams = get_teams();
    let mut fixtures: HashSet<Fixture> = HashSet::new();
    // group teams into pots
    let pot_1_teams = filter_teams_by_pot(&teams, Pot::One);
    let pot_2_teams = filter_teams_by_pot(&teams, Pot::Two);
    let pot_3_teams = filter_teams_by_pot(&teams, Pot::Three);
    let pot_4_teams = filter_teams_by_pot(&teams, Pot::Four);

    for pot in [&pot_1_teams, &pot_2_teams, &pot_3_teams, &pot_4_teams] {
        for curr_team in pot {
            println!("{} chosen", &curr_team.name);
            for (idx, pot) in [&pot_1_teams, &pot_2_teams, &pot_3_teams, &pot_4_teams]
                .iter()
                .enumerate()
            {
                // TODO: check if team already has a fixture for the pot so we dont attempt to draw again
                // i.e. if a team already has a home fix for with pot 2 skip this step
                println!("drawing for {} in pot {}", curr_team, idx + 1);
                // TODO: remove teams from pot that already have a fixture from the curr_teams pot
                match curr_team.draw_opponent(&pot, &fixtures, true) {
                    Ok(fix) => {
                        println!("away: {}", fix);
                        fixtures.insert(fix.clone())
                    }
                    Err(_) => continue,
                };
                match curr_team.draw_opponent(&pot, &fixtures, false) {
                    Ok(fix) => {
                        println!("home: {}", fix);
                        fixtures.insert(fix.clone())
                    }
                    Err(_) => continue,
                };
            }
        }
    }
    println!("fixture list:",);
    for t in get_teams() {
        println!("{}", "-".repeat(10));
        println!("{}", t);
        println!("{}", "-".repeat(10));
        for f in fixtures.iter().filter(|f| f.has_team(&t)) {
            println!("{}", f);
        }
    }
}
