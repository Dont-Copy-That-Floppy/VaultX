use mongodb::{Client, Database};

pub async fn get_database(uri: &str) -> Database {
    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");
    client.database("valutx")
}
