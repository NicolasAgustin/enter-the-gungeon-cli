use scraper;

fn main() {
    let base_url = "https://enterthegungeon.fandom.com/wiki/";

    let content = match reqwest::blocking::get(base_url.to_owned() + "Shotgun_Coffee") {
        Ok(v) => v.text().unwrap_or_else(|e| { panic!("Error al obtener el contenido: {}", e) }),
        Err(e) => panic!("Se produjo un error: {}", e)
    };

    let document = scraper::Html::parse_document(&content);

    let main_table_selector = scraper::Selector::parse("div.mw-parser-output")
        .unwrap_or_else(|e| { panic!("Error al obtener la tabla principal: {}", e) });

    let nodes = document.select(&main_table_selector).map(|node| { node.inner_html() });

    println!("CONTENIDO DE NODOS ENCONTRADOS");
    for n in nodes {
        println!("{}", n)
    }

}