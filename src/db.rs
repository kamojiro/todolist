use mongodb::bson::{doc};
use mongodb::options::{FindOptions,ClientOptions};
use mongodb::{Client, Collection};
use futures::stream::StreamExt;

use crate::errors::CustomError::NotFound;
use crate::errors::CustomError;
use crate::model::TodoEntry;

const DB_NAME: &str = "todo_list";
const COLLECTION_NAME: &str = "todos";

#[derive(Clone, Debug)]
pub struct MongoDbClient {
    client: Client,
}

impl MongoDbClient{
    pub async fn new(mongodb_uri: String) -> Self {
        let mut client_options = ClientOptions::parse(mongodb_uri).await.unwrap();
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options).expect("failed to create client");
        for db_name in client.list_database_names(None, None).await.expect("failed to get database list") {
            println!("{}", db_name);
        }
        MongoDbClient{
            client: client,
        }
    }

    pub async fn get_todos(&self) -> Result<Vec<TodoEntry>>{
        unimplemented!()
    }
    
    pub async fn get_all_todos(&self) -> Result<Vec<TodoEntry>>{
        let mut result = Vec::new();
        let collection = self.get_todos_collection().await;
        let filter = doc!{};
        let find_options = FindOptions::builder().sort(doc! {"name": 1}).build();
        println!("get all todo");
        let mut todos = collection.find(filter, find_options).await?;
        while let Some(todo) = todos.next().await {
            result.push(todo?)
        }
        println!("{:?}", result);
        Ok(result)
    }

    pub async fn create_todo(&self, todo: TodoEntry) -> Result<TodoEntry>{
        let collection = self.get_todos_collection().await;
        let insert_result = collection.insert_one(todo, None).await?;
        let filter = doc!{"_id": &insert_result.inserted_id};
        collection.find_one(filter, None).await?.ok_or(NotFound {
            message: String::from("Can't find a created todo")
        })
    }

    pub async fn get_todos_collection(&self) -> Collection<TodoEntry>{
        self.client
            .database(DB_NAME)
            .collection::<TodoEntry>(COLLECTION_NAME)
    }

}



#[allow(dead_code)]
type Result<T> = std::result::Result<T,CustomError>;
