use rusqlite;

pub async fn store_json_to_db(
  events: Vec<serde_json::Value>,
  path_to_db: &str,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
  let conn = rusqlite::Connection::open(path_to_db)?;
  conn.execute(
    "CREATE TABLE IF NOT EXISTS events (
          id INTEGER PRIMARY KEY,
          sender TEXT NOT NULL,
          event TEXT NOT NULL,
          time_begin TEXT NOT NULL,
          time_end TEXT NOT NULL,
          position TEXT NOT NULL,
          \"abstract\" TEXT NOT NULL,
          speaker TEXT NOT NULL
      )",
    [],
  )?;

  let mut updated = 0;
  let mut inserted = 0;

  for event in events.clone() {
    let event_name = event["event"].as_str().unwrap_or_default();
    let sender = event["sender"].as_str().unwrap_or_default();
    let position = event["position"].as_str().unwrap_or_default();
    let time_begin = event["time_begin"].as_str().unwrap_or_default();
    let time_end = event["time_end"].as_str().unwrap_or_default();

    let mut stmt = conn.prepare(
      "SELECT id FROM events WHERE sender = ?1 AND position = ?2 AND time_begin = ?3 AND time_end = ?4",
    )?;
    let exists = stmt.exists(rusqlite::params![sender, position, time_begin, time_end])?;

    if exists {
      // Update existing record
      conn.execute(
      "UPDATE events SET time_begin = ?1, time_end = ?2, position = ?3, \"abstract\" = ?4, speaker = ?5 
       WHERE sender = ?6 AND event = ?7",
      rusqlite::params![
        event["time_begin"].as_str().unwrap_or_default(),
        event["time_end"].as_str().unwrap_or_default(),
        event["position"].as_str().unwrap_or_default(),
        event["abstract"].as_str().unwrap_or_default(),
        event["speaker"].as_str().unwrap_or_default(),
        sender,
        event_name,
      ],
    )?;
      updated += 1;
    } else {
      // Insert new record
      conn.execute(
        "INSERT INTO events (sender, event, time_begin, time_end, position, \"abstract\", speaker) 
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
          sender,
          event_name,
          event["time_begin"].as_str().unwrap_or_default(),
          event["time_end"].as_str().unwrap_or_default(),
          event["position"].as_str().unwrap_or_default(),
          event["abstract"].as_str().unwrap_or_default(),
          event["speaker"].as_str().unwrap_or_default(),
        ],
      )?;
      inserted += 1;
    }
  }

  Ok((inserted, updated))
}
