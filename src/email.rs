use chrono::{Duration, Local};
use imap;
use mailparse::parse_mail;
use mailparse::MailHeaderMap;
use native_tls::TlsConnector;

#[derive(Debug, Clone)]
pub struct EmailTable {
    pub sender: String,
    pub subject: String,
    pub body: String,
}

pub async fn fetch_emails(
    email_address: &str,
    password: &str,
    days_ago: u64,
    imap_server: &str,
) -> Vec<EmailTable> {
    match inner_fetch_emails(email_address, password, days_ago, imap_server) {
        Ok(emails) => emails,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    }
}

fn inner_fetch_emails(
    email_address: &str,
    password: &str,
    days_ago: u64,
    imap_server: &str,
) -> Result<Vec<EmailTable>, Box<dyn std::error::Error>> {
    let mut email_tables = Vec::new();

    let tls = TlsConnector::builder().build()?;
    let client = imap::connect((imap_server, 993), imap_server, &tls).unwrap();

    let mut imap_session = client
        .login(email_address, password)
        .map_err(|(err, _client)| err)?;

    imap_session.select("INBOX")?;

    let since_date = (Local::now() - Duration::days(days_ago as i64))
        .format("%d-%b-%Y")
        .to_string();
    let search_criteria = format!("SINCE \"{}\"", since_date);

    let messages = imap_session.search(search_criteria)?;

    for num in messages.iter() {
        if let Ok(msg) = imap_session.fetch(num.to_string(), "RFC822") {
            if let Some(msg_body) = msg.iter().next().and_then(|m| m.body()) {
                if let Ok(parsed) = parse_mail(msg_body) {
                    process_email(&parsed, &mut email_tables);
                }
            }
        }
    }

    Ok(email_tables)
}

fn process_email(parsed: &mailparse::ParsedMail, results: &mut Vec<EmailTable>) {
    let sender = parsed.headers.get_first_value("From").unwrap_or_default();
    let sender = extract_email(&sender);
    if !is_tsinghua_sender(&sender) {
        return;
    }

    let subject = parsed
        .headers
        .get_first_value("Subject")
        .unwrap_or_default();

    let body = extract_body(parsed);

    results.push(EmailTable {
        sender,
        subject,
        body,
    });
}

fn extract_email(s: &str) -> String {
    s.split('<')
        .last()
        .and_then(|s| s.split('>').next())
        .unwrap_or(s)
        .trim()
        .to_lowercase()
}

fn is_tsinghua_sender(sender: &str) -> bool {
    sender.contains("mail.tsinghua") || sender.contains("mails.tsinghua")
}

fn extract_body(parsed: &mailparse::ParsedMail) -> String {
    let mut body = String::new();

    fn walk_part(part: &mailparse::ParsedMail) -> Option<String> {
        if part.ctype.mimetype.starts_with("text/") {
            return part.get_body().ok();
        }

        for subpart in &part.subparts {
            if let Some(body) = walk_part(subpart) {
                return Some(body);
            }
        }
        None
    }

    if let Some(content) = walk_part(parsed) {
        body = content;
    }

    body
}
