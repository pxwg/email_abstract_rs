#[cfg(test)]
mod tests {
  use ::email_abstract_rs::email::EmailTable;
  use ::email_abstract_rs::email_abstract;

  #[test]
  fn test_format_emails() {
    let emails = vec![
      EmailTable {
        sender: "sender1@example.com".to_string(),
        subject: "Subject 1".to_string(),
        body: "Body 1".to_string(),
      },
      EmailTable {
        sender: "sender2@example.com".to_string(),
        subject: "Subject \"2\"".to_string(),
        body: "Body\n2".to_string(),
      },
    ];

    let formatted = email_abstract::format_emails(&emails);
    assert!(formatted.starts_with("mails = {"));
    assert!(formatted.contains("sender: \"sender1@example.com\""));
    assert!(formatted.contains("subject: \"Subject 1\""));
    assert!(formatted.contains("body: \"Body 1\""));
    assert!(formatted.ends_with("}"));

    assert!(formatted.starts_with("mails = {"));
    assert!(formatted.contains("sender: \"sender2@example.com\""));
    assert!(formatted.contains("subject: \"Subject '2'\""));
    assert!(formatted.contains("body: \"Body 2\""));
    assert!(formatted.ends_with("}"));
  }

  #[test]
  fn test_generate_summary_prompt() {
    let emails = vec![EmailTable {
      sender: "sender1@example.com".to_string(),
      subject: "Subject 1".to_string(),
      body: "Body 1".to_string(),
    }];
    let config = email_abstract_rs::config::Config::get();
    let prompt = email_abstract::generate_summary_prompt(&emails);
    let result =
      "mails = {{sender: \"sender1@example.com\", subject: \"Subject 1\", body: \"Body 1\"}}";
    assert_eq!(prompt, config.prompt.replace("{emails_input}", result));
  }
}
