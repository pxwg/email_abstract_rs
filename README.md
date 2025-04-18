# Email 数据存档工具

基于 DeepSeek 等 API 的邮件数据存档工具，包含简单的单元测试。

## 说明

### 基本工作流程

```mermaid
graph TD
    A[emails] --> B[DeepSeek API]
    B --> C[SQLite Database]
```
本工具主要用于抓取清华大学官方发送的邮件数据，抓取、生成对应报告的摘要并保存在数据库之中。目前直接硬编码过滤包含 `tsinghua.edu` 的发件人，之后会添加配置项。

### 配置

配置文件位于 `~/.config/email_abstract/config.toml` (见`/src/config.rs`)，格式如下：

```toml
model = "deepseek-chat" # or "deepseek-rensonor"
prompt = "input = {emails_input} \n 请按照某要求处理输入的邮件数据" # {emails_input} 处会插入格式化的邮件输入，默认配置可以参考 /src/config.rs
temperature = 0.5
max_tokens = 100
dates = 100 # 查询日期范围
# TODO: 添加过滤发件人的配置项
```
API 与 邮件地址、邮箱密码通过环境变量
```bash
DEEPSEEK_API_KEY="your_api_key"
MAIL_ADDRESS="zhangsan@mails.tsinghua.edu.cn"  # Set this elsewhere in your application
MAIL_PASSWORD="your_passowrd"  # Set this elsewhere in your application
PATH_TO_DB="/path/to/your/sqlite.db" # absolute path to the database
```
存储在 `.env` 文件之中。

### 使用

见 (懒得写了，回头用 ci 自动生成使用方法)
```bash
cargo run --release --bin email_abstract_rs -- -h
```
