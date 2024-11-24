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

    let mut developer_list: Vec<Developer> = Vec::new();

    let trending_header_selector = scraper::Selector::parse("h1").unwrap();
    let trending_header_element = document.select(&trending_header_selector)
        .find(|element| element.inner_html().trim() == "Trending");

    if trending_header_element.is_none() {
        return Err("Cannot find DOM node with GitHub trending header".into());
    }

    let empty_list_selector = scraper::Selector::parse(".blankslate").unwrap();
    if document.select(&empty_list_selector).next().is_some() {
        return Ok(developer_list);
    }

    let list_row_selector = scraper::Selector::parse("article.Box-row").unwrap();
    if document.select(&list_row_selector).count() == 0 {
        return Err("Cannot find DOM node with GitHub trending developer row".into());
    }

    for (index, developer_element) in document.select(&list_row_selector).enumerate() {
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
