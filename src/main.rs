mod teams;

use rand::{seq::SliceRandom, thread_rng};
use std::{collections::HashSet, vec};
use teams::{get_teams, Fixture, Pot, Team};
fn main() {
    let teams = get_teams();
    let mut fixtures: Vec<Fixture> = vec![];
    let curr_team = teams
        .choose(&mut thread_rng())
        .expect("no teams to choose from");
    // draw two teams from each pot

    // group teams into pots
    let pot_1_teams = filter_teams_by_pot(&teams, Pot::One);
    let pot_2_teams = filter_teams_by_pot(&teams, Pot::Two);
    let pot_3_teams = filter_teams_by_pot(&teams, Pot::Three);
    let pot_4_teams = filter_teams_by_pot(&teams, Pot::Four);

    println!("{} chosen", &curr_team.name);
    let pot1_home_fixture = Fixture{home: &curr_team, away: draw_team(&pot_1_teams)};
    let pot1_away_fixture = Fixture{away: &curr_team, home: draw_team(&pot_1_teams)};
    println!("Pot 1:");
    println!("{}", &pot1_home_fixture);
    println!("{}", &pot1_away_fixture);
    fixtures.push(pot1_home_fixture);
    fixtures.push(pot1_away_fixture);

}

fn filter_teams_by_pot(teams: &[Team], pot: Pot) -> Vec<&Team> {
    teams.iter().filter(|team| team.pot == pot).collect()
}

fn get_compatible_teams<'a>(
    team: &Team,
    teams: Vec<&'a Team>,
    fixtures: Vec<&Fixture>,
) -> Vec<&'a Team> {
    // remove current team (cant play yourself)

    // remove teams with a fixture already (can't play same team twice)
    let fixtures_for_curr_team = fixtures.iter().filter(|f| f.away == team || f.home == team);
    let teams_w_home_fixtures: Vec<&Team> =
        fixtures_for_curr_team.clone().map(|f| f.away).collect();
    let teams_w_away_fixtures: Vec<&Team> =
        fixtures_for_curr_team.clone().map(|f| f.home).collect();
    let all_existing_fixtures = [teams_w_away_fixtures, teams_w_home_fixtures].concat();

    let compat_teams = teams
        .iter()
        .filter(|t| !all_existing_fixtures.contains(t))
        .map(|t| *t)
        .collect();
    compat_teams
}

fn draw_team<'a>(compatible_teams: &'a [&Team]) -> &'a Team {
    compatible_teams
        .choose(&mut thread_rng())
        .expect("no teams available to draw from")
}
