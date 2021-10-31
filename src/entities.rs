use serde::{ Serialize, Deserialize };
use std::str::{ FromStr };
use rand::Rng;


#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Position 
{
    Attacker,
    Midfield,
    Defender,
    Goalkeeper,
    NotOnTheField
}

impl Default for Position
{
    fn default() -> Self
    {
        Position::NotOnTheField
    }
}

impl FromStr for Position 
{
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        match string.to_lowercase().as_ref() {
            "av" => { Ok(Position::Attacker) },
            "vd" => { Ok(Position::Defender) },
            "mv" => { Ok(Position::Midfield) },
            "kp" => { Ok(Position::Goalkeeper) },
            _ => { Ok(Position::NotOnTheField) }
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Player
{
    pub id: u32,
    pub name: String,
    pub dob: String,
    pub height: String,
    pub position: Position,
    pub quality: u8
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Surface
{
    Unknown,
    Grass,
    ArtificialGrass,
    Hybrid
}

impl FromStr for Surface
{
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        match string.to_lowercase().as_ref() {
            "grass" => { Ok(Surface::Grass) },
            "kunstgras" => { Ok(Surface::ArtificialGrass) },
            "hybride" => { Ok(Surface::Hybrid) },
            _ => { Ok(Surface::Unknown) }
        }
    }
}

impl Default for Surface {
    fn default() -> Self { Surface::Unknown }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Team
{
    pub id: u32,
    pub name: String,
    pub city: String,
    pub stadium: String,
    pub surface: Surface,
    pub since: u32
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct TeamWithPlayers
{
    pub team: Team,
    pub players: Vec<Player>
}

impl TeamWithPlayers 
{
    pub fn get_random_player(&self, position: Position) -> Player
    {
        let mut player: Player = Player{ ..Default::default() };
        let mut index = 0;

        let mut rng = rand::thread_rng();

        loop
        {
            let current_player = self.players.get(index);

            if let Some(current_found_player) = current_player
            {
                if current_found_player.position == position 
                {
                    let random = rng.gen_range(0..100);

                    if random > 90 
                    {
                        player = current_found_player.clone();
                        break;
                    }
                }
            }

            index = (index + 1) % self.players.len();
        }

        return player;
    }
}


#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Goal
{
    pub time: u32,
    pub team_id: u32,
    pub player_id: u32
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Match 
{
    pub team_home: u32,
    pub team_out: u32,
    pub goals: Vec<Goal>,
}

impl Match 
{
    pub fn add_goal(&mut self, goal: Goal)
    {
        self.goals.push(goal);
    }

    pub fn get_team(&self, teams: &Vec<TeamWithPlayers>, id: u32) -> Option<TeamWithPlayers>
    {
        for team in teams 
        {
            if team.team.id == id
            {
                return Some(team.clone());
            }
        }

        None
    }

    pub fn get_players(&self, teams: &Vec<TeamWithPlayers>) -> (Vec<Player>, Vec<Player>)
    {   
        let mut team_home_players: Vec<Player> = Vec::new();
        let mut team_out_players: Vec<Player> = Vec::new();

        if let Some(home_team) = self.get_team(teams, self.team_home)
        {
            for player in home_team.players 
            {
                team_home_players.push(player);
            }
        }

        if let Some(out_team) = self.get_team(teams, self.team_out)
        {
            for player in out_team.players 
            {
                team_out_players.push(player);
            }
        }

        return (team_home_players, team_out_players)
    }
}