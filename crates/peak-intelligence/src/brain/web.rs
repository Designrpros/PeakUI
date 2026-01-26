use crate::brain::assistant::Message;
use crate::brain::{Assistant, Error, Url};

use sipper::{sipper, Sipper, Straw};

pub struct Search {
    pub results: Vec<Url>,
}

#[derive(Debug, Clone)]
pub struct Summary {
    pub url: Url,
    pub content: String,
}

impl Summary {
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub async fn search(query: &str) -> Result<Search, Error> {
    log::info!("Searching on DuckDuckGo: {query}");

    let mut headers = std::collections::HashMap::new();
    headers.insert(
        "User-Agent".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36".to_string(),
    );
    headers.insert("Accept".to_string(), "*/*".to_string());

    let url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        urlencoding::encode(query)
    );

    let response = crate::http::HttpClient::get_with_headers(&url, headers).await?;
    let search_results = response
        .text()
        .map_err(|e| Error::WasmError(format!("Invalid text response: {}", e)))?;

    let html = scraper::Html::parse_document(&search_results);
    let selector = scraper::Selector::parse(".result__a").unwrap();

    let results = html
        .select(&selector)
        .filter_map(|link| {
            let encoded = link.attr("href")?;

            if encoded.contains("ad_domain") {
                return None;
            }

            let query_pairs = url::form_urlencoded::parse(encoded.as_bytes());
            for (_key, value) in query_pairs {
                if let Ok(url) = url::Url::parse(&value) {
                    return Some(url);
                }
            }
            None
        })
        .take(5)
        .collect();

    log::info!("-- Found: {results:?}");

    Ok(Search { results })
}

pub fn summarize<'a>(
    assistant: &'a Assistant,
    query: &'a str,
    url: Url,
) -> impl Straw<Summary, Summary, Error> + 'a {
    sipper(move |sender| async move {
        let text = scrape(url.clone()).await?;

        let reply = assistant
            .clone()
            .reply(
                "You are a helpful assistant.".to_string(),
                vec![Message::User(format!(
                    "```\n\
                    {text}\n\
                    ```\n\n\
                    Please, summarize the parts of the previous text that \
                    are relevant to the query: \"{query}\"."
                ))],
                vec![],
            )
            .with(|(reply, _token)| Summary {
                url: url.clone(),
                content: reply.content,
            })
            .run(sender)
            .await?;

        Ok(Summary {
            url,
            content: reply.content,
        })
    })
}

async fn scrape(url: Url) -> Result<String, Error> {
    log::info!("Scraping text: {url}");

    let candidates = scraper::Selector::parse("p, a").unwrap();

    let mut headers = std::collections::HashMap::new();
    headers.insert(
        "User-Agent".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36".to_string(),
    );

    let response = crate::http::HttpClient::get_with_headers(url.as_str(), headers).await?;
    let html = response
        .text()
        .map_err(|e| Error::WasmError(format!("Invalid text response: {}", e)))?;

    log::info!("-- HTML retrieved ({} chars)", html.len());
    log::trace!("{html}");

    let html = scraper::Html::parse_document(&html);

    let lines = html
        .select(&candidates)
        .flat_map(|candidate| candidate.text())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>();

    log::info!("-- Scraped {} lines of text", lines.len());

    Ok(lines.join("\n"))
}
