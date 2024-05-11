use rocket::serde::{Deserialize, Serialize};

const COUNTRY: &str = "us";
const LANG: &str = "en";

#[derive(Debug, Serialize, Deserialize)]
pub struct GNewsData {
    pub total_articles: u32,
    pub articles: Vec<Article>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub content: String,
    pub url: String,
    pub image: String,
    pub published_at: String,
    pub source: Source,
}

pub enum SearchArea {
    Title,
    Description,
    Content,
}

impl SearchArea {
    pub fn str(&self) -> &str {
        match &self {
            Self::Title => "title",
            Self::Description => "description",
            Self::Content => "content",
        }
    }
}

pub fn create_search_req(
    query: &str,
    search_area: &[SearchArea],
    news_count: Option<usize>,
    /* this only works on paid*/ page: Option<usize>,
) -> String {
    let news_count = news_count.or(Some(10)).unwrap();
    let page = page.or(Some(1)).unwrap();

    let search_area = if search_area.len() == 0 {
        &[SearchArea::Content]
    } else {
        search_area
    };
    let search_area = search_area
        .into_iter()
        .map(|x| x.str())
        .collect::<Vec<&str>>()
        .join(",");
    let api_key = std::env::var("API_KEY").expect("You must to enter API_KEY environment varaible");
    format!(
        "https://gnews.io/api/v4/search?q={query}&lang={LANG}&country={COUNTRY}&max={news_count}\
    &apikey={api_key}&page={page}&in={search_area}"
    )
}
