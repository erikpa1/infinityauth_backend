use azure_core::{Pageable, StatusCode};

use azure_data_tables::{prelude::*};
use azure_data_tables::operations::{InsertEntityResponse};

use azure_storage::prelude::*;

use futures::stream::StreamExt;

use std::error::Error;


use serde_json::{json, Value};

use super::super::credentials;


fn _get_storage_credentials() -> StorageCredentials {
    StorageCredentials::Key(credentials::get_storage_name(), credentials::get_storage_key())
}

pub fn get_service_client() -> TableServiceClient {
    TableServiceClient::new(credentials::get_storage_name(),
                            _get_storage_credentials())
}

pub fn get_table_client(table_name: &str) -> TableClient {
    return get_service_client().table_client(table_name);
}

pub async fn create_table(tableName: &str) -> Result<bool, Box<dyn Error>> {
    let table_client = get_table_client(tableName);
    table_client.create().await?;
    Ok(true)
}


pub async fn get_entities(tableName: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let table_client = get_service_client().table_client(tableName);

    let mut entityStream = table_client.query().into_stream::<Value>();

    let mut retValues: Vec<Value> = vec![];

    while let Some(response) = entityStream.next().await {
        let response = response?;

        for entity in response.entities {
            retValues.push(entity);
        }
    }

    Ok(retValues)
}

pub async fn get_entity(table_name: &str, partition_key: &str, row_key: &str) -> Result<Value, Box<dyn Error>> {
    let table_client = get_table_client(table_name);

    let entity_client = table_client.partition_key_client(partition_key).entity_client(row_key)?;

    let response = entity_client.get().await?;

    let mut entity: Value = response.entity;

    return Ok(entity);
}

// pub async fn get_entities_stream(tableName: &str) -> Result<(), Box<dyn Error>> {
//     let table_client = get_service_client().table_client(tableName);
//     let mut entityStream = table_client.query().into_stream::<Value>();
//
//     return Ok(entityStream);
// }

pub async fn insert_entity(table_name: &str, rk: &str, entity: &Value) -> Result<(), Box<dyn Error>> {
    let client = get_table_client(table_name);

    let pkClient = client.partition_key_client(rk);
    pkClient.transaction().insert(entity);
    Ok(())
}

pub async fn delete_entity(table_name: &str, pk: &str, rk: &str) -> Result<(), Box<dyn Error>> {
    let client = get_table_client(table_name);
    client.partition_key_client(pk).entity_client(rk)?.delete().await;
    Ok(())
}