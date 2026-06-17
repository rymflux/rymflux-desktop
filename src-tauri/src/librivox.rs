#![allow(dead_code, reason = "serde deserialization fields / step 1.4+ callers")]
use rymflux_core::types::{ContentItem, DomainId};
use serde::Deserialize;
use std::sync::LazyLock;

// ── API response structs ─────────────────────────────────────────────────────

#[derive(Deserialize)]
pub(crate) struct LibrivoxListResponse {
    books: Vec<LibrivoxBook>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct LibrivoxBook {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) url_text_source: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) copyright_year: Option<String>,
    pub(crate) num_sections: Option<String>,
    pub(crate) url_zip_file: Option<String>,
    pub(crate) url_librivox: Option<String>,
    pub(crate) totaltime: Option<String>,
    pub(crate) totaltimesecs: Option<i64>,
    pub(crate) authors: Vec<LibrivoxAuthor>,
    // Extended fields (when extended=1)
    pub(crate) url_iarchive: Option<String>,
    pub(crate) sections: Option<Vec<LibrivoxSection>>,
}

#[derive(Deserialize)]
pub(crate) struct LibrivoxAuthor {
    pub(crate) id: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct LibrivoxSection {
    pub(crate) id: String,
    pub(crate) section_number: String,
    pub(crate) title: String,
    pub(crate) listen_url: String,
    pub(crate) playtime: Option<String>,  // seconds as string
    pub(crate) language: Option<String>,
    pub(crate) readers: Vec<LibrivoxReader>,
}

#[derive(Deserialize)]
pub(crate) struct LibrivoxReader {
    pub(crate) reader_id: String,
    pub(crate) display_name: String,
}

// ── Shared HTTP client ───────────────────────────────────────────────────────

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

const BASE_URL: &str = "https://librivox.org/api/feed/audiobooks";

// ── Public API ───────────────────────────────────────────────────────────────

/// Search by title (uses `^` prefix for anchored search).
pub async fn search_by_title(query: &str, limit: u32, offset: u32) -> Result<Vec<LibrivoxBook>, String> {
    let url = format!("{BASE_URL}?title=%5E{query}&format=json&limit={limit}&offset={offset}");
    let resp = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("LibriVox request failed: {e}"))?;
    let data: LibrivoxListResponse = resp
        .json()
        .await
        .map_err(|e| format!("LibriVox parse failed: {e}"))?;
    Ok(data.books)
}

/// Search by author last name.
pub async fn search_by_author(author: &str, limit: u32, offset: u32) -> Result<Vec<LibrivoxBook>, String> {
    let url = format!("{BASE_URL}?author={author}&format=json&limit={limit}&offset={offset}");
    let resp = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("LibriVox request failed: {e}"))?;
    let data: LibrivoxListResponse = resp
        .json()
        .await
        .map_err(|e| format!("LibriVox parse failed: {e}"))?;
    Ok(data.books)
}

/// Get book details with all sections (chapters).
pub async fn get_book(id: &str) -> Result<LibrivoxBook, String> {
    let url = format!("{BASE_URL}/id/{id}?extended=1&format=json");
    let resp = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("LibriVox request failed: {e}"))?;
    // The extended endpoint returns `{ "books": { … } }` as a single object (not array)
    #[derive(Deserialize)]
    struct SingleBookResponse {
        books: LibrivoxBook,
    }
    let data: SingleBookResponse = resp
        .json()
        .await
        .map_err(|e| format!("LibriVox parse failed: {e}"))?;
    Ok(data.books)
}

/// Convert a LibrivoxBook to the domain's ContentItem format.
pub fn book_to_content_item(book: &LibrivoxBook) -> ContentItem {
    let author = book
        .authors
        .first()
        .map(|a| format!("{} {}", a.first_name, a.last_name))
        .unwrap_or_default();
    let metadata = serde_json::json!({
        "title": book.title,
        "author": author,
        "description": book.description,
        "language": book.language,
        "librivox_id": book.id,
        "total_time_secs": book.totaltimesecs,
        "num_sections": book.num_sections,
    });
    ContentItem {
        id: format!("librivox_{}", book.id),
        domain_id: DomainId::from("librivox"),
        source_uri: book.url_librivox.clone().unwrap_or_default(),
        metadata_json: Some(metadata),
        content_hash: String::new(),
        added_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
    }
}

/// Get the URL for streaming a chapter.
pub fn get_stream_url(section: &LibrivoxSection) -> String {
    section.listen_url.clone()
}
