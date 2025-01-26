use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum League {
    ENG,
    ESP,
    ITA,
    GER,
    FRA,
    NED,
    POR,
    CZE,
    CRO,
    SRB,
    SCO,
    UKR,
    AUT,
    SUI,
    SVK,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pot {
    One,
    Two,
    Three,
    Four,
}
impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pot::One => write!(f, "{}", 1),
            Pot::Two => write!(f, "{}", 2),
            Pot::Three => write!(f, "{}", 3),
            Pot::Four => write!(f, "{}", 4),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Fixture {
    pub home: Team,
    pub away: Team,
}

impl Fixture {
    pub fn has_team(&self, team: &Team) -> bool {
        (&self.home == team) || (&self.away == team)
    }
}
impl fmt::Display for Fixture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(H) v {}(A)", self.home, self.away)
    }
}
#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,
    pub league: League,
    pub pot: Pot,
}

impl Team {
    pub fn new(name: &str, league: League, pot: Pot) -> Team {
        Team {
            name: String::from(name),
            league,
            pot,
        }
    }
    pub fn can_draw(&self, other: &Team) -> bool {
        if self.league == other.league {
            return false;
        } else {
            return true;
        }
    }
    pub fn get_compatible_teams(&self, teams: Vec<Team>, fixtures: Vec<&Fixture>) -> Vec<Team> {
        // remove current team (cant play yourself)
        let mut incompat_teams = vec![self.clone()];
        // remove teams with a fixture already (can't play same team twice)
        let fixtures_for_curr_team = fixtures
            .iter()
            .filter(|f| f.away == *self || f.home == *self);
        let teams_w_home_fixtures: Vec<Team> = fixtures_for_curr_team
            .clone()
            .map(|f| f.away.clone())
            .collect();
        let teams_w_away_fixtures: Vec<Team> = fixtures_for_curr_team
            .clone()
            .map(|f| f.home.clone())
            .collect();
        let all_existing_fixture_teams = [teams_w_away_fixtures, teams_w_home_fixtures].concat();
        incompat_teams.extend_from_slice(all_existing_fixture_teams.as_slice());

        let compat_teams = teams
            .iter()
            .filter(|t| !incompat_teams.contains(t))
            .map(|t| t.clone())
            .collect();
        compat_teams
    }

    fn draw_random_team<'a>(&self, teams_to_draw_from: &'a Vec<Team>) -> &'a Team {
        teams_to_draw_from
            .choose(&mut thread_rng())
            .expect("no teams available to draw from")
    }

    pub fn draw_opponent(
        &self,
        teams_to_draw_from: &Vec<Team>,
        curr_fixtures: &HashSet<Fixture>,
        home: bool,
    ) -> Fixture {
        let valid_teams_to_draw: Vec<Team> = teams_to_draw_from
            .iter()
            .filter(|t| self.is_opponent_valid(t, home, curr_fixtures))
            .cloned()
            .collect();
        println!("valid teams for {}", self);
        for t in &valid_teams_to_draw {
            println!("{}", t);
        }
        // need to remove invalid teams from teams_to_draw from so we dont keep drawing them
        let opponent = self.draw_random_team(&valid_teams_to_draw);
        if home {
            Fixture {
                home: self.clone(),
                away: opponent.clone(),
            }
        } else {
            Fixture {
                away: self.clone(),
                home: opponent.clone(),
            }
        }
    }
    fn is_opponent_valid(
        &self,
        opponent: &Team,
        home: bool,
        curr_fixtures: &HashSet<Fixture>,
    ) -> bool {
        // cannot play yourself
        if opponent == self {
            // congratulations, you played yourself
            return false;
        }
        // cannot play team from same league
        if opponent.league == self.league {
            return false;
        }
        // cannot play the same team twice
        if self.has_fixture(opponent, curr_fixtures) {
            return false;
        }
        // cannot play opponents who already have an equivalent fixture with a team from that pot
        // equivalent fix = same pot & same home/away
        for fix in curr_fixtures {
            // if were checking if the opponent is valid for a home fixtures, this will be an away fixture for the opponent
            // so we need to check the opponents current away fixtures
            if home {
                if &fix.away == opponent && fix.home.pot == self.pot {
                    println!(
                        "invalid {} for home={} due to fixture: {} for pot {}",
                        opponent, home, fix, self.pot
                    );
                    return false;
                }
            } else if &fix.home == opponent && fix.away.pot == self.pot {
                // this is an away fixture for self, so we need to check opponents home fixtures
                println!(
                    "invalid {} for home={} due to fixture: {} for pot {}",
                    opponent, home, fix, self.pot
                );
                return false;
            }
        }
        // cannot play more than two teams from the same league
        let mut league_counts = self.get_league_counts(curr_fixtures);
        if *league_counts.entry(&opponent.league).or_insert(0) > 1 {
            return false;
        }
        true
    }

    fn get_league_counts<'a>(
        &self,
        curr_fixtures: &'a HashSet<Fixture>,
    ) -> HashMap<&'a League, i32> {
        let mut league_counts = HashMap::new();
        for fix in curr_fixtures {
            if fix.has_team(self) {
                let opp;
                if &fix.home == self {
                    opp = &fix.away;
                } else {
                    opp = &fix.home;
                }
                league_counts
                    .entry(&opp.league)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
        league_counts
    }

    fn has_fixture(&self, opponent: &Team, curr_fixtures: &HashSet<Fixture>) -> bool {
        curr_fixtures.contains(&Fixture {
            home: opponent.clone(),
            away: self.clone(),
        }) || curr_fixtures.contains(&Fixture {
            home: self.clone(),
            away: opponent.clone(),
        })
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Team {}
impl Hash for Team {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub fn filter_teams_by_pot(teams: &[Team], pot: Pot) -> Vec<Team> {
    teams
        .iter()
        .filter(|team| team.pot == pot)
        .map(|t| t.clone())
        .collect()
}

pub fn get_teams() -> Vec<Team> {
    vec![
        Team::new("Liverpool", League::ENG, Pot::One),
        Team::new("Barcelona", League::ESP, Pot::One),
        Team::new("Arsenal", League::ENG, Pot::Two),
        Team::new("Bayer Leverkusen", League::GER, Pot::Two),
        Team::new("Aston Villa", League::ENG, Pot::Four),
        Team::new("Inter Milan", League::ITA, Pot::One),
        Team::new("Brest", League::FRA, Pot::Four),
        Team::new("Lille", League::FRA, Pot::Three),
        Team::new("Borussia Dortmund", League::FRA, Pot::One),
        Team::new("Bayern Munich", League::GER, Pot::One),
        Team::new("Atlético Madrid", League::ESP, Pot::Two),
        Team::new("AC Milan", League::ITA, Pot::Two),
        Team::new("Atalanta", League::ITA, Pot::Two),
        Team::new("Juventus", League::ITA, Pot::Two),
        Team::new("Benfica", League::POR, Pot::Two),
        Team::new("Monaco", League::FRA, Pot::Four),
        Team::new("Sporting CP", League::POR, Pot::Three),
        Team::new("Feyenoord", League::NED, Pot::Three),
        Team::new("Club Brugge", League::POR, Pot::Two),
        Team::new("Real Madrid", League::ESP, Pot::One),
        Team::new("Celtic", League::SCO, Pot::Three),
        Team::new("Manchester City", League::ENG, Pot::One),
        Team::new("PSV", League::NED, Pot::Three),
        Team::new("Dinamo Zagreb", League::CRO, Pot::Three),
        Team::new("PSG", League::FRA, Pot::One),
        Team::new("Stuttgart", League::GER, Pot::Four),
        Team::new("Shakhtar Donetsk", League::UKR, Pot::Two),
        Team::new("Sparta Prague", League::CZE, Pot::Four),
        Team::new("Sturm Graz", League::AUT, Pot::Four),
        Team::new("Girona", League::ESP, Pot::Four),
        Team::new("Red Star Belgrade", League::SRB, Pot::Three),
        Team::new("Red Bull Salzburg", League::AUT, Pot::Three),
        Team::new("Bologna", League::ITA, Pot::Four),
        Team::new("RB Leipzig", League::GER, Pot::One),
        Team::new("Slovan Bratislava", League::SVK, Pot::Four),
        Team::new("Young Boys", League::SUI, Pot::Three),
    ]
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn same_league_cant_draw() {
        let team1 = Team::new("team1", League::AUT, Pot::One);
        let team2 = Team::new("team2", League::AUT, Pot::Two);
        let can_draw = team1.can_draw(&team2);
        assert_eq!(can_draw, false);
    }
    #[test]
    fn different_league_can_draw() {
        let team1 = Team::new("team1", League::AUT, Pot::One);
        let team2 = Team::new("team2", League::ENG, Pot::One);
        let can_draw = team1.can_draw(&team2);
        assert_eq!(can_draw, true);
    }
    #[test]
    fn same_fixture_equals() {
        let team1 = Team::new("team1", League::AUT, Pot::One);
        let team2 = Team::new("team1", League::AUT, Pot::One);
        let fix1 = Fixture {
            home: team1.clone(),
            away: team2.clone(),
        };
        let fix2 = Fixture {
            home: team1.clone(),
            away: team2.clone(),
        };
        assert_eq!(fix1, fix2)
    }

    #[test]
    fn opponent_is_not_valid_against_same_team() {
        // you cant draw a team against themselves
        let team1 = Team::new("team1", League::AUT, Pot::One);
        let team1_copy = team1.clone();
        let curr_fix = HashSet::new();
        let is_valid = team1.is_opponent_valid(&team1_copy, true, &curr_fix);
        assert_eq!(is_valid, false)
    }
    #[test]
    fn opponent_is_valid_empty_fixtures() {
        // you cant draw a team that already has a home fixture
        let team1 = Team::new("team1", League::AUT, Pot::One);
        let team2 = Team::new("team2", League::CRO, Pot::Two);
        let mut curr_fix = HashSet::new();
        curr_fix.insert(Fixture {
            home: team1.clone(),
            away: team2.clone(),
        });
        let is_valid = team1.is_opponent_valid(&team2, true, &curr_fix);
        assert_eq!(is_valid, false)
    }
    #[test]
    fn opponent_is_not_valid_if_already_drawn_home() {
        // you cant draw a team that already has a home fixture
        let team1 = Team::new("team1", League::AUT, Pot::One);
        let team2 = Team::new("team2", League::CRO, Pot::Two);
        let mut curr_fix = HashSet::new();
        curr_fix.insert(Fixture {
            home: team1.clone(),
            away: team2.clone(),
        });
        let is_valid = team1.is_opponent_valid(&team2, true, &curr_fix);
        assert_eq!(is_valid, false)
    }
    #[test]
    fn opponent_is_not_valid_if_already_drawn_away() {
        // you cant draw a team that already has a away fixture
        let team1 = Team::new("team1", League::AUT, Pot::One);
        let team2 = Team::new("team2", League::CRO, Pot::Two);
        let mut curr_fix = HashSet::new();
        curr_fix.insert(Fixture {
            home: team2.clone(),
            away: team1.clone(),
        });
        let is_valid = team1.is_opponent_valid(&team2, true, &curr_fix);
        assert_eq!(is_valid, false)
    }

    #[test]
    fn opponent_is_not_valid_if_max_league_allocation_full() {
        // you cant draw a team that already has fixtures with two teams from that league
        let team1 = Team::new("team1", League::ESP, Pot::One);
        let team2 = Team::new("team2", League::ENG, Pot::Two);
        let team3 = Team::new("team3", League::ENG, Pot::Three);
        let team4 = Team::new("team4", League::ENG, Pot::Four);
        let mut curr_fix = HashSet::new();
        curr_fix.insert(Fixture {
            home: team1.clone(),
            away: team2.clone(),
        });
        curr_fix.insert(Fixture {
            home: team1.clone(),
            away: team3.clone(),
        });
        let is_valid = team1.is_opponent_valid(&team4, true, &curr_fix);
        assert_eq!(is_valid, false)
    }
    #[test]
    fn opponent_is_not_valid_against_same_league() {
        // you cant draw a team that already has fixtures with two teams from that league
        let team1 = Team::new("team1", League::ENG, Pot::One);
        let team2 = Team::new("team2", League::ENG, Pot::Two);

        let is_valid = team1.is_opponent_valid(&team2, true, &HashSet::new());
        assert_eq!(is_valid, false)
    }
    #[test]
    fn opponent_is_not_valid_if_already_drawn_pot_home() {
        // you cant draw a team away from home if that team already has a home fixture with a team from that pot
        let team1 = Team::new("team1", League::ENG, Pot::One);
        let team2 = Team::new("team2", League::ESP, Pot::Two);
        let team3 = Team::new("team3", League::GER, Pot::Two);
        let mut curr_fix = HashSet::new();
        // team 1 has a home fixture against a pot two team
        curr_fix.insert(Fixture {
            home: team1.clone(),
            away: team2.clone(),
        });
        // we cant draw team 1 away from home, since that would be another home fixture for team 1 against a pot two team
        let is_valid = team3.is_opponent_valid(&team1, false, &curr_fix);
        assert_eq!(is_valid, false)
    }
    #[test]
    fn opponent_is_not_valid_if_already_drawn_pot_away() {
        // you cant draw a team at home if that team already has an away fixture with a team from that pot
        let team1 = Team::new("team1", League::ENG, Pot::One);
        let team2 = Team::new("team2", League::ESP, Pot::Two);
        let team3 = Team::new("team3", League::GER, Pot::Two);
        let mut curr_fix = HashSet::new();
        // team 1 has an away fixture with a pot two teams
        curr_fix.insert(Fixture {
            home: team2.clone(),
            away: team1.clone(),
        });
        // we cant draw team 1 at home, since that would be another away fixture for team 1 against a pot two team
        let is_valid = team3.is_opponent_valid(&team3, true, &curr_fix);
        assert_eq!(is_valid, false)
    }
}
