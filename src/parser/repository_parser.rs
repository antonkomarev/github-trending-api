use std::error::Error;

pub struct Repository {
    pub full_name: String,
    pub rank: usize,
}

pub async fn parse(
    language: Option<String>,
) -> Result<Vec<Repository>, Box<dyn Error>> {
    let url = match language {
        Some(value) => format!("https://github.com/trending/{}?since=daily", value),
        None => "https://github.com/trending?since=daily".to_string(),
    };
    println!("Fetching repositories: {}", url);
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let response = http_client.get(url).send().await?.text().await?;
    let document = scraper::Html::parse_document(&response);

    let mut repository_list: Vec<Repository> = Vec::new();

    let trending_header_selector = scraper::Selector::parse("h1").unwrap();
    let trending_header_element = document.select(&trending_header_selector)
        .find(|element| element.inner_html().trim() == "Trending");

    if trending_header_element.is_none() {
        return Err("Cannot find DOM node with GitHub trending header".into());
    }

    let empty_list_selector = scraper::Selector::parse(".blankslate").unwrap();
    if document.select(&empty_list_selector).next().is_some() {
        return Ok(repository_list);
    }

    let list_row_selector = scraper::Selector::parse("article.Box-row").unwrap();
    if document.select(&list_row_selector).count() == 0 {
        return Err("Cannot find DOM node with GitHub trending repository row".into());
    }

    for (index, repository_element) in document.select(&list_row_selector).enumerate() {
        let full_name_element = repository_element
            .select(&scraper::Selector::parse("h2 a").unwrap())
            .next()
            .ok_or("Cannot find DOM node with GitHub trending repository full name")?;

        let full_name = full_name_element
            .value()
            .attr("href")
            .ok_or("Failed to get 'href' attribute")?
            .trim_start_matches('/')
            .to_string();

        repository_list.push(Repository {
            full_name,
            rank: index + 1,
        });
    }

    Ok(repository_list)
}
