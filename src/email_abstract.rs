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

    return format!("input = {} \n 是一系列邮件的组合，这些邮件是一系列会议邀请的邮件，现在，请你结合这一系列邮件的内容，按照如下格式进行总结: 输出一个JSON表格，这个表格包含内容: {{sender(发件人), event(事件标题) , time_begin(活动开始时间 格式为 xxxx年xx月xx日 xx时xx分), time_end(活动结束时间 格式为 xxxx年xx月xx日 xx时xx分), position(活动发生的地点), abstract(简要概括,包含活动内容，主要参与人(注意概括头衔)，学科内容。如果是学术报告，还需要概括学术报告内的科研结果简介)}}, 这样的内容请为每一个邮件组织一个，最终以一系列JSON表格的形式输出，并注意使用中文输出。注意，你仅被允许输出纯JSON格式的内容，包括markdown的代码块也不被允许输出。不允许输出任何额外的内容。请注意所总结时间的正确性！尤其是年份，要注意是不是2025，请仔细检查。如果邮件input为空，则输出空表", formatted_emails);
}
