//// TODO: I dont know how to test this yet

// use email_abstract_rs::api_req::query_openai;
// use email_abstract_rs::config::Config;
// use mockito::{mock, server_url};
//
// // #[tokio::test]
// async fn test_query_openai_success() {
//   let mock_server = mock("POST", "/v1/chat/completions")
//     .with_status(200)
//     .with_header("content-type", "application/json")
//     .with_body(r#"{"choices":[{"message":{"content":"Test response"}}]}"#)
//     .create();
//
//   let result = query_openai(
//     "Test prompt",
//     &Config {
//       api_base_url: Some(server_url()),
//       ..Config::default()
//     },
//   )
//   .await;
//
//   assert!(result.is_ok());
//   assert_eq!(result.unwrap(), "Test response");
//   mock_server.assert();
// }
//
// // #[tokio::test]
// async fn test_error_handling() {
//   let mock_server = mock("POST", "/v1/chat/completions")
//     .with_status(401)
//     .with_body("Unauthorized")
//     .create();
//
//   let result = query_openai(
//     "Test prompt",
//     &Config {
//       api_base_url: Some(server_url()),
//       ..Config::default()
//     },
//   )
//   .await;
//
//   assert!(result.is_err());
//   mock_server.assert();
// }
