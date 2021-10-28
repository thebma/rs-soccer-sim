mod secret;
mod entities;
mod crawler;

use std::path::{Path};
use std::fs::File;
use std::io::Read;
use entities::{Position, Player, Team, TeamWithPlayers};
use rand::Rng;

fn load_players() -> Vec<Player>
{
    const PLAYER_PATH: &str = "./data/players.json";

    if !Path::new(PLAYER_PATH).exists()
    {
        let players: Vec<Player> = crawler::crawl();
        let json: String = serde_json::to_string_pretty(&players).unwrap();
        std::fs::write(PLAYER_PATH, json).unwrap();
        return players;
    }
    else
    {
        let mut players_file = File::open(PLAYER_PATH).expect("Where is my players.json?");

        let mut json = String::new();
        players_file.read_to_string(&mut json).unwrap();

        let players: Vec<Player> = serde_json::from_str(&json).unwrap();
        return players;
    }
}

fn load_teams() -> Vec<Team>
{
    const TEAMS_PATH: &str = "./data/teams.json";

    if Path::new(TEAMS_PATH).exists()
    {
        let mut teams_file = File::open(TEAMS_PATH)
            .expect("Where is my teams.json?");
    
        let mut json = String::new();
        teams_file.read_to_string(&mut json).unwrap();
    
        let teams: Vec<Team> = serde_json::from_str(&json).unwrap();
        return teams;
    }
    else
    {
        panic!("No teams.json was found!");
    }
}

fn load_standings() -> Vec<u32>
{
    const STANDING_PATH: &str = "./data/standings.json";

    if Path::new(STANDING_PATH).exists()
    {
        let mut standings_file = File::open(STANDING_PATH)
            .expect("Where is my standings.json?");
    
        let mut json = String::new();
        standings_file.read_to_string(&mut json).unwrap();
    
        let standings: Vec<u32> = serde_json::from_str(&json).unwrap();
        return standings;
    }
    else
    {
        panic!("No teams.json was found!");
    }
}

fn make_teams(in_players: Vec<Player>, in_teams: Vec<Team>) -> Vec<TeamWithPlayers>
{
    let mut teams = Vec::new();
    for team in in_teams { teams.push(team); }

    let mut players: Vec<Player> = Vec::new();
    for player in in_players { players.push(player); }

    let mut teams_with_players: Vec<TeamWithPlayers> = Vec::new();

    for team in teams 
    {
        let mut team_players = TeamWithPlayers {
            team: team,
            players: Vec::new()
        };

        let mut attacker_count = 4;
        let mut midfield_count = 3;
        let mut defender_count = 3;
        let mut goalie_count = 1;

        let mut index = 0;
        let mut rng = rand::thread_rng();
        let mut used: Vec<usize> = Vec::new();

        while attacker_count > 0 || midfield_count > 0 || defender_count > 0 || goalie_count > 0
        {
            if used.contains(&index)
            {
                index = (index + 1) % players.len();
                continue;
            }

            if let Some(player) = players.get(index)
            {
                let chance: u8 = rng.gen_range(0..100) as u8;

                if chance > 97
                {
                    match player.position 
                    {
                        Position::Attacker => 
                        { 
                            if attacker_count > 0
                            {
                                team_players.players.push(player.clone());
                                attacker_count -= 1;
                                used.push(index);
                            }
                        },
                        Position::Midfield => 
                        {
                            if midfield_count > 0
                            {
                                team_players.players.push(player.clone());
                                midfield_count -= 1;
                                used.push(index);
                            }
                        },
                        Position::Defender => 
                        {
                            if defender_count > 0
                            {
                                team_players.players.push(player.clone());
                                defender_count -= 1;
                                used.push(index);
                            }
                        },
                        Position::Goalkeeper => 
                        {
                            if goalie_count > 0
                            {
                                team_players.players.push(player.clone());
                                goalie_count -= 1;
                                used.push(index);
                            }
                        }
                        _ => { }
                    }
                }
            }

            index = (index + 1) % players.len();
        }

        teams_with_players.push(team_players);
    }

    return teams_with_players;
}

fn save_team_players(team_players: Vec<TeamWithPlayers>)
{
    const TEAM_PLAYER_PATH: &str = "./data/team_with_players.json";

    if Path::new(TEAM_PLAYER_PATH).exists()
    {
        std::fs::remove_file(TEAM_PLAYER_PATH).unwrap();
    }

    let json: String = serde_json::to_string_pretty(&team_players).unwrap();
    std::fs::write(TEAM_PLAYER_PATH, json).unwrap();
}

fn main() 
{
    let players: Vec<Player> = load_players();
    let teams: Vec<Team> = load_teams();

    let team_players: Vec<TeamWithPlayers> = make_teams(players, teams);
    save_team_players(team_players);

    let standings: Vec<u32> = load_standings();
    println!("{:?}", standings);
}
