#[cfg(test)]
mod tests {
  use email_abstract_rs::email::{extract_email, is_tsinghua_sender, EmailTable};
  #[test]
  fn test_extract_email() {
    assert_eq!(
      extract_email("John Doe <john@example.com>"),
      "john@example.com"
    );
    assert_eq!(
      extract_email("no-brackets@example.com"),
      "no-brackets@example.com"
    );
    assert_eq!(
      extract_email("  spaced@example.com  "),
      "spaced@example.com"
    );
  }

  #[test]
  fn test_is_tsinghua_sender() {
    assert!(is_tsinghua_sender("someone@mail.tsinghua.edu.cn"));
    assert!(is_tsinghua_sender("someone@mails.tsinghua.edu.cn"));
    assert!(!is_tsinghua_sender("someone@example.com"));
  }

  #[test]
  fn test_email_table_creation() {
    let email = EmailTable {
      sender: "test@example.com".to_string(),
      subject: "Test Subject".to_string(),
      body: "Test Body".to_string(),
    };

    assert_eq!(email.sender, "test@example.com");
    assert_eq!(email.subject, "Test Subject");
    assert_eq!(email.body, "Test Body");
  }
}
