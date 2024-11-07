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

    let repository_element_list = scraper::Selector::parse("article.Box-row").unwrap();
    let mut repository_list: Vec<Repository> = Vec::new();

    for (index, repository_element) in document.select(&repository_element_list).enumerate() {
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
