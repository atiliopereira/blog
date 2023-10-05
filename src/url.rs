struct DocumentUrl<'a> {
    pub url: &'a str,
    pub document_path: &'a str,
}

const URLS: [DocumentUrl; 2] = [
    DocumentUrl { url: "home/", document_path: "home.html" },
    DocumentUrl { url: "/blog", document_path: "blog.html" },
];

const NOT_FOUND_DOCUMENT_PATH: &str = "404.html";

#[derive(Debug)]
pub enum StatusCode {
    Ok,
    NotFound,
}

pub fn handle_url(url: &str) -> Option<(&str, StatusCode)> {
    for document_url in URLS {
        if document_url.url == url {
            return Some((document_url.document_path, StatusCode::Ok));
        }
    }
    return Some((NOT_FOUND_DOCUMENT_PATH, StatusCode::NotFound))
}