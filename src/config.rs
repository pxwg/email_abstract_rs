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

const DEFAULT_PROMPT: &str = "input = {emails_input}

任务：分析一系列会议邀请邮件并提取关键信息。

输出格式：纯JSON数组，每个邮件对应一个JSON对象，不要包含markdown代码块或任何额外说明。

必须包含的字段：
- sender: 发件人邮箱
- event: 会议或活动标题
- time_begin: 活动开始时间（格式：YYYY年MM月DD日 HH时MM分）
- time_end: 活动结束时间（格式：YYYY年MM月DD日 HH时MM分）
- position: 活动地点
- speaker: 主讲人信息，包含姓名和头衔（例如：张三，北京大学教授）
- abstract: 活动内容概要，包括主要议题、参会嘉宾和重要信息。若为学术报告，需概括研究成果。注意，摘要需要尽可能详实丰富，包含所有关键信息。但不能直接照搬邮件内容，而应当利用介绍的表达方式进行概括。

重要提示：
1. 只输出纯JSON，不包含任何其他文本或格式标记
2. 所有输出必须使用中文,除非输入中包含英文或其他语言
3. 仔细核对日期信息，特别是年份的准确性
4. 若输入为空，则输出空数组[]

请确保提取的信息准确、完整，尤其注意日期和时间格式的正确性。";

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
