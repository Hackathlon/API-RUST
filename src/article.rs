use actix_web::web::{Json, Path};
use actix_web::{HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON};
use crate::response::Response;

pub type Articles = Response<Article>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

impl Article {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleRequest {
    pub message: Option<String>,
}

impl ArticleRequest {
    pub fn to_article(&self) -> Option<Article> {
        match &self.message {
            Some(message) => Some(Article::new(message.to_string())),
            None => None,
        }
    }
}

/// list 10 last Articles `/article`
#[get("/article")]
pub async fn list() -> HttpResponse {
    // TODO find the last 10 Articles and return them

    let articles = Articles { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(articles)
}

/// create a Article `/article`
#[post("/Articles")]
pub async fn create(article_req: Json<ArticleRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(article_req.to_article())
}

/// find a Article by its id `/article/{id}`
#[get("/article/{id}")]
pub async fn get(path: Path<(String, )>) -> HttpResponse {
    // TODO find Article a Article by ID and return it
    let found_article: Option<Article> = None;

    match found_article {
        Some(Article) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(Article),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a Article by its id `/article/{id}`
#[delete("/article/{id}")]
pub async fn delete(path: Path<(String, )>) -> HttpResponse {
    // TODO delete Article by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}