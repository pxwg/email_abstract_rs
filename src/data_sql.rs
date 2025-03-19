use rusqlite;

pub async fn store_json_to_db(
    events: Vec<serde_json::Value>,
    path_to_db: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open(path_to_db)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY,
            sender TEXT NOT NULL,
            event TEXT NOT NULL,
            time_begin TEXT NOT NULL,
            time_end TEXT NOT NULL,
            position TEXT NOT NULL,
            abstract TEXT NOT NULL
        )",
        [],
    )?;

    for event in events.clone() {
        conn.execute(
            "INSERT INTO events (sender, event, time_begin, time_end, position, abstract) 
              VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                event["sender"].as_str().unwrap_or_default(),
                event["event"].as_str().unwrap_or_default(),
                event["time_begin"].as_str().unwrap_or_default(),
                event["time_end"].as_str().unwrap_or_default(),
                event["position"].as_str().unwrap_or_default(),
                event["abstract"].as_str().unwrap_or_default(),
            ],
        )?;
    }

    println!(
        "Successfully stored {} events in the database",
        events.len()
    );
    Ok(())
}
