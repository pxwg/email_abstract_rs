use email_abstract_rs::config::Config;
use std::fs;

#[test]
fn test_load_config_from_file() {
  // Create test config content
  let test_config = r#"
model = "test-model"
prompt = "test prompt {emails_input}"
temperature = 0.5
max_tokens = 114527
dates = 11134
"#;

  // Write test config to file
  fs::write("test_config.toml", test_config).expect("Failed to write test config file");

  // Load config from file
  let config = Config::load_from_file("test_config.toml").expect("Failed to load config");

  // Verify config values
  assert_eq!(config.model, "test-model");
  assert_eq!(config.prompt, "test prompt {emails_input}");
  assert_eq!(config.temperature, 0.5);
  assert_eq!(config.max_tokens, 114527);
  assert_eq!(config.dates, 11134);

  // Clean up
  fs::remove_file("test_config.toml").expect("Failed to remove test file");
}
