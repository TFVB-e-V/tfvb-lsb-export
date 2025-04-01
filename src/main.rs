use std::{collections::BTreeMap, fs};

use chrono::Datelike;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Gender {
    Male,
    Female,
    Diverse,
}

struct Player {
    first: String,
    last: String,
    gender: Gender,
    club: String,
    year: u32,
}

#[derive(Default)]
struct PlayerCount([u32; 3]);

#[derive(Default)]
struct Club {
    total: PlayerCount,
    to6: PlayerCount,
    to14: PlayerCount,
    to18: PlayerCount,
    to20: PlayerCount,
    to26: PlayerCount,
    to40: PlayerCount,
    to50: PlayerCount,
    to60: PlayerCount,
    older: PlayerCount,
}

impl Club {
    fn add_player(&mut self, player: &Player) {
        let add = |pc: &mut PlayerCount| {
            pc.0[player.gender as usize] += 1;
        };
        add(&mut self.total);
        let curr_year = chrono::Utc::now().year() as u32;
        let age = curr_year - player.year;
        add(match age {
            0..=6 => &mut self.to6,
            7..=14 => &mut self.to14,
            15..=18 => &mut self.to18,
            19..=20 => &mut self.to20,
            21..=26 => &mut self.to26,
            27..=40 => &mut self.to40,
            41..=50 => &mut self.to50,
            51..=60 => &mut self.to60,
            _ => &mut self.older,
        });
    }

    fn print(&self, name: &str) {
        print!("{}\t{}\t", name, self.total.0.iter().sum::<u32>());

        let print = |pc: &PlayerCount| print!("{}\t{}\t{}\t", pc.0[0], pc.0[1], pc.0[2]);
        print(&self.total);
        print(&self.to6);
        print(&self.to14);
        print(&self.to18);
        print(&self.to20);
        print(&self.to26);
        print(&self.to40);
        print(&self.to50);
        print(&self.to60);
        print(&self.older);
        println!();
    }
}

impl Player {
    fn from(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split("\t").collect();
        if parts.len() != 11 {
            return None;
        }

        let first = parts[1].to_string();
        let last = parts[0].to_string();
        let gender = match parts[4] {
            "M" => Gender::Male,
            "W" => Gender::Female,
            _ => Gender::Diverse,
        };
        let mut club = parts[6].trim().to_string();
        if club.starts_with('"') && club.ends_with('"') {
            club = club[1..club.len() - 2].to_string();
        }
        let year = parts[10].trim().parse().ok();

        year.map(|year| Self {
            first,
            last,
            gender,
            club,
            year,
        })
    }
}

fn main() {
    let players: Vec<_> = fs::read_to_string("input.csv")
        .expect("failed to read input.csv")
        .split("\n")
        .filter_map(|line| Player::from(line))
        .collect();

    let mut total = Club::default();
    let mut clubs = BTreeMap::<_, Club>::new();
    for player in &players {
        clubs
            .entry(player.club.clone())
            .or_default()
            .add_player(player);
        total.add_player(player);
    }

    for (name, club) in clubs {
        club.print(&name);
    }
    total.print("Gesamtmitglieder");
}
