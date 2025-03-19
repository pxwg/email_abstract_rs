use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub model: String,
  pub prompt: String,
  pub temperature: f32,
  pub max_tokens: i32,
  pub dates: u64,
}

const DEFAULT_PROMPT: &str = "input = {emails_input} \n 是一系列邮件的组合，这些邮件是一系列会议邀请的邮件，现在，请你结合这一系列邮件的内容，按照如下格式进行总结: 输出一个JSON表格，这个表格包含内容: {{sender(发件人), event(事件标题) , time_begin(活动开始时间 格式为 xxxx年xx月xx日 xx时xx分), time_end(活动结束时间 格式为 xxxx年xx月xx日 xx时xx分), position(活动发生的地点), abstract(简要概括,包含活动内容，主要参与人，主要内容。如果是学术报告，还需要概括学术报告内的科研结果简介)}}, 这样的内容请为每一个邮件组织一个，最终以一系列JSON表格的形式输出，并注意使用中文输出。注意，你仅被允许输出纯JSON格式的内容，包括markdown的代码块也不被允许输出。不允许输出任何额外的内容。请注意所总结时间的正确性！尤其是年份，要注意是不是2025，请仔细检查。如果邮件input为空，则输出为空";

impl Default for Config {
  fn default() -> Self {
    Self {
      model: "deepseek-chat".to_string(),
      prompt: DEFAULT_PROMPT.to_string(),
      temperature: 0.7,
      max_tokens: 1024,
      dates: 1,
    }
  }
}

impl Config {
  pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;

    // Parse TOML into a generic Value first
    let toml_value: toml::Value = toml::from_str(&contents)?;

    let mut config = Self::default();

    if let Some(model) = toml_value.get("model").and_then(|v| v.as_str()) {
      config.model = model.to_string();
    }
    if let Some(prompt) = toml_value.get("prompt").and_then(|v| v.as_str()) {
      config.prompt = prompt.to_string();
    }
    if let Some(temperature) = toml_value.get("temperature").and_then(|v| v.as_float()) {
      config.temperature = temperature as f32;
    }
    if let Some(max_tokens) = toml_value.get("max_tokens").and_then(|v| v.as_integer()) {
      config.max_tokens = max_tokens as i32;
    }
    if let Some(dates) = toml_value.get("dates").and_then(|v| v.as_integer()) {
      config.dates = dates as u64;
    }

    Ok(config)
  }

  /// Gets config from standard locations or creates default if not found
  pub fn get() -> Self {
    let possible_paths = vec![
      Some(PathBuf::from("./email_abstract.toml")),
      dirs::config_dir().map(|p| p.join("email_abstract/config.toml")),
      dirs::home_dir().map(|p| p.join(".config/email_abstract/config.toml")),
    ];

    for path in possible_paths.into_iter().flatten() {
      if let Ok(config) = Self::load_from_file(&path) {
        return config;
      }
    }

    return Self::default();
  }
}
