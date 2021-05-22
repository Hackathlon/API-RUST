use mongodb::bson::{doc, Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

use crate::constants::{MONGODB_URL, MONGODB_DB, MONGODB_ARTICLE_COLL};
use crate::article::Article;
use mongodb::error::Error;
use warp::{Rejection, Reply};
use std::convert::Infallible;
use warp::http::StatusCode;
use futures::StreamExt;
use chrono::{DateTime, Utc};


const ID: &str = "_id";
const CONTENT: &str = "content";
const CREATED_AT: &str = "created_at";

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse(MONGODB_URL).await?;
        client_options.app_name = Some(MONGODB_DB.to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }
    pub async fn fetch_books(&self) -> Result<Vec<Article>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(|e| e)?;// TODO do something

        let mut result: Vec<Article> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_article(&doc?)?);
        }
        Ok(result)
    }

    fn get_collection(&self) -> Collection {
        self.client.database(MONGODB_DB).collection(MONGODB_ARTICLE_COLL)
    }

    fn doc_to_article(&self, doc: &Document) -> Result<Article> {
        let id = doc.get_object_id(ID);
        let content = doc.get_str(CONTENT);
        let added_at = doc.get_datetime(CREATED_AT);

        /*let article = Article {
            id: id.to_hex(),
            created_at: DateTime::from(*added_at),
            content: content.to_owned()
        };
        Ok(article)*/
        Ok(Article {
            id: "".to_string(),
            created_at: Utc::now(),
            content: "".to_string()
        })
    }

}