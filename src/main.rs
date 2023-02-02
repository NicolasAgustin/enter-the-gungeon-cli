use std::env;
use enter_the_gungeon_cli::wiki_scraper::{GameItem, WikiScraper};

fn main() {

    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    if args.len() < 2 || args.len() > 2 {
        println!("Usage: etgcli [item name]");
        return;
    }

    let item_name = &args[1];

    let base_url = "https://enterthegungeon.fandom.com/wiki/".to_string();

    let mut wscraper = WikiScraper::new(base_url);

    let item: GameItem = wscraper.fetch_item_info(item_name);

    println!("{}", item.title);
    println!("{}", item.description);

}