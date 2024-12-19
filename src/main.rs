mod teams;

use rand::{seq::SliceRandom, thread_rng};
use std::{collections::HashSet, vec};
use teams::{get_teams, Fixture, Pot, Team};
fn main() {
    let teams = get_teams();
    let mut fixtures: Vec<Fixture> = vec![];
    let curr_team = teams.choose(&mut thread_rng()).expect("no teams to choose from");
    // draw two teams from each pot

    // group teams into pots
    let pot_1_teams = filter_teams_by_pot(&teams, Pot::One);
    let pot_2_teams = filter_teams_by_pot(&teams, Pot::Two);
    let pot_3_teams = filter_teams_by_pot(&teams, Pot::Three);
    let pot_4_teams = filter_teams_by_pot(&teams, Pot::Four);

    println!("{} chosen", &curr_team.name);
    let pot1_home_fixture = generate_fixture_for_team(&curr_team, &pot_1_teams);
    let pot1_away_fixture = generate_fixture_for_team(&curr_team, &pot_1_teams);
    println!("Pot 1:");
    println!("{}", &pot1_home_fixture);
    println!("{}", &pot1_away_fixture);
    fixtures.push(pot1_home_fixture);
    fixtures.push(pot1_away_fixture);

    let pot2_home_fixture = generate_fixture_for_team(&curr_team, &pot_2_teams);
    let pot2_away_fixture = generate_fixture_for_team(&curr_team, &pot_2_teams);
    println!("Pot 2:");
    println!("{}", &pot2_home_fixture);
    println!("{}", &pot2_away_fixture);
    fixtures.push(pot2_home_fixture);
    fixtures.push(pot2_away_fixture);

    let pot3_home_fixture = generate_fixture_for_team(&curr_team, &pot_3_teams);
    let pot3_away_fixture = generate_fixture_for_team(&curr_team, &pot_3_teams);
    println!("Pot 3:");
    println!("{}", &pot3_home_fixture);
    println!("{}", &pot3_away_fixture);
    fixtures.push(pot3_home_fixture);
    fixtures.push(pot3_away_fixture);

    let pot4_home_fixture = generate_fixture_for_team(&curr_team, &pot_4_teams);
    let pot4_away_fixture = generate_fixture_for_team(&curr_team, &pot_4_teams);
    println!("Pot 4:");
    println!("{}", &pot4_home_fixture);
    println!("{}", &pot4_away_fixture);
    fixtures.push(pot4_home_fixture);
    fixtures.push(pot4_away_fixture);
}

fn filter_teams_by_pot(teams: &[Team], pot: Pot) -> Vec<&Team> {
    teams.iter().filter(|team| team.pot == pot).collect()
}

fn get_incompatible_teams<'a>(team:&'a Team, fixtures: Vec<&'a Fixture>) ->Vec<&'a Fixture<'a>>{
    // remove current team (cant play yourself)
    let mut incompat_teams= vec![team];

    // remove teams with a fixture already (can't play same team twice) 
    let fixtures_for_curr_team = fixtures.iter().filter(|f| f.away == team || f.home == team);
    let teams_w_home_fixtures: Vec<&Team> = fixtures_for_curr_team.clone().map(|f| f.away).collect();
    let teams_w_away_fixtures: Vec<&Team> = fixtures_for_curr_team.clone().map(|f| f.home).collect();
    let all_existing_fixtures = [teams_w_away_fixtures, teams_w_home_fixtures].concat();
    let existing_fixture_teams: HashSet<&&Team> =all_existing_fixtures.iter().collect();
    incompat_teams.extend(existing_fixture_teams);
    fixtures
}

fn generate_fixture_for_team<'a>(
    team: &'a Team,
    compatible_teams: &'a [&Team],
) -> Fixture<'a> {

    // remove 
    let draw = compatible_teams
        .choose(&mut thread_rng())
        .expect("no teams available to draw from");

    let fixture = Fixture {
        home: &draw,
        away: &team,
    };
    return fixture;
}
