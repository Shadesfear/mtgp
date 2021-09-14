#![allow(dead_code)]

use reqwest;
use std::collections::HashMap;
mod card;
mod cli;
mod config_reader;
use card::Card;

use crate::card::print_prices;

#[tokio::main]
async fn get_card(url: String) -> Result<Card, Box<dyn std::error::Error>> {
    let response = reqwest::get(&url).await?;
    let card = response.json::<Card>().await?;
    Ok(card)
}

fn handle_price(request_url: String, currency: String) {
    match get_card(request_url) {
        Ok(mut c) => {
            let cards = c.get_sets().unwrap();
            print_prices(cards, currency)
        }
        Err(e) => println!("{}", e),
    }
}

fn handle_search(request_url: String) {
    match get_card(request_url) {
        Ok(c) => {
            c.pretty_print();
        }
        Err(e) => println!("{}", e),
    }
}

fn main() -> Result<(), ()> {
    let matches = cli::generate_cli().get_matches();
    let mut conf: HashMap<String, String> = config_reader::config_reader::read_config();

    if matches.is_present("config") {
        conf.insert(
            "config".to_string(),
            matches.value_of("config").unwrap().to_string(),
        );
    } else {
        todo!("Check if config config path is valid")
    }

    if let Some(matches) = matches.subcommand_matches("search") {
        let search_term: String = matches
            .values_of("search")
            .unwrap()
            .collect::<Vec<_>>()
            .join("+");
        let request_url = format!("{}{}", conf.get("api_url").unwrap(), search_term);
        handle_search(request_url)
    }

    if let Some(matches) = matches.subcommand_matches("price") {
        let search_term: String = matches
            .values_of("price")
            .unwrap()
            .collect::<Vec<_>>()
            .join("+");
        let request_url = format!("{}{}", conf.get("api_url").unwrap(), search_term);
        handle_price(request_url, conf.get("currency").unwrap().to_string())
    }

    // if let Some(matches) = matches.subcommand_matches("price") {
    //     let price_term: String = matches
    //         .values_of("price")
    //         .unwrap()
    //         .collect::<Vec<_>>()
    //         .join("+");
    //     let request_url = format!("{}{}", conf.get("api_url").unwrap(), price_term);
    //     handle_price(request_url, "usd".to_string());
    // }

    // TODO: Get match approach to work!
    // match matches.subcommand_name() {
    //     Some("search") => {
    //         let search_term: String = matches
    //             .values_of("search")
    //             .unwrap()
    //             .collect::<Vec<_>>()
    //             .join("+");
    //         let request_url = format!("{}{}", conf.get("api_url").unwrap(), search_term);
    //         handle_search(request_url)
    //     }
    //     Some("price") => {
    //         let search_term: String = matches
    //             .values_of("search")
    //             .unwrap()
    //             .collect::<Vec<_>>()
    //             .join("+");
    //         let request_url = format!("{}{}", conf.get("api_url").unwrap(), search_term);
    //         handle_price(request_url, conf.get("currency").unwrap().to_string())
    //     }
    //     _ => println!("Nothing",),
    // }
    Ok(())
}
