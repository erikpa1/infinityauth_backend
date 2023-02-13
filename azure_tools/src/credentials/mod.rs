

use std::env;

pub fn get_storage_name() -> String {
    String::from(env::var("INF_AUTH_STORAGE_NAME").unwrap_or("".into()))
}

pub fn get_storage_key() -> String {
    String::from(env::var("INF_AUTH_STORAGE_KEY").unwrap_or("".into()))//
}

