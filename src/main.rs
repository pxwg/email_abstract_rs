use std::env;

pub mod api_req;
pub mod data_sql;
pub mod email;
pub mod email_abstract;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("DEEPSEEK_API_KEY").expect("API key not found");
    let email_address = env::var("MAIL_ADDRESS").expect("Email address not found");
    let email_password = env::var("MAIL_PASSWORD").expect("Email password not found");
    let path_to_db = env::var("PATH_TO_DB").expect("Path to DB not found");

    let emails_result =
        email::fetch_emails(&email_address, &email_password, 7, "mails.tsinghua.edu.cn").await;
    let prompt = email_abstract::generate_summary_prompt(&emails_result);

    let result = api_req::query_openai(&prompt, &api_key, "deepseek-chat", 1000, 0.7).await?;

    let events: Vec<serde_json::Value> = serde_json::from_str(&result)?;

    data_sql::store_json_to_db(events, &path_to_db).await?;

    Ok(())
}
