#[macro_use]
// use bytes::Bytes;
use azure_core::error::{ErrorKind, ResultExt};


use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;


use std::error::Error;
use std::fmt::{Binary, Display};
use std::sync::{Mutex, MutexGuard};

use futures::stream::StreamExt;


use super::super::credentials;

struct StorageInstance {
    pub text: String,
}

impl StorageInstance {
    const fn new() -> StorageInstance {
        StorageInstance {
            text: String::new(),
        }
    }
}

static STORAGE: Mutex<StorageInstance> = Mutex::new(StorageInstance::new());


fn _get_storage_credentials() -> StorageCredentials {
    StorageCredentials::Key(credentials::get_storage_name(), credentials::get_storage_key())
}


pub fn get_project_prefix<T>(project_uid: &T) -> String where T: Display {
    return format!("project-{}", project_uid);
}


pub fn get_service_client() -> BlobServiceClient {
    BlobServiceClient::new(credentials::get_storage_name(),
                           _get_storage_credentials())
}

pub fn get_container_client(container: &String) -> ContainerClient {
    let service_client = get_service_client();
    let cont_client = service_client.container_client(container);
    return cont_client;
}

pub async fn get_blob(container: &String, blob_path: &String) -> Result<String, Box<dyn Error>> {
    let blob_client = get_container_client(container).blob_client(blob_path);

    let s_content = String::from_utf8(blob_client.get_content().await?)
        .map_kind(ErrorKind::DataConversion)?;

    Ok(s_content)
}


pub async fn get_project_blob(container: &String, blob_path: &String) -> Result<String, Box<dyn Error>> {
    let containerName = get_project_prefix(container);

    let blob_client = get_container_client(&containerName).blob_client(blob_path);

    let s_content = String::from_utf8(blob_client.get_content().await?)
        .map_kind(ErrorKind::DataConversion)?;

    Ok(s_content)
}


pub async fn get_project_binary_blob(container: &String, blob_path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let containerName = get_project_prefix(container);

    let blob_client = get_container_client(&containerName).blob_client(blob_path);

    let binResponse = blob_client.get_content().await?;

    Ok(binResponse)
}


pub async fn get_container_size(container: &String) -> Result<f64, Box<dyn Error>> {
    let cClient = get_container_client(container);

    let page = cClient.list_blobs().into_stream().next().await.unwrap().unwrap();

    let mut size: f64 = 0.0;

    for content in page.blobs.blobs() {
        size += (content.properties.content_length as f64) / 1000000.0
    }

    return Ok(size);
}


pub async fn create_container(containerName: &String) -> Result<bool, Box<dyn Error>> {
    let container = get_container_client(containerName);
    let response = container.create().await?;
    return Ok(true);
}


pub async fn upload_blob_string(container: &String, blob_path: &String, blob_type: &String, blob: &String) -> Result<bool, Box<dyn Error>> {
    let container = get_container_client(container);

    let blob_client = container.blob_client(blob_path);

    let response = blob_client
        .put_block_blob(blob.to_owned())
        .content_type(blob_type.to_owned()).await?;

    return Ok(true);
}

pub async fn upload_blob(container: &String, blob_path: &String, blob_type: &String, blob: &Vec<u8>) -> Result<bool, Box<dyn Error>> {
    let container = get_container_client(container);

    let blob_client = container.blob_client(blob_path);

    let response = blob_client
        .put_block_blob(blob.to_owned())
        .content_type(blob_type.to_owned()).await?;

    return Ok(true);
}


pub async fn delete_container(container: &String) -> Result<(), Box<dyn Error>> {
    let container = get_container_client(container);
    container.delete().await;
    Ok(())
}

