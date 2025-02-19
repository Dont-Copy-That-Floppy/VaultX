use mongodb::{Collection, Database};
use log::{info, error};
use crate::models::record::Record;

impl Record {
    /// Validates the Record data
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Validation Error: Title cannot be empty.".to_string());
        }
        if self.encrypted_data.trim().is_empty() {
            return Err("Validation Error: Encrypted data cannot be empty.".to_string());
        }
        Ok(())
    }
}

/// Retrieves the records collection from the database
/// No Result needed as `db.collection()` does not return an error
pub fn get_records_collection(db: &Database) -> Collection<Record> {
    info!("Fetching 'records' collection from the database.");
    db.collection::<Record>("records")
}

/// Example function to insert a record into the collection
/// Handles validation and database insertion errors
pub async fn insert_record(db: &Database, record: Record) -> Result<(), String> {
    let collection = get_records_collection(db);

    // Validate the record before insertion
    if let Err(e) = record.validate() {
        error!("Validation failed: {}", e);
        return Err(e);
    }

    // Attempt to insert the record into MongoDB
    match collection.insert_one(record, None).await {
        Ok(_) => {
            info!("Record inserted successfully.");
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to insert record: {}", e);
            error!("{}", error_msg);
            Err(error_msg)
        }
    }
}
