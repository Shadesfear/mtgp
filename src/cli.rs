use clap::{App, Arg};
pub fn generate_cli() -> App<'static> {
    let matches = App::new("mtg-pricer")
        .version("0.1")
        .author("Christopher Carman")
        .about("MTG price finder")
        .license("MIT")
        .arg(
            Arg::new("config")
                .about("Sets a config file to use")
                .default_value("config")
                .takes_value(true)
                .short('c')
                .long("config"),
        )
        .subcommand(
            App::new("search").about("Search a card").arg(
                Arg::new("search")
                    .about("Searches a card")
                    .index(1)
                    .required(true)
                    .min_values(1),
            ),
        )
        .subcommand(
            App::new("price").about("Gets price for card").arg(
                Arg::new("price")
                    .about("gets price for a card")
                    .index(1)
                    .required(true)
                    .min_values(1),
            ),
        );
    matches
}
