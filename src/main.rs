mod secret;
mod entities;
mod crawler;

use std::path::{Path};
use std::fs::File;
use std::io::Read;
use entities::{Player, Position};

fn load_players() -> Vec<Player>
{
    const player_path: &str = "./data/players.json";

    if !Path::new(player_path).exists()
    {
        let players: Vec<Player> = crawler::crawl();
        let json: String = serde_json::to_string_pretty(&players).unwrap();
        std::fs::write(player_path, json);
        return players;
    }
    else
    {
        let mut players_file = File::open(player_path).expect("Where is my players.json?");

        let mut json = String::new();
        players_file.read_to_string(&mut json);

        let players: Vec<Player> = serde_json::from_str(&json).unwrap();
        return players;
    }
}

fn get_random_player_by_position(players: &Vec<Player>, position: Position)
{

}

fn load_teams() {}

fn main() 
{
    let players: Vec<Player> = load_players();
    load_teams();
}
