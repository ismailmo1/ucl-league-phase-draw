use std::{fmt, hash::Hash};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Pot {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug)]
pub struct Fixture<'a> {
    pub home: &'a Team,
    pub away: &'a Team,
}

impl fmt::Display for Fixture<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(H) v {}(A)", self.home, self.away)
    }
}
#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub league: League,
    pub pot: Pot,
}

impl Team {
    pub fn can_draw(&self, other: &Team) -> bool {
        if &self.league == &other.league {
            return false;
        } else {
            return true;
        }
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_league_cant_draw() {
        let team1 = Team {
            name: String::from("team1"),
            league: League::AUT,
            pot: Pot::One,
        };
        let team2 = Team {
            name: String::from("team2"),
            league: League::AUT,
            pot: Pot::Two,
        };
        let can_draw = team1.can_draw(&team2);
        assert_eq!(can_draw, false);
    }
    #[test]
    fn different_league_can_draw() {
        let team1 = Team {
            name: String::from("team1"),
            league: League::AUT,
            pot: Pot::One,
        };
        let team2 = Team {
            name: String::from("team2"),
            league: League::ENG,
            pot: Pot::One,
        };
        let can_draw = team1.can_draw(&team2);
        assert_eq!(can_draw, true);
    }
}

pub fn get_teams() -> Vec<Team> {
    vec![
        Team {
            name: String::from("Liverpool"),
            league: League::ENG,
            pot: Pot::One,
        },
        Team {
            name: String::from("Barcelona"),
            league: League::ESP,
            pot: Pot::One,
        },
        Team {
            name: String::from("Arsenal"),
            league: League::ENG,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Bayer Leverkusen"),
            league: League::GER,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Aston Villa"),
            league: League::ENG,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Inter Milan"),
            league: League::ITA,
            pot: Pot::One,
        },
        Team {
            name: String::from("Brest"),
            league: League::FRA,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Lille"),
            league: League::FRA,
            pot: Pot::Three,
        },
        Team {
            name: String::from("Borussia Dortmund"),
            league: League::FRA,
            pot: Pot::One,
        },
        Team {
            name: String::from("Bayern Munich"),
            league: League::GER,
            pot: Pot::One,
        },
        Team {
            name: String::from("Atlético Madrid"),
            league: League::ESP,
            pot: Pot::Two,
        },
        Team {
            name: String::from("AC Milan"),
            league: League::ITA,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Atalanta"),
            league: League::ITA,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Juventus"),
            league: League::ITA,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Benfica"),
            league: League::POR,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Monaco"),
            league: League::FRA,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Sporting CP"),
            league: League::POR,
            pot: Pot::Three,
        },
        Team {
            name: String::from("Feyenoord"),
            league: League::NED,
            pot: Pot::Three,
        },
        Team {
            name: String::from("Club Brugge"),
            league: League::POR,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Real Madrid"),
            league: League::ESP,
            pot: Pot::One,
        },
        Team {
            name: String::from("Celtic"),
            league: League::SCO,
            pot: Pot::Three,
        },
        Team {
            name: String::from("Manchester City"),
            league: League::ENG,
            pot: Pot::One,
        },
        Team {
            name: String::from("PSV"),
            league: League::NED,
            pot: Pot::Three,
        },
        Team {
            name: String::from("Dinamo Zagreb"),
            league: League::CRO,
            pot: Pot::Three,
        },
        Team {
            name: String::from("PSG"),
            league: League::FRA,
            pot: Pot::One,
        },
        Team {
            name: String::from("Stuttgart"),
            league: League::GER,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Shakhtar Donetsk"),
            league: League::UKR,
            pot: Pot::Two,
        },
        Team {
            name: String::from("Sparta Prague"),
            league: League::CZE,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Sturm Graz"),
            league: League::AUT,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Girona"),
            league: League::ESP,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Red Star Belgrade"),
            league: League::SRB,
            pot: Pot::Three,
        },
        Team {
            name: String::from("Red Bull Salzburg"),
            league: League::AUT,
            pot: Pot::Three,
        },
        Team {
            name: String::from("Bologna"),
            league: League::ITA,
            pot: Pot::Four,
        },
        Team {
            name: String::from("RB Leipzig"),
            league: League::GER,
            pot: Pot::One,
        },
        Team {
            name: String::from("Slovan Bratislava"),
            league: League::SVK,
            pot: Pot::Four,
        },
        Team {
            name: String::from("Young Boy"),
            league: League::SUI,
            pot: Pot::Three,
        },
    ]
}
