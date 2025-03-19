use crate::email::EmailTable;

/// Format a vector of EmailTable into a string representation
pub fn format_emails(emails: &Vec<EmailTable>) -> String {
  let mut formatted = String::from("mails = {");

  for (i, email) in emails.iter().enumerate() {
    if i > 0 {
      formatted.push_str(", ");
    }

    formatted.push_str(&format!(
      "{{sender: \"{}\", subject: \"{}\", body: \"{}\"}}",
      clean_string(&email.sender),
      clean_string(&email.subject),
      clean_string(&email.body)
    ));
  }

  formatted.push_str("}");
  formatted
}

/// Clean a string to make it suitable for inclusion in the prompt
fn clean_string(s: &str) -> String {
  s.replace("\"", "'").replace("\n", " ").replace("\r", " ")
}

/// Create a prompt for summarizing emails
pub fn generate_summary_prompt(emails: &Vec<EmailTable>) -> String {
  let formatted_emails = format_emails(emails);
  let config = crate::config::Config::get();

  return format!(
    "{}",
    config.prompt.replace("{emails_input}", &formatted_emails)
  );
}
