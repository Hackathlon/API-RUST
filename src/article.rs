use actix_web::web::{Json, Path};
use actix_web::{HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, MONGODB_URL};
use crate::response::Response;
use crate::db::DB;
use mongodb::sync::Client;
use mongodb::bson::{Document};

pub type Articles = Response<Article>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: String,
    //pub created_at: DateTime<Utc>,
    pub content: String,
}

impl Article {
    pub fn new(content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            //created_at: Utc::now(),
            content,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleRequest {
    pub content: Option<String>,
}

impl ArticleRequest {
    pub fn to_article(&self) -> Option<Article> {
        match &self.content {
            Some(content) => Some(Article::new(content.to_string())),
            None => None,
        }
    }
}

/// list 10 last Articles `/article`
#[get("/article")]
pub async fn list() -> HttpResponse {
    let client = Client::with_uri_str(MONGODB_URL).unwrap();
    let db = client.database("ez-tax");
    let collection = db.collection::<Document>("articles");
    let cursor = collection.find(None, None).unwrap();
    let mut articles: Vec<Document> = vec![];
    for r in cursor {
        articles.push(r.unwrap());
    }

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(articles)
}

/// create a Article `/article`
#[post("/article")]
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
        Some(article) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(article),
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