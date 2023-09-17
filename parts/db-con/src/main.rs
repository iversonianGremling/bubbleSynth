extern crate mongodb;
use std::borrow::BorrowMut;
use std::thread::AccessError;
use mongodb::{bson::doc, bson::Document, options::FindOptions, Client, error::Result as MongoResult};
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Set the MongoDB URI to connect to the local server
    let uri: &str = "mongodb://localhost:27017";

    // Create a new client and connect to the server
    let client: mongodb::Client = mongodb::Client::with_uri_str(uri).await?;

    // Access the "wiwiwi" database
    let database_name: &str = "wiwiwi";
    let database = client.database(database_name);

    // Access the "wiwiwi" collection
    let collection_name: &str = "wiwiwi";
    let collection: mongodb::Collection<mongodb::bson::Document> = database.collection(collection_name);

    // Find all documents in the collection
    let filter: mongodb::bson::Document = mongodb::bson::doc! {}; // An empty filter matches all documents
    let find_options: mongodb::options::FindOptions = mongodb::options::FindOptions::default(); // Default options, no modifications needed
                                                                                                //
    //Insert One-------------------------------
    let doc = doc! { "nombre": "John", "edad": 30 };
    collection.insert_one(doc, None).await.unwrap();

    //Update One--------------------------------
    //let filter = doc! {"name": "John"};
    let update = doc! {"$set": {"age": 35}};
    collection.update_one(filter, update, None).await.unwrap();

    //Look for documents-----------------------
    let mut cursor: mongodb::Cursor<mongodb::bson::Document> = collection.find(filter, find_options).await?;

    //Delete One----------------------------------
    //let filter = doc! {"name": "John"};
    collection.delete_one(filter, None).await.unwrap();


    // Iterate through the cursor and print documents
    while cursor.advance().await? {
        println!("{:?}", cursor.current().get_str("nombre").unwrap());
        let nombre = cursor.current().get_str("nombre").unwrap();
    }

    Ok(())
}


async fn print_index_names (database: mongodb::Database) {
  // Access the desired collection
    let collection: mongodb::Collection<mongodb::bson::Document> = database.collection("wiwiwi");

    // List index names in the collection
    let index_names = collection.list_index_names().await.unwrap();
    for index_name in index_names {
        println!("Index: {}", index_name);
    }

}
