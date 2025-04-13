use clap::Parser;
use data_sql::search_events_by_time_begin;
use dotenv::dotenv;
use indicatif::MultiProgress;

pub mod api_req;
pub mod cli;
pub mod config;
pub mod data_sql;
pub mod email;
pub mod email_abstract;
pub mod insert_html;

/// Fetch emails with progress indication
async fn fetch_emails_with_progress(
  m: &MultiProgress,
  email_address: &str,
  email_password: &str,
  days: u64,
  mail_server: &str,
) -> Vec<email::EmailTable> {
  let pb = cli::create_progress_bar(m, "Fetching emails...", "⠁⠂⠄⡀⢀⠠⠐⠈ ", "blue");

  let emails = email::fetch_emails(email_address, email_password, days, mail_server).await;
  let num_emails = emails.len();

  pb.finish_with_message(format!("✓ {} emails fetched successfully!", num_emails));
  emails
}

/// Generate email summary with progress indication
fn generate_summary_with_progress(m: &MultiProgress, emails: &Vec<email::EmailTable>) -> String {
  let pb = cli::create_progress_bar(m, "Generating email summary...", "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏", "green");

  let prompt = email_abstract::generate_summary_prompt(emails);

  pb.finish_with_message("✓ Summary generated!");
  prompt
}

/// Query API with progress indication
async fn query_api_with_progress(
  m: &MultiProgress,
  prompt: &str,
  api_key: &str,
  model: &str,
  max_tokens: i32,
  temperature: f32,
) -> Result<String, Box<dyn std::error::Error>> {
  let pb = cli::create_progress_bar(m, "Querying API...", "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏", "yellow");

  let result = api_req::query_openai(prompt, api_key, model, max_tokens, temperature).await?;

  pb.finish_with_message("✓ API response received!");
  Ok(result)
}

/// Store data in database with progress indication
async fn store_data_with_progress(
  m: &MultiProgress,
  result: &str,
  path_to_db: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let pb = cli::create_progress_bar(
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

/// Process emails and generate summary
async fn process_query(
  api_key: &str,
  email_address: &str,
  email_password: &str,
  path_to_db: &str,
  days: u64,
  model: &str,
  max_tokens: i32,
  temperature: f32,
  mail_server: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  // Set up progress display
  let m = MultiProgress::new();

  // Process emails
  let emails =
    fetch_emails_with_progress(&m, email_address, email_password, days, mail_server).await;

  // Generate summary
  let prompt = generate_summary_with_progress(&m, &emails);

  // Query API
  let api_result =
    query_api_with_progress(&m, &prompt, api_key, model, max_tokens, temperature).await?;
  println!("\n✓ API Result: {}", api_result);

  // Store data
  store_data_with_progress(&m, &api_result, path_to_db).await?;

  println!("\n✅ Process completed successfully!");
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let app = cli::Cli::parse();

  match app.command {
    cli::Commands::Query {
      date,
      api_key,
      mail_address,
      mail_pwd,
      db_path,
      model,
      max_tokens,
      temperature,
      mail_server,
    } => {
      // Get configuration values from CLI args or env vars
      let (api_key, email_address, email_password, path_to_db) =
        cli::get_config_values(api_key, mail_address, mail_pwd, db_path)?;

      // Get config and override with CLI args if provided
      let config = config::Config::get();
      let days = date.unwrap_or(config.dates);
      let model = model.unwrap_or_else(|| config.model.clone());
      let max_tokens = max_tokens.unwrap_or(config.max_tokens);
      let temperature = temperature.unwrap_or(config.temperature);

      process_query(
        &api_key,
        &email_address,
        &email_password,
        &path_to_db,
        days,
        &model,
        max_tokens,
        temperature,
        &mail_server,
      )
      .await?;
    }
    cli::Commands::Search { query, db_path } => {
      let path_to_db =
        db_path.unwrap_or_else(|| dotenv::var("PATH_TO_DB").expect("Path to DB not found"));

      let events = search_events_by_time_begin(&query, &path_to_db).await?;
      println!("Found {} events containing '{}':", events.len(), query);
      for event in events {
        println!("{}", serde_json::to_string_pretty(&event)?);
      }
    }
    cli::Commands::Generate {
      date,
      db_path,
      template,
      output,
    } => {
      let path_to_db =
        db_path.unwrap_or_else(|| dotenv::var("PATH_TO_DB").expect("Path to DB not found"));

      let output_path = output.unwrap_or_else(|| format!("./out/{}.html", date));

      println!("Searching for events on date: {}", date);
      let events = data_sql::search_events_by_time_begin(&date, &path_to_db).await?;
      println!("Found {} events", events.len());

      if events.is_empty() {
        println!("No events found for the specified date");
        return Ok(());
      }

      let template_path = template.unwrap_or_else(|| {
        dotenv::var("TEMPLATE_PATH").unwrap_or_else(|_| "./template/wanyou_mini.html".to_string())
      });
      println!("Generating HTML output to {}", output_path);
      insert_html::generate_events_html(&events, &template_path, &output_path).await?;
      println!("HTML generation completed successfully");
    }
  }

  Ok(())
}
