use serde::{ Serialize, Deserialize };
use std::str::{ FromStr };

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub name: String,
    pub dob: String,
    pub height: String,
    pub position: Position,
    pub quality: u8
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Team
{
    pub id: u32,
    pub name: String,
    pub city: String,
    pub stadium: String,
    pub surface: Surface,
    pub since: u32
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TeamWithPlayers
{
    pub team: Team,
    pub players: Vec<Player>
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Goal
{
    pub time: u32,
    pub scorer: Player,
    pub for_team: Team,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Match 
{
    pub team_home: Team,
    pub team_out: Team,
    pub goals: Vec<Goal>,
}