use std::error::Error;

pub struct Developer {
    pub username: String,
    pub rank: usize,
}

pub async fn parse(
    language: Option<String>,
) -> Result<Vec<Developer>, Box<dyn Error>> {
    let url = match language {
        Some(value) => format!("https://github.com/trending/developers/{}?since=daily", value),
        None => "https://github.com/trending/developers?since=daily".to_string(),
    };
    println!("Fetching developers: {}", url);
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let response = http_client.get(url).send().await?.text().await?;
    let document = scraper::Html::parse_document(&response);

    let developer_element_list = scraper::Selector::parse("article.Box-row").unwrap();
    let mut developer_list: Vec<Developer> = Vec::new();

    for (index, developer_element) in document.select(&developer_element_list).enumerate() {
        let username_element = developer_element
            .select(&scraper::Selector::parse("h1 a").unwrap())
            .next()
            .ok_or("Cannot find DOM node with GitHub trending developer username")?;

        let username = username_element
            .value()
            .attr("href")
            .ok_or("Failed to get 'href' attribute")?
            .trim_start_matches('/')
            .to_string();

        developer_list.push(Developer {
            username,
            rank: index + 1,
        });
    }

    Ok(developer_list)
}
