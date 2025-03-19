use email_abstract_rs::data_sql::store_json_to_db;
use rusqlite::Connection;
use serde_json::json;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_store_json_to_db_english() {
  let db_file = NamedTempFile::new().unwrap();
  let db_path = db_file.path().to_str().unwrap();

  let test_events = vec![json!({
    "sender": "test@example.com",
    "event": "Test Event",
    "time_begin": "2023-01-01T10:00:00",
    "time_end": "2023-01-01T11:00:00",
    "position": "Room 101",
    "abstract": "This is a test abstract with English content."
  })];

  let result = store_json_to_db(test_events.clone(), db_path).await;
  assert!(result.is_ok());

  // Verify database contents
  let conn = Connection::open(db_path).unwrap();
  let mut stmt = conn.prepare("SELECT * FROM events").unwrap();
  let event = stmt
    .query_row([], |row| {
      Ok((
        row.get::<_, String>(1)?, // sender
        row.get::<_, String>(2)?, // event
        row.get::<_, String>(3)?, // time_begin
        row.get::<_, String>(4)?, // time_end
        row.get::<_, String>(5)?, // position
        row.get::<_, String>(6)?, // abstract
      ))
    })
    .unwrap();

  assert_eq!(event.0, "test@example.com");
  assert_eq!(event.1, "Test Event");
  assert_eq!(event.2, "2023-01-01T10:00:00");
  assert_eq!(event.3, "2023-01-01T11:00:00");
  assert_eq!(event.4, "Room 101");
  assert_eq!(event.5, "This is a test abstract with English content.");
}

#[tokio::test]
async fn test_store_json_to_db_chinese() {
  let db_file = NamedTempFile::new().unwrap();
  let db_path = db_file.path().to_str().unwrap();

  let test_events = vec![json!({
    "sender": "测试@example.com",
    "event": "测试活动",
    "time_begin": "2023-01-01T10:00:00",
    "time_end": "2023-01-01T11:00:00",
    "position": "清华大学主楼",
    "abstract": "这是一个中文摘要，测试数据库对中文的支持。"
  })];

  let result = store_json_to_db(test_events.clone(), db_path).await;
  assert!(result.is_ok());

  // Verify database contents
  let conn = Connection::open(db_path).unwrap();
  let mut stmt = conn.prepare("SELECT * FROM events").unwrap();
  let event = stmt
    .query_row([], |row| {
      Ok((
        row.get::<_, String>(1)?, // sender
        row.get::<_, String>(2)?, // event
        row.get::<_, String>(3)?, // time_begin
        row.get::<_, String>(4)?, // time_end
        row.get::<_, String>(5)?, // position
        row.get::<_, String>(6)?, // abstract
      ))
    })
    .unwrap();

  assert_eq!(event.0, "测试@example.com");
  assert_eq!(event.1, "测试活动");
  assert_eq!(event.2, "2023-01-01T10:00:00");
  assert_eq!(event.3, "2023-01-01T11:00:00");
  assert_eq!(event.4, "清华大学主楼");
  assert_eq!(event.5, "这是一个中文摘要，测试数据库对中文的支持。");
}

#[tokio::test]
async fn test_store_json_to_db_multiple_events() {
  let db_file = NamedTempFile::new().unwrap();
  let db_path = db_file.path().to_str().unwrap();

  let test_events = vec![
    json!({
      "sender": "english@example.com",
      "event": "English Event",
      "time_begin": "2023-01-01T10:00:00",
      "time_end": "2023-01-01T11:00:00",
      "position": "Room 101",
      "abstract": "English abstract text."
    }),
    json!({
      "sender": "chinese@example.com",
      "event": "中文活动",
      "time_begin": "2023-01-02T14:00:00",
      "time_end": "2023-01-02T16:00:00",
      "position": "图书馆",
      "abstract": "中文摘要内容。"
    }),
    json!({
      "sender": "mixed@example.com",
      "event": "Mixed 混合 Event",
      "time_begin": "2023-01-03T09:00:00",
      "time_end": "2023-01-03T10:30:00",
      "position": "Conference Room 会议室",
      "abstract": "This is a mixed language abstract 这是一个混合语言的摘要。"
    }),
  ];

  let result = store_json_to_db(test_events.clone(), db_path).await;
  assert!(result.is_ok());

  // Verify database contents - count records
  let conn = Connection::open(db_path).unwrap();
  let count: i64 = conn
    .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
    .unwrap();
  assert_eq!(count, 3);

  // Verify specific events
  let mut stmt = conn
    .prepare("SELECT sender, event FROM events ORDER BY time_begin")
    .unwrap();
  let rows = stmt
    .query_map([], |row| {
      Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })
    .unwrap();

  let mut results = Vec::new();
  for row in rows {
    results.push(row.unwrap());
  }

  assert_eq!(results.len(), 3);
  assert_eq!(results[0].0, "english@example.com");
  assert_eq!(results[0].1, "English Event");
  assert_eq!(results[1].0, "chinese@example.com");
  assert_eq!(results[1].1, "中文活动");
  assert_eq!(results[2].0, "mixed@example.com");
  assert_eq!(results[2].1, "Mixed 混合 Event");
}
