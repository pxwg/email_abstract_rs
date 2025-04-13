use dotenv::dotenv;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

pub mod api_req;
pub mod config;
pub mod data_sql;
pub mod email;
pub mod email_abstract;

/// Load environment variables and return essential configuration
///
/// # Returns
/// A tuple containing (api_key, email_address, email_password, path_to_db)
fn load_environment() -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
  dotenv().ok();
  let api_key = dotenv::var("DEEPSEEK_API_KEY").expect("API key not found");
  let email_address = dotenv::var("MAIL_ADDRESS").expect("Email address not found");
  let email_password = dotenv::var("MAIL_PASSWORD").expect("Email password not found");
  let path_to_db = dotenv::var("PATH_TO_DB").expect("Path to DB not found");

  Ok((api_key, email_address, email_password, path_to_db))
}

/// Create a styled progress bar
///
/// # Arguments
/// * `m` - MultiProgress instance to add the progress bar to
/// * `message` - Initial message to display
/// * `spinner_style` - Character set for spinner animation
/// * `color` - Color for the spinner (e.g., "blue", "green")
///
/// # Returns
/// A configured ProgressBar instance
fn create_progress_bar(
  m: &MultiProgress,
  message: &str,
  spinner_style: &str,
  color: &str,
) -> ProgressBar {
  let pb = m.add(ProgressBar::new_spinner());
  pb.set_style(
    ProgressStyle::default_spinner()
      .tick_chars(spinner_style)
      .template(&format!("{{spinner:.{color}}} {{msg}}"))
      .unwrap(),
  );
  pb.set_message(message.to_string());
  pb.enable_steady_tick(Duration::from_millis(100));
  pb
}

/// Fetch emails with progress indication
///
/// # Arguments
/// * `m` - MultiProgress instance for coordinated display
/// * `email_address` - Email address to use for authentication
/// * `email_password` - Email password for authentication
/// * `days` - Number of days of emails to fetch
/// * `mail_server` - Mail server address
///
/// # Returns
/// Vector of fetched email entries
async fn fetch_emails_with_progress(
  m: &MultiProgress,
  email_address: &str,
  email_password: &str,
  days: u64,
  mail_server: &str,
) -> Vec<email::EmailTable> {
  let pb = create_progress_bar(m, "Fetching emails...", "⠁⠂⠄⡀⢀⠠⠐⠈ ", "blue");

  let emails = email::fetch_emails(email_address, email_password, days, mail_server).await;
  let num_emails = emails.len();

  pb.finish_with_message(format!("✓ {} emails fetched successfully!", num_emails));
  emails
}

/// Generate email summary with progress indication
///
/// # Arguments
/// * `m` - MultiProgress instance
/// * `emails` - Vector of emails to summarize
///
/// # Returns
/// Generated summary prompt
fn generate_summary_with_progress(m: &MultiProgress, emails: &Vec<email::EmailTable>) -> String {
  let pb = create_progress_bar(m, "Generating email summary...", "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏", "green");

  let prompt = email_abstract::generate_summary_prompt(emails);

  pb.finish_with_message("✓ Summary generated!");
  prompt
}

/// Query API with progress indication
///
/// # Arguments
/// * `m` - MultiProgress instance
/// * `prompt` - The summary prompt to send to the API
/// * `api_key` - API key for authentication
/// * `model` - The model name to use
/// * `max_tokens` - Maximum tokens for response
/// * `temperature` - Temperature setting for response randomness
///
/// # Returns
/// API response as string or error
async fn query_api_with_progress(
  m: &MultiProgress,
  prompt: &str,
  api_key: &str,
  model: &str,
  max_tokens: i32,
  temperature: f32,
) -> Result<String, Box<dyn std::error::Error>> {
  let pb = create_progress_bar(m, "Querying API...", "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏", "yellow");

  let result = api_req::query_openai(prompt, api_key, model, max_tokens, temperature).await?;

  pb.finish_with_message("✓ API response received!");
  Ok(result)
}

/// Store data in database with progress indication
///
/// # Arguments
/// * `m` - MultiProgress instance
/// * `result` - JSON string from API to parse
/// * `path_to_db` - Path to the SQLite database
///
/// # Returns
/// Result indicating success or error
async fn store_data_with_progress(
  m: &MultiProgress,
  result: &str,
  path_to_db: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let pb = create_progress_bar(
    m,
    "Processing and storing results...",
    "⣾⣽⣻⢿⡿⣟⣯⣷",
    "magenta",
  );

  let events: Vec<serde_json::Value> = serde_json::from_str(result)?;

  match data_sql::store_json_to_db(events, path_to_db).await {
    Ok((inserted, updated)) => {
      pb.finish_with_message(format!(
        "✓ {inserted} rows inserted, {updated} rows updated in database!"
      ));
      Ok(())
    }
    Err(e) => {
      pb.finish_with_message(format!("✗ Error: {}", e));
      Err(e)
    }
  }
}

/// Main application function
///
/// Orchestrates the entire email processing workflow:
/// 1. Loading configuration and environment
/// 2. Fetching emails
/// 3. Generating summaries
/// 4. Querying AI API
/// 5. Storing results in database
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Load configuration
  let (api_key, email_address, email_password, path_to_db) = load_environment()?;
  let config = config::Config::get();

  // Set up progress display
  let m = MultiProgress::new();

  // Process emails
  let emails = fetch_emails_with_progress(
    &m,
    &email_address,
    &email_password,
    config.dates,
    "mails.tsinghua.edu.cn",
  )
  .await;

  // Generate summary
  let prompt = generate_summary_with_progress(&m, &emails);

  // Query API
  let api_result = query_api_with_progress(
    &m,
    &prompt,
    &api_key,
    &config.model,
    config.max_tokens,
    config.temperature,
  )
  .await?;
  println!("\n✓ API Result: {}", api_result);

  // Store data
  store_data_with_progress(&m, &api_result, &path_to_db).await?;

  println!("\n✅ Process completed successfully!");
  Ok(())
}
