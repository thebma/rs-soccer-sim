use rand::Rng;

use crate::TeamWithPlayers;
use crate::{ Match, Player, Position, Goal, Team };

pub fn make_matches(teams: &Vec<TeamWithPlayers>) -> Vec<Match>
{
    let mut matches: Vec<Match> = Vec::new();

    for team_home in teams.clone()
    {
        let home_team = team_home.clone();

        for team_out in teams.clone()
        {
            if team_home.team.id == team_out.team.id {
                break;
            }

            let out_team = team_out.clone();
            
            let game_match: Match = Match {
                team_home: home_team.clone().team.id,
                team_out: out_team.clone().team.id,
                goals: Vec::new()
            };

            matches.push(game_match);
        }
    }
    
    return matches;
}

pub fn simulate(teams: &Vec<TeamWithPlayers>, standings: Vec<u32>) -> Vec<Match>
{
    let matches_to_play: Vec<Match> = make_matches(&teams);
    let mut matches_resolved: Vec<Match> = Vec::new();

    println!("We have {} matches to play.", matches_to_play.len());

    for game in matches_to_play
    {
        let game_ref = &mut game.clone();
        let resolved_match = simulate_match(teams, game_ref, &standings);
        matches_resolved.push(resolved_match);
    }

    return matches_resolved;
}

fn caculate(players: &Vec<Player>, home_min: f32, home_max: f32) -> (u32, u32, u32, u32)
{
    let mut sum_atk: u32 = 0;
    let mut sum_mid: u32 = 0;
    let mut sum_def: u32 = 0;
    let mut sum_goal: u32 = 0;

    let mut rng = rand::thread_rng();

    for player in players
    {
        let home_advantage = rng.gen_range(home_min..home_max) as f32;
        let player_score = (player.quality as f32 * home_advantage).round() as u32;

        match player.position
        {
            Position::Attacker => {
                sum_atk += player_score;
            },
            Position::Midfield => {
                sum_mid += player_score;
            },
            Position::Defender => {
                sum_def += player_score;
            },
            Position::Goalkeeper => {
                sum_goal = player_score;
            }
            _ => {}
        }
    }
    return (sum_atk, sum_mid, sum_def, sum_goal);
}

fn simulate_match(teams: &Vec<TeamWithPlayers>, game_match: &mut Match, standings: &Vec<u32>) -> Match
{
    /*
        The socccer field is layed out as following:
        0 = home_goal, 25 = home_mid, 50 = center, 75 = out mid, 100 = out goal
    */

    const MINUTES: u32 = 90;
    const STANDING_FACTOR_MIN: f32 = 1.2;
    const STANDING_FACTOR_MAX: f32 = 1.6;
    const HOME_FACTOR_MIN: f32 = 1.05;
    const HOME_FACTOR_MAX: f32 = 1.25;
    const HOME_PITCH: i32 = 45;
    const OUT_PITCH: i32 = 55;
    const GOAL_REBOUND: i32 = 25;

    //49 because home gets to kick the ball off the center always.
    let mut field: i32 = HOME_PITCH;
    let mut rng = rand::thread_rng();

    let mut goals_out = 0;
    let mut goals_home = 0;

    for minute in 0 .. MINUTES
    {
        let standing_advantage = rng.gen_range(STANDING_FACTOR_MIN..STANDING_FACTOR_MAX) as f32;
        let home_team_standing = (*standings.get(game_match.team_home as usize - 1).unwrap() as f32 * standing_advantage).floor() as u32;
        let out_team_standing = (*standings.get(game_match.team_out as usize - 1).unwrap() as f32 * standing_advantage).floor() as u32;

        let (home_players, out_players) = &game_match.get_players(*teams);

        let (home_atk, home_mid, home_def, home_goal) = caculate(home_players, HOME_FACTOR_MIN, HOME_FACTOR_MAX);
        let (out_atk, out_mid, out_def, out_goal) = caculate(out_players, HOME_FACTOR_MIN, HOME_FACTOR_MAX);

        //println!("{} {} {} {} vs {} {} {} {}", home_atk, home_def, home_mid, home_goal, out_atk, out_def, out_mid, out_goal);
        
        let home_variance_upper = home_team_standing as f32 / 1000.0;
        let out_variance_upper = out_team_standing as f32 / 1000.0;

        let home_variance = rng.gen_range(0.92..1.0 + home_variance_upper) as f32;
        let out_variance = rng.gen_range(0.92..1.0 + out_variance_upper) as f32;

        //Handle logic if home team is attacking.
        if field > 75 && field < 100
        {
            let mut home_attack_score = (home_atk as f32+ home_mid as f32 * 0.33).round() as i32;
            home_attack_score = (home_attack_score as f32 * home_variance).round() as i32;

            let mut out_defending_score = (out_def as f32 + out_mid as f32 * 0.33).round() as i32;
            out_defending_score = (out_defending_score as f32 * out_variance).round() as i32;

            let delta = (home_attack_score as i32 - out_defending_score as i32).abs();

            if out_defending_score > home_attack_score {
                field -= delta
            }
            else {
                field += delta;
            }
        }
        //Handle logic when both team are in the mid field.
        else if field < 75 && field > 25
        {
            let home_mid_score = (home_mid as f32 * home_variance).round() as i32;
            let out_mid_score = (out_mid as f32 * out_variance).round() as i32;
            let delta = (home_mid_score as i32 - out_mid_score as i32).abs();

            if out_mid_score > home_mid_score {
                field -= delta;
            }
            else {
                field += delta;
            }
        }
        //Handle logic when out is attacking
        else if field > 0 && field < 25
        {
            let mut out_attack_score = (out_atk as f32+ out_mid as f32 * 0.33).round() as i32;
            out_attack_score = (out_attack_score as f32 * out_variance).round() as i32;

            let mut home_defending_score = (home_def as f32 + home_mid as f32 * 0.33).round() as i32;
            home_defending_score = (home_defending_score as f32 * home_variance).round() as i32;

            let delta = (home_defending_score as i32 - out_attack_score as i32).abs();

            if home_defending_score > out_attack_score {
                field += delta
            }
            else {
                field -= delta;
            }
        }
        else if field > 100 || field < 0
        {
            let goal_pick = rng.gen_range(0..100) as u32;
            let goal_variance = rng.gen_range(0.9..1.1) as f32;
            
            let home_goal_score = (home_goal as f32 * goal_variance).round() as u32;
            let out_goal_score = (out_goal as f32 * goal_variance).round() as u32;


            if field > 100 
            {
                if goal_pick > home_goal_score
                {
                    let team_copy: Team = game_match.team_home.team.clone();
                    let scorerer: Player = game_match.team_home.get_random_player(Position::Attacker);
                    let home_goal = Goal { time: minute, team_id: team_copy.id, player_id: scorerer.id };
                    game_match.add_goal(home_goal);

                    goals_home += 1;
                    field = OUT_PITCH;
                }
                else
                {
                    field -= GOAL_REBOUND;
                }
            }
            else
            {
                if goal_pick > out_goal_score 
                {
                    let team_copy: Team = game_match.team_out.clone()
                    let scorerer: Player = game_match.team_out.get_random_player(Position::Attacker);
                    let out_goal = Goal { time: minute, team_id: team_copy.id, player_id: scorerer.id };
                    game_match.add_goal(out_goal);
                    goals_out += 1;
                    field = HOME_PITCH;
                }
                else
                {
                    field += GOAL_REBOUND;
                }
            }
        }
    }

    println!("{} vs. {} {} - {} ", 
        game_match.team_home.team_home,  
        game_match.team_out.team_out, 
        goals_home, goals_out
    );

    return game_match.clone();
}
