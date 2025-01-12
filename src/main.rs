mod teams;

use rand::{thread_rng, Rng};
use teams::{filter_teams_by_pot, get_teams, Fixture, Pot};
fn main() {
    let mut teams = get_teams();
    let mut fixtures: Vec<Fixture> = vec![];
    // group teams into pots
    let pot_1_teams = filter_teams_by_pot(&teams, Pot::One);
    let pot_2_teams = filter_teams_by_pot(&teams, Pot::Two);
    let pot_3_teams = filter_teams_by_pot(&teams, Pot::Three);
    let pot_4_teams = filter_teams_by_pot(&teams, Pot::Four);

    while teams.len() > 0 {
        let rnd_idx = thread_rng().gen_range(0..teams.len());
        let curr_team = teams[rnd_idx].clone();
        teams.remove(rnd_idx);
        println!("{} chosen", &curr_team.name);
        for pot in [&pot_1_teams, &pot_2_teams, &pot_3_teams, &pot_4_teams] {
            // TODO: remove teams from pot that already have a fixture from the curr_teams pot
            let home_fixture = curr_team.draw_opponent(&pot, true);
            let away_fixture = curr_team.draw_opponent(&pot, false);
            fixtures.push(home_fixture);
            fixtures.push(away_fixture);
        }
    }
    for fx in fixtures {
        println!("{}", fx);
    }
}
