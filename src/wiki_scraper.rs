use std::error::Error;

use scraper::{self, Selector, ElementRef};

pub struct GameItem {
    pub title: String,
    pub description: String,
    pub effects: String,
    pub notes: String
}

pub struct WikiScraper {
    base_url: String
}

impl WikiScraper {

    pub fn get_main_content_selector(&self) -> Selector {
        scraper::Selector::parse("div.mw-parser-output")
            .unwrap_or_else(|e| { panic!("Error al obtener la tabla principal: {}", e) })
    }

    pub fn get_title(&self, main_document: &ElementRef) -> String {
        let title_selector = scraper::Selector::parse("#firstHeading")
            .unwrap_or_else(|e| { panic!("Error al obtener el titulo: {}", e) });

        let title;

        match main_document.select(&title_selector).next() {
            Some(v) => {
                title = v.first_child()
                    .unwrap()
                    .value()
                    .as_text()
                    .unwrap().to_string()
            },
            None => panic!("Error al obtener el titulo")
        };

        title
    }

    pub fn get_description(&self, main_document: &ElementRef) -> String {
        let description_selector = scraper::Selector::parse("table + p")
            .unwrap_or_else(|err| { panic!("Error al obtener la descripcion: {}", err) });

        let mut all_content = String::new();

        {
            let content_str = &mut all_content;
            main_document.select(&description_selector).for_each(
                |elem| {
                    let extracted_text = elem.text().collect::<Vec<_>>();
                    *content_str = content_str.to_owned() + &extracted_text.join("") + "\n";
                }
            );
        }
        all_content
    }

    pub fn get_effects(&mut self, main_document: &ElementRef) -> String {
        let effects_selector = scraper::Selector::parse("h2 > #Effects")
            .unwrap_or_else(|err| { panic!("Error al obtener la descripcion: {}", err) });

        let mut all_content = String::new();
        {
            let content_str = &mut all_content;
            main_document.select(&effects_selector).for_each(
                |elem| {
                    let extracted_text = elem.text().collect::<Vec<_>>();
                    *content_str = content_str.to_owned() + &extracted_text.join("") + "\n";
                }
            )
        }

        let mut effects_content = String::new();
        {
            let effects_str = &mut effects_content;
            main_document.select(&Selector::parse("h2 + ul").unwrap()).for_each(
                |elem| {
                    let extracted_text = elem.text().collect::<Vec<_>>();
                    *effects_str = effects_str.to_owned() + &extracted_text.join("") + "\n";
                }
            )
        }

        all_content.to_owned().to_ascii_uppercase() + &effects_content
    }

    pub fn fetch_item_info(&mut self, item_name: &String) -> Result<GameItem, Box<dyn Error>> {

        let mut item: GameItem = GameItem { 
            title: "".to_string(),
            description: "".to_string(),
            effects: "".to_string(),
            notes: "".to_string()
        };

        let content = match reqwest::blocking::get(self.base_url.to_owned() + item_name) {
            Ok(v) => v.text()
                .unwrap_or_else(
                    |e| { panic!("Error: {}", e) }
                ),
            Err(e) => panic!("Error: {}", e)
        };

        let document = scraper::Html::parse_document(&content);
        let header = document.select(
            &scraper::Selector::parse("div.page-header").unwrap()
        ).next()
        .unwrap();

        let main_table_selector = self.get_main_content_selector();

        let main = document.select(&main_table_selector).next().unwrap();

        let title = self.get_title(&header);
        let description = self.get_description(&main);
        let effects = self.get_effects(&main);

        /*
            TODO:
            Obtener effectos
            Obtener notas
        */

        println!("{}", title.trim().to_ascii_uppercase());
        println!("{}", description.trim());

        println!("\n{}", effects.trim());

        Ok(item)

    }

    pub fn new(base_url: String) -> Self {
        WikiScraper { base_url: base_url }
    }

}


