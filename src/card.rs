use serde::Deserialize;
#[derive(Deserialize, Debug, Default)]
pub struct Card {
    pub name: String,
    pub mana_cost: String,
    pub type_line: String,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub oracle_text: String,
    pub prints_search_uri: String,
    pub prices: Prices,
    pub set_name: String,
    pub loyalty: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Cards {
    data: Vec<Card>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Prices {
    usd: Option<String>,
    usd_foil: Option<String>,
    eur: Option<String>,
    eur_foil: Option<String>,
}

impl Card {
    pub fn pretty_print(&self) {
        println!("",);
        print!("{}", self.name);
        print!("         {}", self.mana_cost);
        println!("",);
        println!("",);
        println!("",);
        println!("{}", self.type_line);
        println!("",);
        println!("{}", self.oracle_text);
        println!("",);
        match self {
            Card {
                power: Some(p),
                toughness: Some(t),
                ..
            } => {
                println!("{} / {}", p, t);
            }
            Card {
                power: Some(p),
                toughness: None,
                ..
            } => {
                println!("{} / N/A", p);
            }
            Card {
                power: None,
                toughness: Some(t),
                ..
            } => {
                println!("N/A / {}", t);
            }
            Card {
                loyalty: Some(l), ..
            } => {
                println!("Loyalty: {}", l);
            }
            _ => {
                println!("",);
            }
        }
    }

    #[tokio::main]
    pub async fn get_sets(&mut self) -> Result<Cards, Box<dyn std::error::Error + 'static>> {
        let response = reqwest::get(&self.prints_search_uri).await?;
        let cards = response.json::<Cards>().await?;
        Ok(cards)
    }
}

pub fn print_prices(cards: Cards, currency: String) {
    // TODO: Fix ugly code copy
    for card in cards.data.iter() {
        println!("{}", card.set_name);
        if currency == "usd".to_string() {
            match &card.prices {
                Prices {
                    usd: Some(u),
                    usd_foil: Some(uf),
                    ..
                } => println!("    {} / {}", u, uf),
                Prices {
                    usd: None,
                    usd_foil: Some(uf),
                    ..
                } => println!("    N/A / {}", uf),

                Prices {
                    usd: Some(u),
                    usd_foil: None,
                    ..
                } => println!("    {} / N/A", u),
                _ => println!("    N/A / N/A",),
            }
        } else {
            match &card.prices {
                Prices {
                    eur: Some(u),
                    eur_foil: Some(uf),
                    ..
                } => println!("    {} / {}", u, uf),
                Prices {
                    eur: None,
                    eur_foil: Some(uf),
                    ..
                } => println!("    N/A / {}", uf),

                Prices {
                    eur: Some(u),
                    eur_foil: None,
                    ..
                } => println!("    {} / N/A", u),
                _ => println!("    N/A / N/A",),
            }
        }
    }
}
