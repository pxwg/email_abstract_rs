use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Generates HTML content for events and saves it to a file
pub async fn generate_events_html(
  events: &Vec<serde_json::Value>,
  template_path: &str,
  output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  // Read template file
  let template = fs::read_to_string(template_path)?;

  // Create event sections for each event
  let mut event_sections = String::new();

  for event in events {
    let event_title = event
      .get("event")
      .and_then(|v| v.as_str())
      .unwrap_or("未知事件");
    let speaker_name = event
      .get("speaker_name")
      .and_then(|v| v.as_str())
      .unwrap_or("");
    let speaker_title = event
      .get("speaker_title")
      .and_then(|v| v.as_str())
      .unwrap_or("");
    let time_begin = event
      .get("time_begin")
      .and_then(|v| v.as_str())
      .unwrap_or("");
    let position = event.get("position").and_then(|v| v.as_str()).unwrap_or("");

    // Create a section for this event
    let event_section = format!(
      r#"        <!-- Event -->
        <section class="content-section box-sizing-border">
          <p class="no-margin box-sizing-border">
            <span class="section-title box-sizing-border"><strong class="box-sizing-border"> {event} </strong></span>
          </p>
          <p class="no-margin box-sizing-border">
            报告人：{speaker_name} {speaker_title}
          </p>
          <p class="no-margin box-sizing-border">
            时间:<span class="highlight-text box-sizing-border"> {time_begin} </span>
          </p>
          <p class="no-margin box-sizing-border">
            地点: {position} </span>
          </p>
        </section>
        <!-- Divider -->
        <section class="divider box-sizing-border">
          <section class="dotted-line box-sizing-border">
            <svg viewbox="0 0 1 1" style="float:left;line-height:0;width:0;vertical-align:top;box-sizing:border-box;" xml:space="default"></svg>
          </section>
        </section>
        "#,
      event = event_title,
      speaker_name = speaker_name,
      speaker_title = speaker_title,
      time_begin = time_begin,
      position = position
    );

    event_sections.push_str(&event_section);
  }

  // // Remove the last divider if there are any events
  // if !events.is_empty() {
  //   event_sections = event_sections.trim_end().to_string();
  //   let last_divider_start = event_sections
  //     .rfind("<!-- Divider -->")
  //     .unwrap_or(event_sections.len());
  //   event_sections = event_sections[..last_divider_start].trim_end().to_string();
  // }

  // Insert the event sections into the template
  let final_html = template
        .replace("<!-- First Seminar -->", "<!-- Events -->")
        .replace(r#"        <section class="content-section box-sizing-border">
          <p class="no-margin box-sizing-border">
            <span class="section-title box-sizing-border"><strong class="box-sizing-border"> {event} </strong></span>
          </p>
          <p class="no-margin box-sizing-border">
            报告人：{speaker_name} {speaker_title}
          </p>
          <p class="no-margin box-sizing-border">
            时间:<span class="highlight-text box-sizing-border"> {time_begin} </span>
          </p>
          <p class="no-margin box-sizing-border">
            地点: {position} </span>
        </p>
      </section>
      <!-- Divider -->
      <section class="divider box-sizing-border">
        <section class="dotted-line box-sizing-border">
          <svg viewbox="0 0 1 1" style="float:left;line-height:0;width:0;vertical-align:top;box-sizing:border-box;" xml:space="default"></svg>
        </section>
      </section>
      <!-- End of the first seminar -->"#, &event_sections);

  // Create output directory if it doesn't exist
  if let Some(parent) = Path::new(output_path).parent() {
    fs::create_dir_all(parent)?;
  }

  // Write to output file
  let mut file = File::create(output_path)?;
  file.write_all(final_html.as_bytes())?;

  Ok(())
}
