use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

use crate::constants::{MONGODB_URL, MONGODB_DB};

const COLL: &str = "articles";

const ID: &str = "_id";
const CONTENT: &str = "content";
const CREATED_AT: &str = "created_at";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let mut client_options = ClientOptions::parse(MONGODB_URL).await?;
        client_options.app_name = Some(MONGODB_DB.to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }
}