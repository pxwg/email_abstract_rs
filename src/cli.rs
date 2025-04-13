use clap::{Parser, Subcommand};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Query emails, generate summaries, and store results
  Query {
    /// Number of days of emails to fetch
    #[arg(long)]
    date: Option<u64>,

    /// API key for authentication
    #[arg(long)]
    api_key: Option<String>,

    /// Email address to use for authentication
    #[arg(long)]
    mail_address: Option<String>,

    /// Email password for authentication
    #[arg(long)]
    mail_pwd: Option<String>,

    /// Path to the SQLite database
    #[arg(long)]
    db_path: Option<String>,

    /// Model name to use for API requests
    #[arg(long)]
    model: Option<String>,

    /// Maximum tokens for API response
    #[arg(long)]
    max_tokens: Option<i32>,

    /// Temperature setting for response randomness
    #[arg(long)]
    temperature: Option<f32>,

    /// Mail server address
    #[arg(long, default_value = "mails.tsinghua.edu.cn")]
    mail_server: String,
  },

  /// Search events by time_begin field
  Search {
    /// Search string for time_begin field (supports Chinese characters)
    #[arg(required = true)]
    query: String,

    /// Path to the SQLite database
    #[arg(long)]
    db_path: Option<String>,
  },

  /// Generate HTML for events
  Generate {
    /// Search string for time_begin field (date to search)
    #[arg(required = true)]
    date: String,

    /// Path to the HTML template file
    #[arg(long)]
    template: Option<String>,

    /// Path to the SQLite database
    #[arg(long)]
    db_path: Option<String>,

    /// Path to output HTML file (default: ./out/date.html)
    #[arg(long)]
    output: Option<String>,
  },
}

/// Create a styled progress bar
pub fn create_progress_bar(
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

/// Load environment variables or use provided CLI values
pub fn get_config_values(
  api_key: Option<String>,
  mail_address: Option<String>,
  mail_pwd: Option<String>,
  db_path: Option<String>,
) -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();

  let api_key =
    api_key.unwrap_or_else(|| dotenv::var("DEEPSEEK_API_KEY").expect("API key not found"));

  let email_address =
    mail_address.unwrap_or_else(|| dotenv::var("MAIL_ADDRESS").expect("Email address not found"));

  let email_password =
    mail_pwd.unwrap_or_else(|| dotenv::var("MAIL_PASSWORD").expect("Email password not found"));

  let path_to_db =
    db_path.unwrap_or_else(|| dotenv::var("PATH_TO_DB").expect("Path to DB not found"));

  Ok((api_key, email_address, email_password, path_to_db))
}
