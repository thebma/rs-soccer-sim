use crate::entities::Player;
use crate::entities::Position;

use scraper::{ Html, Selector };
use rand::Rng;
use std::str::FromStr;
use crate::secret;

pub fn crawl() -> Vec<Player>
{   
    const MAX_PAGES: u32 = 11;

    let mut all_players: Vec<Player> = Vec::new();

    for page in 1..MAX_PAGES+1 {
        let players = crawl_page(page);

        for player in players {
            all_players.push(player);
        }
    }

    println!("{}", all_players.len());

    return all_players;
}

fn crawl_page(i: u32) -> Vec<Player>
{
    let mut players: Vec<Player> = Vec::new();

    let url: String = secret::get_vicitim().to_owned();
    let page_url = url + "/" + i.to_string().as_ref();

    let page_content: String = reqwest::blocking::
        get(page_url).unwrap().text().unwrap();

    let document = Html::parse_document(page_content.as_str());

    //Loop over tables....
    let table_selector = Selector::parse("table.standard_tabelle").unwrap();
    for table in document.select(&table_selector) 
    {
        let mut values: Vec<String> = Vec::new();

        //Loop over table rows...
        let table_row_selector = Selector::parse("tr").unwrap();
        for table_row in table.select(&table_row_selector)
        {
            //Loop over table date...
            let table_data_selector = Selector::parse("td").unwrap();
            for table_data in table_row.select(&table_data_selector)
            {
                let text = table_data.inner_html().replace("\r", "").replace("\n", "");

                //Stip <a> tags if we have one...
                if text.contains("<a href") && !text.contains("<img")
                {
                    let text_a = table_data.select(&Selector::parse("a").unwrap()).next().unwrap();
                    let text = text_a.inner_html().replace("\r", "").replace("\n", "");
                    values.push(text);   
                }
                else if !text.contains("<img")
                {
                    values.push(text);
                }
            }

            let mut rng = rand::thread_rng();

            if values.len() >= 5 
            {
                let player: Player = Player {
                    name: values.get(0).unwrap().to_owned(),
                    dob: values.get(2).unwrap().to_owned(),
                    height: values.get(3).unwrap().to_owned(),
                    position: Position::from_str(values.get(4).unwrap()).unwrap(),
                    quality: rng.gen_range(0..100) as u8
                };

                players.push(player);
            }
            
            values.clear();
        }
    }

    return players;
}