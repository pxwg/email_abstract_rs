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
          speaker_name TEXT NOT NULL,
          speaker_title TEXT NOT NULL
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
      "UPDATE events SET time_begin = ?1, time_end = ?2, position = ?3, \"abstract\" = ?4, speaker_name = ?5, speaker_title = ?6 
       WHERE sender = ?7 AND event = ?8",
      rusqlite::params![
        event["time_begin"].as_str().unwrap_or_default(),
        event["time_end"].as_str().unwrap_or_default(),
        event["position"].as_str().unwrap_or_default(),
        event["abstract"].as_str().unwrap_or_default(),
        event["speaker_name"].as_str().unwrap_or_default(),
        event["speaker_title"].as_str().unwrap_or_default(),
        sender,
        event_name,
      ],
    )?;
      updated += 1;
    } else {
      conn.execute(
        "INSERT INTO events (sender, event, time_begin, time_end, position, \"abstract\", speaker_name, speaker_title) 
       VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
          sender,
          event_name,
          event["time_begin"].as_str().unwrap_or_default(),
          event["time_end"].as_str().unwrap_or_default(),
          event["position"].as_str().unwrap_or_default(),
          event["abstract"].as_str().unwrap_or_default(),
          event["speaker_name"].as_str().unwrap_or_default(),
          event["speaker_title"].as_str().unwrap_or_default(),
        ],
      )?;
      inserted += 1;
    }
  }

  Ok((inserted, updated))
}

pub async fn search_events_by_time_begin(
  search_string: &str,
  path_to_db: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
  let conn = rusqlite::Connection::open(path_to_db)?;

  // Prepare the query with LIKE operator to search for substring
  let query = "SELECT * FROM events WHERE time_begin LIKE ?";
  let mut stmt = conn.prepare(query)?;

  // Execute the query with search pattern including wildcards
  let search_pattern = format!("%{}%", search_string);
  let rows = stmt.query_map([search_pattern], |row| {
    Ok(serde_json::json!({
        "id": row.get::<_, i64>("id")?,
        "sender": row.get::<_, String>("sender")?,
        "event": row.get::<_, String>("event")?,
        "time_begin": row.get::<_, String>("time_begin")?,
        "time_end": row.get::<_, String>("time_end")?,
        "position": row.get::<_, String>("position")?,
        "abstract": row.get::<_, String>("abstract")?,
        "speaker_name": row.get::<_, String>("speaker_name")?,
        "speaker_title": row.get::<_, String>("speaker_title")?
    }))
  })?;

  let mut events = Vec::new();
  for row in rows {
    events.push(row?);
  }

  Ok(events)
}
