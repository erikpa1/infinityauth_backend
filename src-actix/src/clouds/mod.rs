mod azure;

pub async fn initialize_cloud() {
    let provider = "azure";

    if provider == "azure" {
        azure::init_azure().await

    }
}