use azure_tools::myazure::tables;

pub async fn init_azure() {
    tables::create_table("users").await;
    tables::create_table("applications").await;
    tables::create_table("organizations").await;
    tables::create_table("licences").await;
}
