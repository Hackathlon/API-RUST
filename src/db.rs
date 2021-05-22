use mongodb::options::ClientOptions;
use mongodb::sync::{Client, Collection};
use mongodb::bson::{doc, Document};
use mongodb::error::Error;

use crate::constants::{MONGODB_URL, MONGODB_DB, MONGODB_ARTICLE_COLL};
use crate::article::Article;
use futures::StreamExt;


// const ID: &str = "_id";
// const CONTENT: &str = "content";
// const CREATED_AT: &str = "created_at";

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub fn init() -> Self {
        DB {
            client: Client::with_uri_str(MONGODB_URL).unwrap()
        }
    }
    pub async fn fetch_articles(&self) -> Result<Vec<Article>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .map_err(|e| e)?;// TODO do something

        let mut result: Vec<Article> = Vec::new();
        for doc in cursor {
            result.push(self.doc_to_article(&doc?))
        }
        Ok(result)
    }

    fn get_collection(&self) -> Collection<Document> {
        self.client.database(MONGODB_DB).collection::<Document>(MONGODB_ARTICLE_COLL)
    }

    fn doc_to_article(&self, doc: &Document) -> Article {
        /*let id = doc.get_object_id(ID);
        let content = doc.get_str(CONTENT);
        let added_at = doc.get_datetime(CREATED_AT);

        let article = Article {
            id: id.to_hex(),
            created_at: DateTime::from(*added_at),
            content: content.to_owned()
        };
        Ok(article)*/
        Article {
            id: "".to_string(),
            //created_at: Utc::now(),
            content: "".to_string(),
        }
    }
}